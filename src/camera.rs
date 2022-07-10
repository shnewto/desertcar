use bevy::{
    math::{Quat, Vec2, Vec3, Vec4},
    prelude::{
        Camera, Commands, GlobalTransform, PerspectiveCameraBundle, Query, Transform, With, Without,
    },
    transform,
};
use smooth_bevy_cameras::{
    controllers::orbit::{OrbitCameraBundle, OrbitCameraController},
    LookAngles, LookTransform, LookTransformBundle, Smoother,
};

use crate::car::Car;

pub fn setup(mut commands: Commands) {
    // commands.spawn_bundle(OrbitCameraBundle::new(
    //     OrbitCameraController::default(),
    //     PerspectiveCameraBundle::default(),
    //     Vec3::new(-180.0, 10.0, 0.0),
    //     Vec3::new(150.0, 1.0, 0.0),
    // ));

    //  Transform::from_xyz(-150.0, 0.5, 0.0);

    let eye = Vec3::new(-180.0, 10.0, 0.0);
    let target = Vec3::new(150.0, 1.0, 0.0);

    commands
        .spawn_bundle(LookTransformBundle {
            transform: LookTransform { eye, target },
            smoother: Smoother::new(0.9),
        })
        .insert_bundle(PerspectiveCameraBundle::default());
}

pub fn move_camera(
    mut look_query: Query<(&mut LookTransform)>,
    car_query: Query<(&Car, &GlobalTransform, &Transform)>,
) {
    if let Ok((mut look_transform)) = look_query.get_single_mut() {
        if let Ok((car, global_transform, car_transform)) = car_query.get_single() {
            look_transform.eye = Vec3::new(
                global_transform.translation.x - 40.0,
                global_transform.translation.y + 10.0,
                global_transform.translation.z,
            );

            // look_transform.target = global_transform.translation;

            // if (car.last_rotation.x != car_transform.rotation.x)
            //     || (car.last_rotation.y != car_transform.rotation.y)
            // {
            // let delta = Vec2::new(
            //     car_transform.translation.x - car.last_translation.x,
            //     car_transform.translation.y - car.last_translation.y,
            // );

            let mut angles = LookAngles::from_vector(look_transform.look_direction().unwrap());
            // angles.add_pitch(0.0);
            // angles.add_yaw(global_transform.rotation.x - car.last_rotation.x);
            // look_transform.eye =
            //     look_transform.target + 1.0 * look_transform.radius() * angles.unit_vector();
        }
    }
}
