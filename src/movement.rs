use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::car::Car;

#[derive(Debug)]
pub enum CarMovement {
    PushForward(f32),
    PushBackward(f32),
    TurnLeft(f32),
    TurnRight(f32),
}

impl CarMovement {
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
pub struct CarMovements(pub Vec<CarMovement>);

pub fn apply_movement(
    mut car_query: Query<(&mut CarMovements, &mut Car, &GlobalTransform, &mut ExternalForce, &Velocity)>,
) {
    if let Ok((mut car_movements, car, global_transform, mut rb_forces, rb_velocities)) =
        car_query.single_mut()
    {
        let mut forces = Vec3::new(0.0, 0.0, 0.0);
        let mut torques = Vec3::new(0.0, 0.0, 0.0);

        for car_movement in car_movements.0.iter() {
            forces += car_movement.as_lin_vec() * car.thrust;
            torques += car_movement.as_ang_vec() * car.thrust;
        }

        let local_to_global = global_transform.affine();
        forces = local_to_global.transform_vector3(forces);
        torques = local_to_global.transform_vector3(torques);

        let linvel: Vec3 = rb_velocities.linvel;
        
        // Apply normal drag - no special handling for upward movement
        forces -= linvel * car.drag;
        
        let angvel: Vec3 = rb_velocities.angvel;
        // Use higher angular drag on Y axis (turning axis) for better control
        torques -= Vec3::new(angvel.x * car.drag.x, angvel.y * car.drag.y * 0.8, angvel.z * car.drag.z);

        rb_forces.force = forces;
        rb_forces.torque = torques;

        car_movements.0.clear();
    }
}


pub fn _wrap_movement(mut _form_query: Query<(&Car, &mut Transform)>) {
    // if let Ok((_form, mut transform)) = form_query.get_single_mut() {
    //     let max_terrain_coord = 50.0;
    //     let min_terrain_coord = -50.0;
    //     let current_x = transform.translation.x;
    //     let current_z = transform.translation.z;

    //     // starting coords
    //     // -40.0, 20.0, 0.0

    //     if current_x > max_terrain_coord
    //         || current_z > max_terrain_coord
    //         || current_x < min_terrain_coord
    //         || current_z < min_terrain_coord
    //     {
    //         transform.translation = Vec3::new(0.0, 20.0, 0.0);
    //     }
    // }
}