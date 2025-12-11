use crate::{camera::{look_and_orbit, activate_camera_on_input, CameraNeedsActivation, CAMERA_OFFSET_FROM_CAR}, input::{get_car_movement, CarAction}, movement::apply_movement, state::GameState};
use bevy::prelude::*;
use bevy_rapier3d::prelude::{Velocity, CollidingEntities};
use leafwing_input_manager::prelude::ActionState;
use smooth_bevy_cameras::LookTransform;

pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                activate_camera_on_input,
                get_car_movement,
                apply_movement.after(get_car_movement),
                look_and_orbit.after(apply_movement),
                // Run game over checks after movement to ensure car state is updated
                check_stuck.after(apply_movement),
                check_game_over.after(apply_movement),
            )
                .run_if(in_state(GameState::Running)),
        )
        // Allow camera controls during game over
        .add_systems(
            Update,
            (
                look_and_orbit,
            )
                .run_if(in_state(GameState::GameOver)),
        )
        .add_systems(OnEnter(GameState::GameOver), (spawn_game_over_screen, stop_car_momentum))
        .add_systems(Update, (
            handle_play_again_button,
            handle_gamepad_play_again,
            freeze_car_on_ground,
        ).run_if(in_state(GameState::GameOver)))
        .add_systems(OnExit(GameState::GameOver), (cleanup_game_over_screen, reset_car_on_exit_game_over));
    }
}

#[derive(Component)]
struct GameOverScreen;

#[derive(Component)]
struct PlayAgainButton;

#[derive(Component)]
struct ControlsText;

#[derive(Component, Default)]
pub struct StuckTimer {
    stuck_duration: f32,
    reset_grace_period: f32, // Grace period after reset to prevent immediate game over
}

pub fn spawn_controls_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let controls_text = "
drive
----------
↑ ↓ ← →
space (boost)

look
----------
w a s d";

    let font_handle = asset_server.load("font/NotoSansMono-Bold.ttf");

    commands
        .spawn(Node {
            width: Val::Px(200.),
            height: Val::Px(10.),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            left: Val::Px(10.),
            top: Val::Px(10.),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text(controls_text.to_string()),
                TextFont {
                    font: font_handle,
                    font_size: 16.,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ControlsText,
            ));
        });
}

fn check_stuck(
    mut car_query: Query<(&Transform, &mut StuckTimer, &CollidingEntities), With<Car>>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Ok((transform, mut stuck_timer, colliding_entities)) = car_query.single_mut() {
        // Decrease grace period after reset
        if stuck_timer.reset_grace_period > 0.0 {
            stuck_timer.reset_grace_period = (stuck_timer.reset_grace_period - time.delta_secs()).max(0.0);
            // Don't check for game over during grace period
            return;
        }
        
        // Check if car is on its side or upside down
        // Get the up vector of the car (Y axis in local space)
        let car_up = transform.rotation * Vec3::Y;
        let world_up = Vec3::Y;
        
        // Use dot product to check orientation
        // If dot product < 0.5, car is on its side or upside down
        // (0.5 corresponds to ~60 degrees, negative means upside down)
        let dot_product = car_up.dot(world_up);
        let is_upside_down_or_on_side = dot_product < 0.5;
        
        // Check if car is touching the ground (any part of the car is colliding)
        let is_touching_ground = !colliding_entities.is_empty();
        
        // If car is in bad orientation AND touching ground, increment timer
        if is_upside_down_or_on_side && is_touching_ground {
            stuck_timer.stuck_duration += time.delta_secs();
            
            // If in bad orientation for more than 1/4 second, trigger game over
            if stuck_timer.stuck_duration >= 0.25 {
                next_state.set(GameState::GameOver);
            }
        } else {
            // Reset timer if conditions aren't met (car is upright or not touching ground)
            stuck_timer.stuck_duration = 0.0;
        }
    }
}

fn check_game_over(
    car_query: Query<&Transform, With<Car>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Ok(car_transform) = car_query.single() {
        // Check if car has fallen off the map (Y position too low)
        if car_transform.translation.y < -200.0 {
            next_state.set(GameState::GameOver);
        }
    }
}

fn stop_car_momentum(
    mut car_query: Query<&mut Velocity, With<Car>>,
) {
    // Zero horizontal velocity and angular velocity, but ensure car falls
    if let Ok(mut velocity) = car_query.single_mut() {
        velocity.linvel.x = 0.0;
        velocity.linvel.z = 0.0;
        velocity.angvel = Vec3::ZERO; // Stop spinning
        // Ensure car falls - if Y velocity is positive (going up) or zero, give it downward velocity
        if velocity.linvel.y >= 0.0 {
            velocity.linvel.y = -5.0; // Small downward push to ensure it falls
        }
        // If already falling, keep the falling velocity
    }
}

fn freeze_car_on_ground(
    mut car_query: Query<(&mut Velocity, &CollidingEntities), With<Car>>,
) {
    // Once car hits the ground (is colliding) and has low velocity, fully freeze it
    if let Ok((mut velocity, colliding_entities)) = car_query.single_mut() {
        if !colliding_entities.is_empty() {
            // Car is on ground - only freeze if velocity is very low (essentially stopped)
            if velocity.linvel.length() < 1.0 {
                velocity.linvel = Vec3::ZERO;
                velocity.angvel = Vec3::ZERO;
            } else {
                // Still moving, dampen it gradually
                velocity.linvel *= 0.9;
                velocity.angvel *= 0.9;
            }
        }
    }
}

