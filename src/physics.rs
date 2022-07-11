use bevy::{math::Vec3, prelude::ResMut};
use bevy_rapier3d::plugin::RapierConfiguration;

pub fn setup(mut physics_config: ResMut<RapierConfiguration>) {
    physics_config.gravity = Vec3::ZERO;
    physics_config.gravity.y = -600.0;
}
