use bevy::input::gamepad::GamepadButton;
use bevy::prelude::*;

use crate::movement::{CarMovement, CarMovements};

pub fn get_car_movement(
    mut query: Query<(&mut CarMovements, &mut Transform)>,
    keys: Res<Input<KeyCode>>,
    gamepads: Res<Gamepads>,
    button_inputs: Res<Input<GamepadButton>>,
    button_axes: Res<Axis<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
) {
    let (keyboard_turn_factor, gamepad_turn_factor) = (10.0, 10.0);
    let (keyboard_push_factor, gamepad_push_factor) = (13.0, 13.0);
    let (keyboard_boost_factor, gamepad_boost_factor) = (6.0, 6.0);

    if let Ok((mut movements, mut transform)) = query.get_single_mut() {
        if let Some(gamepad) = gamepads.iter().next() {
            let right_trigger = button_axes
                .get(GamepadButton(*gamepad, GamepadButtonType::RightTrigger2))
                .unwrap();

            let left_trigger = button_axes
                .get(GamepadButton(*gamepad, GamepadButtonType::LeftTrigger2))
                .unwrap();

            let push_factor = if left_trigger.abs() > 0.01 {
                gamepad_push_factor + gamepad_boost_factor
            } else {
                gamepad_push_factor
            };

            let axis_rx = GamepadAxis(*gamepad, GamepadAxisType::RightStickX);
            let axis_ry = GamepadAxis(*gamepad, GamepadAxisType::RightStickY);

            if let (Some(x), Some(y)) = (axes.get(axis_rx), axes.get(axis_ry)) {
                // combine X and Y into one vector
                let stick_pos = Vec2::new(x, y);

                // Example: check if the stick is pushed up
                if right_trigger.abs() > 0.01 {
                    // if stick_pos.length() > 0.9 && stick_pos.y > 0.5 {
                    movements.0.push(CarMovement::PushForward(push_factor))
                }

                if stick_pos.length() > 0.9 && stick_pos.y < -0.5 {
                    // if left_trigger.abs() > 0.01 {
                    movements.0.push(CarMovement::PushBackward(push_factor))
                }

                if stick_pos.length() > 0.9 && stick_pos.x > 0.5 {
                    movements
                        .0
                        .push(CarMovement::TurnRight(gamepad_turn_factor))
                }

                if stick_pos.length() > 0.9 && stick_pos.x < -0.5 {
                    movements.0.push(CarMovement::TurnLeft(gamepad_turn_factor))
                }
            }

            if button_inputs.just_pressed(GamepadButton(*gamepad, GamepadButtonType::North)) {
                // transform.rotation = Quat::from_rotation_x(0.0);
                let x = transform.rotation.x;
                let y = transform.rotation.y;
                let z = transform.rotation.z;
                let w = transform.rotation.w;
                transform.rotation = Quat::from_vec4(Vec4::new(x, y, z, w));
                transform.translation.y = 10.0;
            }
        }
        let push_factor = if keys.pressed(KeyCode::Space) {
            keyboard_push_factor + keyboard_boost_factor
        } else {
            keyboard_push_factor
        };

        if keys.pressed(KeyCode::Up) {
            movements.0.push(CarMovement::PushForward(push_factor))
        }
        if keys.pressed(KeyCode::Down) {
            movements.0.push(CarMovement::PushBackward(push_factor))
        }
        if keys.pressed(KeyCode::Left) {
            movements
                .0
                .push(CarMovement::TurnLeft(keyboard_turn_factor))
        }
        if keys.pressed(KeyCode::Right) {
            movements
                .0
                .push(CarMovement::TurnRight(keyboard_turn_factor))
        }

        // eprintln!("x: {:#?}", transform.rotation.x.to_degrees());
        // eprintln!("y: {:#?}", transform.rotation.y.to_degrees());
        // eprintln!("z: {:#?}", transform.rotation.z.to_degrees());
        // eprintln!("w: {:#?}", transform.rotation.w.to_degrees());

        // let min_pitch_roll = -30.0f32.to_radians();
        // let max_pitch_roll = 30.0f32.to_radians();
        // let clamped_rotation_x = clamp(transform.rotation.x, min_pitch_roll, max_pitch_roll);
        // let clamped_rotation_z = clamp(transform.rotation.z, min_pitch_roll, max_pitch_roll);
        // transform.rotation = Quat::from_vec4(Vec4::new(
        //     clamped_rotation_x,
        //     transform.rotation.y,
        //     clamped_rotation_z,
        //     transform.rotation.w,
        // ));

        if keys.pressed(KeyCode::Key1) {
            let _x = transform.rotation.x;
            let _z = transform.rotation.z;

            let y = transform.rotation.y;
            let w = transform.rotation.w;
            transform.rotation = Quat::from_vec4(Vec4::new(0.0, y, 0.0, w));
            // transform.rotation = Quat::from_rotation_x(0.0);
            // transform.translation.y = 20.0;
        }
    }
}
