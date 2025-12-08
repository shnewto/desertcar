use bevy::{light::PointLightShadowMap, prelude::*, window::{PresentMode, WindowResolution}};
use bevy_kira_audio::AudioPlugin;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::*;
use smooth_bevy_cameras::LookTransformPlugin;

mod assets;
mod camera;
mod car;
mod lighting;
mod movement;
mod scene;
mod setup;
mod state;
mod input; 

fn main() {
    App::new()
        .insert_resource(PointLightShadowMap { size: 2048 })
        .insert_resource(ClearColor(
            Color::srgb_u8(0xE5, 0xD9, 0xAB),
        ))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "desert-car".to_string(),
                resolution: WindowResolution::new(1280, 720),
                present_mode: PresentMode::Fifo,
                ..default()
            }),
            ..default()
        }))
        .init_state::<state::GameState>()
        .add_plugins((
            LookTransformPlugin,
            AudioPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            InputManagerPlugin::<input::CarAction>::default(),
            GamePlugin,
        ))
        // .add_plugins(RapierDebugRenderPlugin::default())
        .run();
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            assets::AssetsPlugin,
            setup::SetupPlugin,
            car::CarPlugin,
        ));
    }
}
