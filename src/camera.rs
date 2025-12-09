use bevy::{
    math::Vec3,
    prelude::*,
};
use bevy_rapier3d::na::clamp;
use leafwing_input_manager::prelude::*;
use smooth_bevy_cameras::{LookAngles, LookTransform, LookTransformBundle, Smoother};

use crate::car::{Car, CAR_START_POSITION};
use crate::input::CarAction;

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
    car_query: Query<(&GlobalTransform, &Transform, &ActionState<CarAction>), CarQueryFilter>,
    camera_activation_query: Query<(), With<CameraNeedsActivation>>,
) {
    // Only apply follow logic if camera has been activated (key pressed)
    let _is_activated = camera_activation_query.is_empty();
    
    if let Ok(mut look_transform) = look_query.single_mut()
        && let Ok((_global_transform, car_transform, action_state)) = car_query.single() {
            let mut orbit = false;
            let orbit_factor = 2.0f32.to_radians(); // Camera orbit sensitivity

            let mut angles = LookAngles::from_vector(-look_transform.look_direction().unwrap());
            look_transform.target = car_transform.translation;

            // Keyboard camera controls
            if keys.pressed(KeyCode::KeyW) {
                angles.add_pitch(orbit_factor);
                orbit = true;
            }
            if keys.pressed(KeyCode::KeyS) {
                angles.add_pitch(-orbit_factor);
                orbit = true;
            }

            if keys.pressed(KeyCode::KeyD) {
                angles.add_yaw(orbit_factor);
                orbit = true;
            }

            if keys.pressed(KeyCode::KeyA) {
                angles.add_yaw(-orbit_factor);
                orbit = true;
            }
            
            // Gamepad left stick for camera control (using leafwing-input-manager)
            let camera_axis = action_state.axis_pair(&CarAction::CameraOrbit);
            if camera_axis.length_squared() > 0.01 {
                angles.add_yaw(camera_axis.x * orbit_factor);
                angles.add_pitch(-camera_axis.y * orbit_factor); // Invert Y for camera
                orbit = true;
            }

            // follow eye
            if orbit {
                look_transform.eye =
                    look_transform.target + 1.0 * look_transform.radius() * angles.unit_vector();
            }

            // Simple clamping-based follow (like the old implementation at 9ff6995)
            // This keeps the camera in a good position without complex distance adjustments
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

            // Always keep camera at a fixed height above the car (like old implementation)
            look_transform.eye.y = car_transform.translation.y + 10.0;
    }
}
