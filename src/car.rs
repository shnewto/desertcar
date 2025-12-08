use crate::{camera::{look_and_orbit, setup as camera_setup}, input::get_car_movement, movement::apply_movement, scene, state::GameState};
use bevy::prelude::*;
use bevy_rapier3d::prelude::Velocity;
use smooth_bevy_cameras::LookTransform;

pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                get_car_movement,
                apply_movement.after(get_car_movement),
                look_and_orbit.after(apply_movement),
                check_game_over,
            )
                .run_if(in_state(GameState::Running)),
        )
        .add_systems(OnEnter(GameState::Running), spawn_controls_text)
        .add_systems(OnEnter(GameState::GameOver), spawn_game_over_screen)
        .add_systems(Update, handle_play_again_button.run_if(in_state(GameState::GameOver)))
        .add_systems(OnExit(GameState::GameOver), cleanup_game_over_screen);
    }
}

#[derive(Component)]
struct GameOverScreen;

#[derive(Component)]
struct PlayAgainButton;

#[derive(Component)]
struct ControlsText;

fn spawn_controls_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let controls_text = "drive
--------------------
↑ ↓ ← →
space bar

look
--------------------
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
    mut interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<PlayAgainButton>)>,
    mut text_color_query: Query<&mut TextColor>,
    mut next_state: ResMut<NextState<GameState>>,
    mut car_query: Query<(&mut Transform, &mut Velocity), With<Car>>,
    mut camera_query: Query<&mut LookTransform>,
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
                        // Reset car to starting position
                        if let Ok((mut transform, mut velocity)) = car_query.single_mut() {
                            let car_pos = Vec3::new(-700.0, 1.0, 0.0);
                            transform.translation = car_pos;
                            transform.rotation = Quat::IDENTITY;
                            velocity.linvel = Vec3::ZERO;
                            velocity.angvel = Vec3::ZERO;
                            
                            // Reset camera to follow car
                            if let Ok(mut look_transform) = camera_query.single_mut() {
                                look_transform.eye = Vec3::new(car_pos.x - 50.0, car_pos.y + 10.0, car_pos.z);
                                look_transform.target = car_pos;
                            }
                        }
                        // Return to running state
                        next_state.set(GameState::Running);
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

#[derive(Default, Component, Debug)]
pub struct Car {
    pub thrust: Vec3,
    pub drag: Vec3,
}
