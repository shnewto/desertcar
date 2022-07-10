use crate::{state::GameState, movement::{get_movement, apply_movement}};
use bevy::prelude::*;


pub struct CarPlugin;

impl Plugin for CarPlugin { 
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Running)
            .with_system(get_movement.label("keys").label("movemet"))
            .with_system(apply_movement.after("movemet").label("apply_movement"))
        );
    }
}

#[derive(Default, Component, Debug)]
pub struct Car {
    pub thrust: Vec3,
    pub drag: Vec3,
}
