use bevy::{
    math::Vec3,
    prelude::{Commands, PerspectiveCameraBundle},
};
use smooth_bevy_cameras::controllers::orbit::{OrbitCameraBundle, OrbitCameraController};

pub fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrbitCameraBundle::new(
        OrbitCameraController::default(),
        PerspectiveCameraBundle::default(),
        Vec3::new(-100.0, 60.0, 20.0),
        Vec3::new(0.0, 0.0, 0.0),
    ));
}
