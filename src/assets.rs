use bevy::prelude::*;
use bevy::gltf::Gltf;

#[derive(Component, Debug, Default, Resource)]
pub struct SceneResource {
    pub handle: Handle<Gltf>,
}

pub fn load(asset_server: ResMut<AssetServer>, mut scene_assets: ResMut<SceneResource>) {
    scene_assets.handle = asset_server.load("gltf/desertcar.gltf");
}
