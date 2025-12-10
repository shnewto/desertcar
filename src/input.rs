use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::movement::{CarMovement, CarMovements};

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum CarAction {
    #[actionlike(Button)]
    PushForward,
    #[actionlike(Button)]
    PushBackward,
    #[actionlike(Button)]
    TurnLeft,
    #[actionlike(Button)]
    TurnRight,
    #[actionlike(DualAxis)]
    TurnAxis, // Gamepad right stick for turning
    #[actionlike(Button)]
    Boost,
    #[actionlike(Button)]
    Reset,
    #[actionlike(DualAxis)]
    CameraOrbit, // Left stick for camera control
    #[actionlike(Button)]
    PlayAgain, // For game over screen
}

pub fn get_car_movement(
    mut query: Query<(&mut CarMovements, &mut Transform, &ActionState<CarAction>)>,
) {
    let turn_factor = 23.0; // turn factor to compensate for high gravity
    let push_factor = 13.0;
    let boost_factor = 6.0;

    if let Ok((mut movements, mut transform, action_state)) = query.single_mut() {
        // Both keyboard and gamepad work simultaneously
        let current_push_factor = if action_state.pressed(&CarAction::Boost) {
            push_factor + boost_factor
        } else {
            push_factor
        };

        // Forward/backward movement
        if action_state.pressed(&CarAction::PushForward) {
            movements.0.push(CarMovement::PushForward(current_push_factor))
        }
        if action_state.pressed(&CarAction::PushBackward) {
            movements.0.push(CarMovement::PushBackward(current_push_factor))
        }
        
        // Keyboard turning (button presses) - always works, even if gamepad is also being used
        if action_state.pressed(&CarAction::TurnLeft) {
            movements.0.push(CarMovement::TurnLeft(turn_factor))
        }
        if action_state.pressed(&CarAction::TurnRight) {
            movements.0.push(CarMovement::TurnRight(turn_factor))
        }
        
        // Gamepad turning (dual axis - right stick X) - works simultaneously with keyboard
        let turn_axis = action_state.axis_pair(&CarAction::TurnAxis);
        let turn_x = turn_axis.x;
        if turn_x.abs() > 0.01 {
            let turn_force = turn_factor * turn_x.abs(); // Use same turn factor for gamepad
            if turn_x < 0.0 {
                movements.0.push(CarMovement::TurnLeft(turn_force))
            } else {
                movements.0.push(CarMovement::TurnRight(turn_force))
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
    input_map.insert(TurnLeft, KeyCode::ArrowLeft); // Keyboard button for turning left
    input_map.insert(TurnRight, KeyCode::ArrowRight); // Keyboard button for turning right
    input_map.insert(Boost, KeyCode::Space);
    input_map.insert(Reset, KeyCode::Digit1);

    // Gamepad mappings (matching old implementation)
    input_map.insert(PushForward, GamepadButton::RightTrigger2);
    input_map.insert(Boost, GamepadButton::LeftTrigger2);
    input_map.insert(Reset, GamepadButton::North);
    
    // Gamepad right stick for turning (X axis) - matching old implementation
    input_map.insert_dual_axis(TurnAxis, GamepadStick::RIGHT);
    
    // Gamepad left stick for camera control - matching old implementation
    input_map.insert_dual_axis(CameraOrbit, GamepadStick::LEFT);
    
    // Play again button - A button on Xbox controller (South button)
    input_map.insert(PlayAgain, GamepadButton::South);

    input_map
}
