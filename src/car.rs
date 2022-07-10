use crate::state::GameState;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Running)
                .with_system(get_movement.label("get_movement"))
                .with_system(apply_movement.after("get_movement").label("apply_movement"))
        );
    }
}

#[derive(Default, Component, Debug)]
pub struct Car {
    pub thrust: Vec3,
    pub drag: Vec3,
}

#[derive(Debug)]
pub enum Movement {
    PushForward(f32),
    PushBackward(f32),
    PushLeft(f32),
    PushRight(f32),
    TurnLeft(f32),
    TurnRight(f32),
}

impl Movement {
    pub fn as_lin_vec(&self) -> Vec3 {
        match self {
            Self::PushForward(p) => Vec3::new(*p, 0.0, 0.0),
            Self::PushBackward(p) => Vec3::new(-*p, 0.0, 0.0),
            Self::PushLeft(p) => Vec3::new(0.0, 0.0, -*p),
            Self::PushRight(p) => Vec3::new(0.0, 0.0, *p),
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

pub fn get_movement(mut query: Query<(&mut Movements, &mut Transform)>, keys: Res<Input<KeyCode>>) {
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
        if keys.pressed(KeyCode::A) {
            movements.0.push(Movement::PushLeft(push_factor))
        }
        if keys.pressed(KeyCode::D) {
            movements.0.push(Movement::PushRight(push_factor))
        }

        if keys.pressed(KeyCode::Left) {
            movements.0.push(Movement::TurnLeft(turn_factor))
        }
        if keys.pressed(KeyCode::Right) {
            movements.0.push(Movement::TurnRight(turn_factor))
        }

        if keys.pressed(KeyCode::Key1) {
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

pub fn _wrap_movement(mut car_query: Query<(&Car, &mut Transform)>) {
    if let Ok((_car, mut transform)) = car_query.get_single_mut() {
        let max_desert_coord = 50.0;
        let min_desert_coord = -50.0;
        let current_x = transform.translation.x;
        let current_z = transform.translation.z;

        if current_x > max_desert_coord
            || current_z > max_desert_coord
            || current_x < min_desert_coord
            || current_z < min_desert_coord
        {
            transform.translation = Vec3::new(0.0, 20.0, 0.0);
        }
    }
}
