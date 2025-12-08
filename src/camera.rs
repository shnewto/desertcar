use bevy::{
    math::Vec3,
    prelude::*,
};
use bevy_rapier3d::na::clamp;
use smooth_bevy_cameras::{LookAngles, LookTransform, LookTransformBundle, Smoother};

use crate::car::Car;

pub fn setup(
    mut commands: Commands,
    car_query: Query<(&Car, &Transform)>,
    camera_query: Query<Entity, With<Camera3d>>,
) {
    // Only spawn camera if one doesn't already exist
    if camera_query.is_empty() {
        let (_car, car_transform) = car_query.single().unwrap();
        let car_x = car_transform.translation.x;
        let car_y = car_transform.translation.y;
        let car_z = car_transform.translation.z;

        commands
            .spawn((
                Camera3d::default(),
                LookTransformBundle {
                    transform: LookTransform {
                        eye: Vec3::new(car_x - 50.0, car_y + 10.0, car_z),
                        target: Vec3::new(car_x, car_y, car_z),
                        up: Vec3::Y,
                    },
                    smoother: Smoother::new(0.9),
                },
            ));
    }
}

type CarQueryFilter = (With<Car>, Without<Camera>);

pub fn look_and_orbit(
    mut look_query: Query<&mut LookTransform>,
    keys: Res<ButtonInput<KeyCode>>,
    car_query: Query<(&GlobalTransform, &Transform), CarQueryFilter>,
) {
    if let Ok(mut look_transform) = look_query.single_mut()
        && let Ok((_global_transform, car_transform)) = car_query.single() {
            let mut orbit = false;
            let keyboard_orbit_factor = 120.0f32.to_radians();
            let _gamepad_orbit_factor = 5.0f32.to_radians();

            let mut angles = LookAngles::from_vector(-look_transform.look_direction().unwrap());
            look_transform.target = car_transform.translation;

            if keys.pressed(KeyCode::KeyW) {
                angles.add_pitch(keyboard_orbit_factor);
                orbit = true;
            }
            if keys.pressed(KeyCode::KeyS) {
                angles.add_pitch(-keyboard_orbit_factor);
                orbit = true;
            }

            if keys.pressed(KeyCode::KeyD) {
                angles.add_yaw(keyboard_orbit_factor);
                orbit = true;
            }

            if keys.pressed(KeyCode::KeyA) {
                angles.add_yaw(-keyboard_orbit_factor);
                orbit = true;
            }

            // follow eye
            if orbit {
                look_transform.eye =
                    look_transform.target + 1.0 * look_transform.radius() * angles.unit_vector();
            }

            // if orbit {
            // look_transform.eye.x = clamp(
            //     look_transform.eye.x,
            //     car_transform.translation.x - 80.0,
            //     car_transform.translation.x + 80.0,
            // );

            // look_transform.eye.z = clamp(
            //     look_transform.eye.z,
            //     car_transform.translation.z - 80.0,
            //     car_transform.translation.z + 80.0,
            // );

            // look_transform.eye.y = clamp(
            //     look_transform.eye.y,
            //     car_transform.translation.y + 10.0,
            //     car_transform.translation.y + 80.0,
            // );
            // } else {
            look_transform.eye.x = clamp(
                look_transform.eye.x,
                car_transform.translation.x - 40.0,
                car_transform.translation.x + 40.0,
            );

            look_transform.eye.z = clamp(
                look_transform.eye.z,
                car_transform.translation.z - 40.0,
                car_transform.translation.z + 40.0,
            );

            look_transform.eye.y = car_transform.translation.y + 10.0;
            // }

            // look_transform.eye.y = car_transform.translation.y + 10.0;
    }
}
