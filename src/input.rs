use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::movement::{CarMovement, CarMovements};

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum CarAction {
    PushForward,
    PushBackward,
    TurnLeft,
    TurnRight,
    Boost,
    Reset,
}

pub fn get_car_movement(
    mut query: Query<(&mut CarMovements, &mut Transform, &ActionState<CarAction>)>,
) {
    let (keyboard_turn_factor, _gamepad_turn_factor) = (23.0, 10.0); // turn factor to compensate for high gravity
    let (keyboard_push_factor, _gamepad_push_factor) = (13.0, 13.0);
    let (keyboard_boost_factor, _gamepad_boost_factor) = (6.0, 6.0);

    if let Ok((mut movements, mut transform, action_state)) = query.single_mut() {
        let push_factor = if action_state.pressed(&CarAction::Boost) {
            keyboard_push_factor + keyboard_boost_factor
        } else {
            keyboard_push_factor
        };

        if action_state.pressed(&CarAction::PushForward) {
            movements.0.push(CarMovement::PushForward(push_factor))
        }
        if action_state.pressed(&CarAction::PushBackward) {
            movements.0.push(CarMovement::PushBackward(push_factor))
        }
        if action_state.pressed(&CarAction::TurnLeft) {
            movements.0.push(CarMovement::TurnLeft(keyboard_turn_factor))
        }
        if action_state.pressed(&CarAction::TurnRight) {
            movements.0.push(CarMovement::TurnRight(keyboard_turn_factor))
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

    input_map.insert(PushForward, KeyCode::ArrowUp);
    input_map.insert(PushBackward, KeyCode::ArrowDown);
    input_map.insert(TurnLeft, KeyCode::ArrowLeft);
    input_map.insert(TurnRight, KeyCode::ArrowRight);
    input_map.insert(Boost, KeyCode::Space);
    input_map.insert(Reset, KeyCode::Digit1);

    input_map.insert(PushForward, GamepadButton::RightTrigger2);
    input_map.insert(Boost, GamepadButton::LeftTrigger2);
    input_map.insert(Reset, GamepadButton::North);

    input_map
}
