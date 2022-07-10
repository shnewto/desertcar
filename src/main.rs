use bevy::{prelude::*, window::PresentMode};
use bevy_kira_audio::AudioPlugin;
use bevy_rapier3d::prelude::*;
use smooth_bevy_cameras::{controllers::orbit::OrbitCameraPlugin, LookTransformPlugin};

mod assets;
mod car;
mod camera;
mod lighting;
mod physics;
mod scene;
mod setup;
mod state;
mod movement;
 
fn main() {
    let clear_color_hex_string = "874f9c";
    App::new()
        .insert_resource(Msaa::default())
        .insert_resource(WindowDescriptor {
            width: 1280.,
            height: 720.,
            title: "car-in-desert".to_string(),
            present_mode: PresentMode::Fifo,
            ..default()
        })
        .insert_resource(ClearColor(
            Color::hex(clear_color_hex_string).unwrap_or_else(|_| {
                panic!("couldn't make hex color from {}", clear_color_hex_string)
            }),
        ))
        .add_plugins(DefaultPlugins)
        .add_plugin(LookTransformPlugin)
        .add_plugin(OrbitCameraPlugin::default())
        .add_plugin(AudioPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(GamePlugin)
        // .add_plugin(ConfigCam)
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
