use bevy::{
    math::Vec3,
    prelude::*,
};
use bevy_rapier3d::na::clamp;
use smooth_bevy_cameras::{LookAngles, LookTransform, LookTransformBundle, Smoother};

use crate::car::{Car, CAR_START_POSITION};

// Camera offset from car start position - edit this to change initial camera position
pub const CAMERA_OFFSET_FROM_CAR: Vec3 = Vec3::new(-32.0, 16.0, -8.0);

#[derive(Component)]
pub struct CameraNeedsActivation;

pub fn setup(
    mut commands: Commands,
    camera_query: Query<Entity, With<Camera3d>>,
) {
    // Only spawn camera if one doesn't already exist
    if camera_query.is_empty() {
        // Start camera far out, high up, and directly behind the car, then it will smoothly zoom in when activated
        commands
            .spawn((
                Camera3d::default(),
                LookTransformBundle {
                    transform: LookTransform {
                        eye: CAR_START_POSITION + CAMERA_OFFSET_FROM_CAR, 
                        target: CAR_START_POSITION,
                        up: Vec3::Y,
                    },
                    smoother: Smoother::new(0.9),
                },
                CameraNeedsActivation,
            ));
    }
}

type CarQueryFilter = (With<Car>, Without<Camera>);

pub fn activate_camera_on_input(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    camera_query: Query<Entity, (With<Camera3d>, With<CameraNeedsActivation>)>,
) {
    // If any key is pressed, remove the activation component to enable normal follow behavior
    if keys.get_pressed().next().is_some() {
        for entity in camera_query.iter() {
            commands.entity(entity).remove::<CameraNeedsActivation>();
        }
    }
}

pub fn look_and_orbit(
    mut look_query: Query<&mut LookTransform>,
    keys: Res<ButtonInput<KeyCode>>,
    car_query: Query<(&GlobalTransform, &Transform), CarQueryFilter>,
    camera_activation_query: Query<(), With<CameraNeedsActivation>>,
) {
    // Only apply follow logic if camera has been activated (key pressed)
    let is_activated = camera_activation_query.is_empty();
    
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

            // If not orbiting and camera is activated, smoothly follow the car
            if !orbit && is_activated {
                let current_distance = look_transform.radius();
                
                // Only zoom in if we're significantly far away (more than 60 units)
                // If already close, just let it follow naturally without forcing a distance
                if current_distance > 60.0 {
                    let target_distance = 50.0;
                    // Gradually zoom in from far position
                    let direction = (look_transform.eye - look_transform.target).normalize();
                    let new_distance = current_distance * 0.95 + target_distance * 0.05; // Smooth interpolation
                    look_transform.eye = look_transform.target + direction * new_distance;
                }
                // If already close (<= 60 units), don't adjust distance - just let it follow naturally
            }
            
            // Only clamp when camera is at follow distance (not when close from reset)
            let current_distance = look_transform.radius();
            if current_distance >= 40.0 && current_distance <= 60.0 {
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
            }

            // Always ensure camera is high enough above ground
            let min_height = car_transform.translation.y + 10.0;
            look_transform.eye.y = look_transform.eye.y.max(min_height);
    }
}
