use bevy::{
    input::{Axis, Input},
    math::{Vec2, Vec3},
    prelude::{
        Camera, Commands, GamepadAxis, GamepadAxisType, Gamepads, GlobalTransform, KeyCode,
        PerspectiveCameraBundle, Query, Res, Transform, With, Without,
    },
};
use bevy_rapier3d::na::clamp;
use smooth_bevy_cameras::{LookAngles, LookTransform, LookTransformBundle, Smoother};

use crate::car::Car;

pub fn setup(mut commands: Commands) {
    //  Transform::from_xyz(-150.0, 0.5, 0.0);
    commands
        .spawn_bundle(LookTransformBundle {
            transform: LookTransform {
                eye: Vec3::new(-180.0, 10.5, 0.0),
                target: Vec3::new(-150.0, 1.0, 0.0),
            },
            smoother: Smoother::new(0.9),
        })
        .insert_bundle(PerspectiveCameraBundle::default());
}

// pub fn look(
//     mut look_query: Query<&mut LookTransform>,
//     car_query: Query<&GlobalTransform, With<Car>>,
// ) {
//     if let Ok(mut look_transform) = look_query.get_single_mut() {
//         if let Ok(global_transform) = car_query.get_single() {
//             look_transform.eye = Vec3::new(
//                 global_transform.translation.x - 40.0,
//                 global_transform.translation.y + 10.0,
//                 global_transform.translation.z,
//             );

//             look_transform.target = global_transform.translation;
//         }
//     }
// }

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
                
                let axis_rx = GamepadAxis(*gamepad, GamepadAxisType::LeftStickX);
                let axis_ry = GamepadAxis(*gamepad, GamepadAxisType::LeftStickY);

                if let (Some(x), Some(y)) = (axes.get(axis_rx), axes.get(axis_ry)) {
                    let stick_pos = Vec2::new(x, y);

                    // Example: check if the stick is pushed up
                    // if stick_pos.length() > 0.9 && stick_pos.y > 0.5 {
                    // movements
                    //     .0
                    //     .push(CarMovement::PushForward(push_factor * stick_pos.y.abs()))
                    // }

                    // Example: check if the stick is pushed down
                    // if stick_pos.length() > 0.9 && stick_pos.y < 0.5 {
                    // movements
                    //     .0
                    //     .push(CarMovement::PushBackward(push_factor * stick_pos.y.abs()))
                    // }

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


            if keys.pressed(KeyCode::A) {
                angles.add_yaw(-keyboard_orbit_factor.to_radians());
                // angles.add_pitch(0.0);
                orbit = true;
            }
            if keys.pressed(KeyCode::D) {
                angles.add_yaw(keyboard_orbit_factor.to_radians());
                // angles.add_pitch(0.0);
                orbit = true;
            }

            // follow eye
            if orbit {
                look_transform.eye = look_transform.target + 1.0 * look_transform.radius() * angles.unit_vector();
            }

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
        }
    }
}
