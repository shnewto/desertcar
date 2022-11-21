use crate::state::GameState;
use bevy::{asset::LoadState, prelude::*};
use bevy::gltf::Gltf;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SceneResource::default())
            .add_system_set(SystemSet::on_enter(GameState::LoadingAssets).with_system(load))
            .add_system_set(SystemSet::on_update(GameState::LoadingAssets).with_system(check));
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
    mut state: ResMut<State<GameState>>,
    asset_server: Res<AssetServer>,
    scene_assets: Res<SceneResource>,
) {
    if let LoadState::Loaded = asset_server.get_load_state(&scene_assets.handle) {
        state.set(GameState::Setup).unwrap();
    }
}
