use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::movement::{CarMovement, CarMovements};

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum CarAction {
    #[actionlike(Button)]
    PushForward,
    #[actionlike(Button)]
    PushBackward,
    #[actionlike(DualAxis)]
    Turn, // Left stick X axis for turning (keyboard: arrow left/right, gamepad: left stick)
    #[actionlike(Button)]
    Boost,
    #[actionlike(Button)]
    Reset,
    #[actionlike(DualAxis)]
    CameraOrbit, // Right stick for camera control
}

pub fn get_car_movement(
    mut query: Query<(&mut CarMovements, &mut Transform, &ActionState<CarAction>)>,
) {
    let (keyboard_turn_factor, gamepad_turn_factor) = (23.0, 10.0); // turn factor to compensate for high gravity
    let (keyboard_push_factor, gamepad_push_factor) = (13.0, 13.0);
    let (keyboard_boost_factor, gamepad_boost_factor) = (6.0, 6.0);

    if let Ok((mut movements, mut transform, action_state)) = query.single_mut() {
        // Check if gamepad is being used (check if analog stick has input)
        let turn_axis = action_state.axis_pair(&CarAction::Turn);
        let is_using_gamepad = turn_axis.length_squared() > 0.01;
        
        let turn_factor = if is_using_gamepad { gamepad_turn_factor } else { keyboard_turn_factor };
        let push_factor_base = if is_using_gamepad { gamepad_push_factor } else { keyboard_push_factor };
        let boost_factor = if is_using_gamepad { gamepad_boost_factor } else { keyboard_boost_factor };
        
        let push_factor = if action_state.pressed(&CarAction::Boost) {
            push_factor_base + boost_factor
        } else {
            push_factor_base
        };

        // Forward/backward movement
        if action_state.pressed(&CarAction::PushForward) {
            movements.0.push(CarMovement::PushForward(push_factor))
        }
        if action_state.pressed(&CarAction::PushBackward) {
            movements.0.push(CarMovement::PushBackward(push_factor))
        }
        
        // Turning - use X component of Turn axis (negative = left, positive = right)
        let turn_x = turn_axis.x;
        if turn_x.abs() > 0.1 {
            if turn_x < 0.0 {
                // Left (negative X)
                movements.0.push(CarMovement::TurnLeft(turn_factor * turn_x.abs()))
            } else {
                // Right (positive X)
                movements.0.push(CarMovement::TurnRight(turn_factor * turn_x))
            }
        }

        if action_state.just_pressed(&CarAction::Reset) {
            let y = transform.rotation.y;
            let w = transform.rotation.w;
            transform.rotation = Quat::from_vec4(Vec4::new(0.0, y, 0.0, w));
        }
    }
}

pub fn default_input_map() -> InputMap<CarAction> {
    use CarAction::*;
    let mut input_map = InputMap::default();

    // Keyboard mappings
    input_map.insert(PushForward, KeyCode::ArrowUp);
    input_map.insert(PushBackward, KeyCode::ArrowDown);
    // Turn is a DualAxis action - use VirtualDPad for keyboard (X axis = left/right)
    input_map.insert_dual_axis(Turn, VirtualDPad::arrow_keys());
    input_map.insert(Boost, KeyCode::Space);
    input_map.insert(Reset, KeyCode::Digit1);

    // Gamepad mappings (matching old implementation)
    input_map.insert(PushForward, GamepadButton::RightTrigger2);
    input_map.insert(Boost, GamepadButton::LeftTrigger2);
    input_map.insert(Reset, GamepadButton::North);
    
    // Gamepad right stick for turning (X axis) - matching old implementation
    input_map.insert_dual_axis(Turn, GamepadStick::RIGHT);
    
    // Gamepad left stick for camera control - matching old implementation
    input_map.insert_dual_axis(CameraOrbit, GamepadStick::LEFT);

    input_map
}
