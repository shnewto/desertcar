use crate::state::GameState;
use bevy::{asset::LoadState, prelude::*};

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SceneAssets::default())
            .add_system_set(SystemSet::on_enter(GameState::LoadingAssets).with_system(load))
            .add_system_set(SystemSet::on_update(GameState::LoadingAssets).with_system(check));
    }
}

#[derive(Component, Debug, Default)]
pub struct SceneAssets {
    pub handle: Handle<Scene>,
}

fn load(asset_server: ResMut<AssetServer>, mut scene_assets: ResMut<SceneAssets>) {
    scene_assets.handle = asset_server.load("gltf/buggy.gltf");
}

fn check(
    mut state: ResMut<State<GameState>>,
    asset_server: Res<AssetServer>,
    scene_assets: Res<SceneAssets>,
) {
    if let LoadState::Loaded = asset_server.get_load_state(&scene_assets.handle) {
        state.set(GameState::Setup).unwrap();
    }
}
