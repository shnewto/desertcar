use bevy::{
    math::Vec3,
    prelude::{Commands, PerspectiveCameraBundle},
};
use smooth_bevy_cameras::controllers::orbit::{OrbitCameraBundle, OrbitCameraController};

pub fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrbitCameraBundle::new(
        OrbitCameraController::default(),
        PerspectiveCameraBundle::default(),
        Vec3::new(-180.0, 10.0, 0.0),
        Vec3::new(150.0, 1.0, 0.0),
    ));
}
