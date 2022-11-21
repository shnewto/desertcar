use bevy::{prelude::*, window::PresentMode};
use bevy_kira_audio::AudioPlugin;
use bevy_rapier3d::prelude::*;
use smooth_bevy_cameras::LookTransformPlugin;

mod assets;
mod camera;
mod car;
mod lighting;
mod movement;
mod physics;
mod scene;
mod setup;
mod state;
mod input; 

fn main() {
    let clear_color_hex_string = "E5D9AB";
    App::new()
        .insert_resource(Msaa::default())
        .insert_resource(ClearColor(
            Color::hex(clear_color_hex_string).unwrap_or_else(|_| {
                panic!("couldn't make hex color from {}", clear_color_hex_string)
            }),
        ))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 1280.,
                height: 720.,
                title: "desert-car".to_string(),
                present_mode: PresentMode::Fifo,
                ..default()
            },
            ..default()
        }))
        .add_plugin(LookTransformPlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(GamePlugin)
        .run();
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(state::GameState::LoadingAssets)
            .add_plugin(assets::AssetsPlugin)
            .add_plugin(setup::SetupPlugin)
            .add_plugin(car::CarPlugin);
    }
}
