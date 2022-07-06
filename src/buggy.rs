use crate::state::GameState;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct BuggyPlugin;

impl Plugin for BuggyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Running)
                .with_system(get_movement.label("get_movement"))
                .with_system(apply_movement.after("get_movement").label("apply_movement"))
                .with_system(stop_roll.after("apply_movement"))
        );
    }
}

#[derive(Default, Component, Debug)]
pub struct Buggy {
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
            push_factor = 30.0;
        } else {
            push_factor = 21.0;
        }

        let turn_factor = 20.0;
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
    mut buggy_query: Query<(
        &mut Movements,
        &Buggy,
        &GlobalTransform,
        &mut ExternalForce,
        &Velocity,
    )>,

) {
    
    if let Ok((mut movements, buggy, global_transform, mut rb_forces, rb_velocities)) =
        buggy_query.get_single_mut()
    {
        let mut forces = Vec3::new(0.0, 0.0, 0.0);
        let mut torques = Vec3::new(0.0, 0.0, 0.0);

        for movement in movements.0.iter() {
            forces += movement.as_lin_vec() * buggy.thrust;
            torques += movement.as_ang_vec() * buggy.thrust;
        }

        let local_to_global = global_transform.compute_matrix();
        forces = local_to_global.transform_vector3(forces);
        torques = local_to_global.transform_vector3(torques);

        let linvel: Vec3 = rb_velocities.linvel;
        forces -= linvel * buggy.drag;
        let angvel: Vec3 = rb_velocities.angvel;
        torques -= angvel * buggy.drag;

        rb_forces.force = forces;
        rb_forces.torque = torques;

        movements.0.clear();
    }
}

pub fn _camera_follow(mut buggy_query: Query<(&Buggy, &Transform)>, mut camera_query: Query<(&Camera, &mut Transform)>) {
    if let (Ok((_buggy, buggy_transform)), Ok((_camera, mut camera_transform))) = (buggy_query.get_single_mut(), camera_query.get_single_mut()) {
        
        camera_transform.translation.x = buggy_transform.translation.x - 10.0;
        camera_transform.translation.y = buggy_transform.translation.y + 10.0;
        camera_transform.translation.z = buggy_transform.translation.z;
        let _ = camera_transform.looking_at(buggy_transform.translation, Vec3::Y);
    }
}


pub fn stop_roll(mut buggy_query: Query<(&Buggy, &mut Transform)>) {
    if let Ok((_buggy, mut transform)) = buggy_query.get_single_mut() {
        let max = 15.0f32.to_radians();
        let min = -15.0f32.to_radians();
        transform.rotation = Quat::from_vec4(Vec4::new(
            transform.rotation.x.clamp(min, max),
            transform.rotation.y,
            transform.rotation.z.clamp(min, max),
            transform.rotation.w,
        ))
    }
}

pub fn _wrap_movement(mut buggy_query: Query<(&Buggy, &mut Transform)>) {
    if let Ok((_buggy, mut transform)) = buggy_query.get_single_mut() {
        let max_terrain_coord = 50.0;
        let min_terrain_coord = -50.0;
        let current_x = transform.translation.x;
        let current_z = transform.translation.z;

        if current_x > max_terrain_coord
            || current_z > max_terrain_coord
            || current_x < min_terrain_coord
            || current_z < min_terrain_coord
        {
            transform.translation = Vec3::new(0.0, 20.0, 0.0);
        }
    }
}
