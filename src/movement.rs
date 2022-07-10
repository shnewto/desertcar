use bevy::input::gamepad::GamepadButton;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::car::Car;

#[derive(Debug)]
pub enum Movement {
    PushForward(f32),
    PushBackward(f32),
    TurnLeft(f32),
    TurnRight(f32),
}

impl Movement {
    pub fn as_lin_vec(&self) -> Vec3 {
        match self {
            Self::PushForward(p) => Vec3::new(*p, 0.0, 0.0),
            Self::PushBackward(p) => Vec3::new(-*p, 0.0, 0.0),
            _ => Vec3::new(0.0, 0.0, 0.0),
        }
    }
    pub fn as_ang_vec(&self) -> Vec3 {
        match self {
            Self::TurnLeft(p) => Vec3::new(0.0, *p, 0.0),
            Self::TurnRight(p) => Vec3::new(0.0, -*p, 0.0),
            _ => Vec3::new(0.0, 0.0, 0.0),
        }
    }
}

#[derive(Default, Component, Debug)]
pub struct Movements(pub Vec<Movement>);


pub fn get_movement(
    query: Query<(&mut Movements, &mut Transform)>,
    keys: Res<Input<KeyCode>>,
    _gamepads: Res<Gamepads>,
    _button_inputs: Res<Input<GamepadButton>>,
    _button_axes: Res<Axis<GamepadButton>>,
    _axes: Res<Axis<GamepadAxis>>,
) {
    // if let Some(gamepad) = gamepads.iter().next() {
    //         gamepad_movement(query, *gamepad, button_inputs, button_axes, axes)
    // } 

    keyboard_movement(query, keys)
}


fn keyboard_movement(
    mut query: Query<(&mut Movements, &mut Transform)>,
    keys: Res<Input<KeyCode>>,
    
) {
    if let Ok((mut movements, mut transform)) = query.get_single_mut() {
        let push_factor;

        if keys.pressed(KeyCode::Space) {
            push_factor = 19.0;
        } else {
            push_factor = 13.0;
        }

        let turn_factor = 10.0;

        if keys.pressed(KeyCode::W) || keys.pressed(KeyCode::Up) {
            movements.0.push(Movement::PushForward(push_factor))
        }
        if keys.pressed(KeyCode::S) || keys.pressed(KeyCode::Down) {
            movements.0.push(Movement::PushBackward(push_factor))
        }
        if keys.pressed(KeyCode::A) || keys.pressed(KeyCode::Left) {
            movements.0.push(Movement::TurnLeft(turn_factor))
        }
        if keys.pressed(KeyCode::D) || keys.pressed(KeyCode::Right) {
            movements.0.push(Movement::TurnRight(turn_factor))
        }

        if keys.pressed(KeyCode::Key1) {
            movements.0.clear();
            transform.rotation = Quat::from_rotation_x(0.0);
            transform.translation.y = 10.0;
        }
    }
}

fn _gamepad_movement(
    mut query: Query<(&mut Movements, &mut Transform)>,
    gamepad: Gamepad,
    button_inputs: Res<Input<GamepadButton>>,
    button_axes: Res<Axis<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
) {
        if let Ok((mut movements, mut transform)) = query.get_single_mut() {

            let push_factor;
            let turn_factor = 10.0;

            let right_trigger = button_axes
                .get(GamepadButton(gamepad,GamepadButtonType::RightTrigger2,))
                .unwrap();
            if right_trigger.abs() > 0.01 {
                push_factor = 19.0;
            } else {
                push_factor = 13.0;
            }

            
            let axis_rx = GamepadAxis(gamepad, GamepadAxisType::RightStickX);
            let axis_ry = GamepadAxis(gamepad, GamepadAxisType::RightStickY);
        
            if let (Some(x), Some(y)) = (axes.get(axis_rx), axes.get(axis_ry)) {
                // combine X and Y into one vector
                let stick_pos = Vec2::new(x, y);
        
                // Example: check if the stick is pushed up
                if stick_pos.length() > 0.9 && stick_pos.y > 0.5 {
                    movements.0.push(Movement::PushForward(push_factor * stick_pos.y.abs()))
                } 
                
                if stick_pos.length() > 0.9 && stick_pos.y < 0.5 {
                    movements.0.push(Movement::PushBackward(push_factor * stick_pos.y.abs()))
                }

                
                if stick_pos.length() > 0.9 && stick_pos.x < 0.5 {
                    movements.0.push(Movement::TurnRight(turn_factor * stick_pos.x.abs()))
                }
                
                if stick_pos.length() > 0.9 && stick_pos.x > 0.5 {
                    movements.0.push(Movement::TurnLeft(turn_factor * stick_pos.x.abs()))
                }
            }
            
            let axis_lx = GamepadAxis(gamepad, GamepadAxisType::LeftStickX);
            let axis_ly = GamepadAxis(gamepad, GamepadAxisType::LeftStickY);
        
            if let (Some(x), Some(y)) = (axes.get(axis_lx), axes.get(axis_ly)) {
                let stick_pos = Vec2::new(x, y);
        
                // Example: check if the stick is pushed up
                if stick_pos.length() > 0.9 && stick_pos.y > 0.5 {
                    movements.0.push(Movement::PushForward(push_factor * stick_pos.y.abs()))
                } 
                
                if stick_pos.length() > 0.9 && stick_pos.y < 0.5 {
                    movements.0.push(Movement::PushBackward(push_factor * stick_pos.y.abs()))
                }

                
                if stick_pos.length() > 0.9 && stick_pos.x < 0.5 {
                    movements.0.push(Movement::TurnRight(turn_factor * stick_pos.x.abs()))
                }
                
                if stick_pos.length() > 0.9 && stick_pos.x > 0.5 {
                    movements.0.push(Movement::TurnLeft(turn_factor * stick_pos.x.abs()))
                }
            }

            if button_inputs.just_pressed(GamepadButton(gamepad, GamepadButtonType::North)) {
                movements.0.clear();
                transform.rotation = Quat::from_rotation_x(0.0);
                transform.translation.y = 10.0;
            }
        }
}

pub fn apply_movement(
    mut car_query: Query<(
        &mut Movements,
        &Car,
        &GlobalTransform,
        &mut ExternalForce,
        &Velocity,
    )>,
) {
    if let Ok((mut movements, car, global_transform, mut rb_forces, rb_velocities)) =
        car_query.get_single_mut()
    {
        let mut forces = Vec3::new(0.0, 0.0, 0.0);
        let mut torques = Vec3::new(0.0, 0.0, 0.0);

        for movement in movements.0.iter() {
            forces += movement.as_lin_vec() * car.thrust;
            torques += movement.as_ang_vec() * car.thrust;
        }

        let local_to_global = global_transform.compute_matrix();
        forces = local_to_global.transform_vector3(forces);
        torques = local_to_global.transform_vector3(torques);

        let linvel: Vec3 = rb_velocities.linvel;
        forces -= linvel * car.drag;
        let angvel: Vec3 = rb_velocities.angvel;
        torques -= angvel * car.drag;

        rb_forces.force = forces;
        rb_forces.torque = torques;

        movements.0.clear();
    }
}