fn reset_car_on_exit_game_over(
    mut car_query: Query<(&mut Transform, &mut Velocity, &mut StuckTimer), With<Car>>,
) {
    // Ensure car is reset when exiting game over state (as a backup to button handler)
    // This runs when transitioning from GameOver to Running, ensuring car is always reset
    if let Ok((mut transform, mut velocity, mut stuck_timer)) = car_query.single_mut() {
        transform.translation = CAR_START_POSITION;
        transform.rotation = Quat::IDENTITY;
        velocity.linvel = Vec3::ZERO;
        velocity.angvel = Vec3::ZERO;
        stuck_timer.stuck_duration = 0.0;
        stuck_timer.reset_grace_period = 0.5;
    }
}

fn spawn_game_over_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handle = asset_server.load("font/NotoSansMono-Bold.ttf");
    
    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            GameOverScreen,
        ))
        .with_children(|parent| {
            // "GAME OVER" text
            parent.spawn((
                Text("GAME OVER".to_string()),
                TextFont {
                    font: font_handle.clone(),
                    font_size: 64.,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
            
            // "PLAY AGAIN?" button
            parent
                .spawn((
                    Button,
                    PlayAgainButton,
                ))
                .with_children(|button| {
                    button.spawn((
                        Text("PLAY AGAIN?".to_string()),
                        TextFont {
                            font: font_handle,
                            font_size: 48.,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    ));
                });
        });
}

fn handle_play_again_button(
    mut commands: Commands,
    interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<PlayAgainButton>)>,
    mut text_color_query: Query<&mut TextColor>,
    mut next_state: ResMut<NextState<GameState>>,
    mut car_query: Query<(&mut Transform, &mut Velocity, &mut StuckTimer), With<Car>>,
    mut camera_query: Query<(Entity, &mut LookTransform), With<Camera3d>>,
) {
    for (interaction, children) in interaction_query.iter() {
        // Update text color on hover
        if let Some(child) = children.first().copied() {
            if let Ok(mut text_color) = text_color_query.get_mut(child) {
                match *interaction {
                    Interaction::Hovered => {
                        // Purple color from limbo_pass (AB69E7)
                        *text_color = TextColor(Color::srgb_u8(0xAB, 0x69, 0xE7));
                    }
                    Interaction::None => {
                        *text_color = TextColor(Color::srgb(0.9, 0.9, 0.9));
                    }
                    Interaction::Pressed => {
                        // Reset car to starting position - try to reset, but OnExit handler will ensure it happens
                        if let Ok((mut transform, mut velocity, mut stuck_timer)) = car_query.single_mut() {
                            transform.translation = CAR_START_POSITION;
                            transform.rotation = Quat::IDENTITY; // Ensure car is upright
                            velocity.linvel = Vec3::ZERO;
                            velocity.angvel = Vec3::ZERO;
                            stuck_timer.stuck_duration = 0.0; // Reset stuck timer to prevent immediate game over
                            stuck_timer.reset_grace_period = 0.5; // Give 0.5 seconds grace period after reset
                        }
                        
                        // Always transition to Running state - the OnExit handler will ensure car is reset
                        next_state.set(GameState::Running);
                        
                        // Reset camera to far out position (behind and high), then it will smoothly zoom in when activated
                        for (entity, mut look_transform) in camera_query.iter_mut() {
                            look_transform.eye = CAR_START_POSITION + CAMERA_OFFSET_FROM_CAR;
                            look_transform.target = CAR_START_POSITION;
                            // Re-add activation component so camera needs to be activated again
                            commands.entity(entity).insert(CameraNeedsActivation);
                        }
                    }
                }
            }
        }
    }
}

fn cleanup_game_over_screen(mut commands: Commands, query: Query<Entity, With<GameOverScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn handle_gamepad_play_again(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut car_query: Query<(&mut Transform, &mut Velocity, &mut StuckTimer), With<Car>>,
    mut camera_query: Query<(Entity, &mut LookTransform), With<Camera3d>>,
    action_state_query: Query<&ActionState<CarAction>, With<Car>>,
) {
    // Check if PlayAgain action is pressed (A button on Xbox controller)
    if let Ok(action_state) = action_state_query.single() {
        if action_state.just_pressed(&CarAction::PlayAgain) {
            // Same logic as handle_play_again_button when pressed
            if let Ok((mut transform, mut velocity, mut stuck_timer)) = car_query.single_mut() {
                transform.translation = CAR_START_POSITION;
                transform.rotation = Quat::IDENTITY;
                velocity.linvel = Vec3::ZERO;
                velocity.angvel = Vec3::ZERO;
                stuck_timer.stuck_duration = 0.0;
                stuck_timer.reset_grace_period = 0.5;
            }
            
            next_state.set(GameState::Running);
            
            for (entity, mut look_transform) in camera_query.iter_mut() {
                look_transform.eye = CAR_START_POSITION + CAMERA_OFFSET_FROM_CAR;
                look_transform.target = CAR_START_POSITION;
                commands.entity(entity).insert(CameraNeedsActivation);
            }
        }
    }
}

// Car starting position - edit this to change where the car spawns/respawns
pub const CAR_START_POSITION: Vec3 = Vec3::new(-700.0, 10.0, 0.0);

#[derive(Default, Component, Debug)]
pub struct Car {
    pub thrust: Vec3,
    pub drag: Vec3,
}
