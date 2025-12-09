use bevy::{light::PointLightShadowMap, prelude::*, window::{PresentMode, WindowResolution}};
use bevy_kira_audio::AudioPlugin;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::*;
use smooth_bevy_cameras::LookTransformPlugin;
use crate::state::GameState;

mod assets;
mod camera;
mod car;
mod lighting;
mod movement;
mod scene;
mod state;
mod input; 

#[derive(Component)]
struct DriveScreen;

#[derive(Component)]
struct DriveButton;

fn main() {
    App::new()
        .insert_resource(PointLightShadowMap { size: 2048 })
        .insert_resource(ClearColor(
            Color::srgb_u8(0x00, 0x00, 0x00), // Black
        ))
        .insert_resource(assets::SceneResource::default())
        .add_plugins(DefaultPlugins
            .set(AssetPlugin {
                meta_check: bevy::asset::AssetMetaCheck::Never,
                ..default()
            })
            .set(WindowPlugin {
            primary_window: Some(Window {
                title: "desert-car".to_string(),
                resolution: WindowResolution::new(1280, 720),
                // Fifo doesn't work on WASM, use AutoVsync instead
                present_mode: if cfg!(target_arch = "wasm32") {
                    PresentMode::AutoVsync
                } else {
                    PresentMode::Fifo
                },
                // Make window fill the viewport on WASM
                fit_canvas_to_parent: cfg!(target_arch = "wasm32"),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .add_plugins((
            LookTransformPlugin,
            AudioPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            InputManagerPlugin::<input::CarAction>::default(),
        ))
        // Setup state (drive button) - start loading assets and show drive screen
        .add_systems(OnEnter(GameState::Setup), (
            spawn_setup_camera,
            lighting::setup,
            assets::load,
            spawn_drive_screen,
        ))
        .add_systems(Update, (
            handle_drive_button.run_if(in_state(GameState::Setup)),
        ))
        .add_systems(OnExit(GameState::Setup), (
            cleanup_drive_screen,
        ))
        // Running state - spawn scene here, not in Setup
        .add_systems(OnEnter(GameState::Running), (
            scene::setup,
            camera::setup,
            car::spawn_controls_text,
        ))
        .add_plugins(car::CarPlugin)
        // .add_plugins(RapierDebugRenderPlugin::default())
        .run();
}

fn spawn_setup_camera(mut commands: Commands) {
    commands.spawn(Camera3d::default());
}

fn spawn_drive_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            DriveScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    DriveButton,
                ))
                .with_children(|button| {
                    button.spawn((
                        Text("drive".to_string()),
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

fn handle_drive_button(
    interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<DriveButton>)>,
    mut text_color_query: Query<&mut TextColor>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let purple_color = Color::srgb_u8(0xAB, 0x69, 0xE7);
    
    for (interaction, children) in interaction_query.iter() {
        if let Some(child) = children.first().copied() {
            if let Ok(mut text_color) = text_color_query.get_mut(child) {
                match *interaction {
                    Interaction::Pressed => {
                        next_state.set(GameState::Running);
                    }
                    Interaction::Hovered => {
                        *text_color = TextColor(purple_color);
                    }
                    Interaction::None => {
                        *text_color = TextColor(Color::srgb(0.9, 0.9, 0.9));
                    }
                }
            }
        }
    }
}

fn cleanup_drive_screen(mut commands: Commands, query: Query<Entity, With<DriveScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
