use bevy::{
    math::Vec3,
    prelude::{Commands, GlobalTransform, PerspectiveCameraBundle, Query, With},
};
use smooth_bevy_cameras::{LookTransform, LookTransformBundle, Smoother};

use crate::car::Car;

pub fn setup(mut commands: Commands) {
    //  Transform::from_xyz(-150.0, 0.5, 0.0);
    let eye = Vec3::new(-190.0, 10.5, 0.0);
    let target = Vec3::new(-150.0, 1.0, 0.0);

    commands
        .spawn_bundle(LookTransformBundle {
            transform: LookTransform { eye, target },
            smoother: Smoother::new(0.9),
        })
        .insert_bundle(PerspectiveCameraBundle::default());
}

pub fn move_camera(
    mut look_query: Query<&mut LookTransform>,
    car_query: Query<&GlobalTransform, With<Car>>,
) {
    if let Ok(mut look_transform) = look_query.get_single_mut() {
        if let Ok(global_transform) = car_query.get_single() {
            look_transform.eye = Vec3::new(
                global_transform.translation.x - 40.0,
                global_transform.translation.y + 10.0,
                global_transform.translation.z,
            );

            look_transform.target = global_transform.translation;
        }
    }
}
