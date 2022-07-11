use crate::{camera::look_and_orbit, input::get_car_movement, movement::apply_movement, state::GameState};
use bevy::prelude::*;

pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Running)
                .with_system(get_car_movement.label("keys").label("movement"))
                .with_system(apply_movement.after("movement").label("apply_movement"))
                .with_system(look_and_orbit.label("look_and_orbit").after("apply_movement")),
        );
    }
}

#[derive(Default, Component, Debug)]
pub struct Car {
    pub thrust: Vec3,
    pub drag: Vec3,
}
