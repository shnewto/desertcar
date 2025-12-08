use crate::state::GameState;
use bevy::{asset::LoadState, prelude::*};
use bevy::gltf::Gltf;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SceneResource::default())
            .add_systems(OnEnter(GameState::LoadingAssets), load)
            .add_systems(Update, check.run_if(in_state(GameState::LoadingAssets)));
    }
}

#[derive(Component, Debug, Default, Resource)]
pub struct SceneResource {
    pub handle: Handle<Gltf>,
}

fn load(asset_server: ResMut<AssetServer>, mut scene_assets: ResMut<SceneResource>) {
    scene_assets.handle = asset_server.load("gltf/desert-car.gltf");
}

fn check(
    mut next_state: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
    scene_assets: Res<SceneResource>,
) {
    if let LoadState::Loaded = asset_server.load_state(&scene_assets.handle) {
        next_state.set(GameState::Setup);
    }
}
