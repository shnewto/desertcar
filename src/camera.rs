use bevy::{
    input::{Axis, Input},
    math::{Vec2, Vec3},
    prelude::{
        Camera, Commands, GamepadAxis, GamepadAxisType, Gamepads, GlobalTransform, KeyCode,
        Query, Res, Transform, With, Without,
    },
};
use bevy::prelude::Camera3dBundle;
use bevy_rapier3d::na::clamp;
use smooth_bevy_cameras::{LookAngles, LookTransform, LookTransformBundle, Smoother};

use crate::car::Car;

pub fn setup(mut commands: Commands, car_query: Query<(&Car, &Transform)>) {
    //  Transform::from_xyz(-150.0, 0.5, 0.0);

    let (_car, car_transform) = car_query.get_single().unwrap();
    let car_x = car_transform.translation.x;
    let car_y = car_transform.translation.y;
    let car_z = car_transform.translation.z;

    commands
        .spawn(Camera3dBundle::default())
        .insert(LookTransformBundle {
            transform: LookTransform {
                eye: Vec3::new(car_x - 50.0, car_y + 10.0, car_z),
                target: Vec3::new(car_x, car_y, car_z),
            },
            smoother: Smoother::new(0.9),
        });
}

pub fn look_and_orbit(
    mut look_query: Query<&mut LookTransform>,
    keys: Res<Input<KeyCode>>,
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
    car_query: Query<(&GlobalTransform, &Transform, With<Car>, Without<Camera>)>,
) {
    if let Ok(mut look_transform) = look_query.get_single_mut() {
        if let Ok((_global_transform, car_transform, _, _)) = car_query.get_single() {
            let mut orbit = false;
            let keyboard_orbit_factor = 120.0f32.to_radians();
            let gamepad_orbit_factor = 5.0f32.to_radians();

            let mut angles = LookAngles::from_vector(-look_transform.look_direction().unwrap());
            look_transform.target = car_transform.translation;

            if let Some(gamepad) = gamepads.iter().next() {

                let axis_rx = GamepadAxis { gamepad, axis_type: GamepadAxisType::LeftStickX };
                let axis_ry = GamepadAxis { gamepad, axis_type: GamepadAxisType::LeftStickY };

                if let (Some(x), Some(y)) = (axes.get(axis_rx), axes.get(axis_ry)) {
                    let stick_pos = Vec2::new(x, y);

                    // Example: check if the stick is pushed up
                    if stick_pos.length() > 0.9 && stick_pos.y > 0.5 {
                        angles.add_pitch(gamepad_orbit_factor);
                        orbit = true;
                    }

                    // Example: check if the stick is pushed down
                    if stick_pos.length() > 0.9 && stick_pos.y < 0.5 {
                        angles.add_pitch(-gamepad_orbit_factor);
                        orbit = true;
                    }

                    // Example: check if the stick is pushed right
                    if stick_pos.length() > 0.9 && stick_pos.x > 0.5 {
                        angles.add_yaw(gamepad_orbit_factor);
                        orbit = true;
                    }

                    // Example: check if the stick is pushed left
                    if stick_pos.length() > 0.9 && stick_pos.x < 0.5 {
                        angles.add_yaw(-gamepad_orbit_factor);
                        orbit = true;
                    }
                }
            }

            if keys.pressed(KeyCode::W) {
                angles.add_pitch(keyboard_orbit_factor.to_radians());
                orbit = true;
            }
            if keys.pressed(KeyCode::S) {
                angles.add_pitch(-keyboard_orbit_factor.to_radians());
                orbit = true;
            }

            if keys.pressed(KeyCode::D) {
                angles.add_yaw(keyboard_orbit_factor.to_radians());
                orbit = true;
            }

            if keys.pressed(KeyCode::A) {
                angles.add_yaw(-keyboard_orbit_factor.to_radians());
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
}
