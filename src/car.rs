use crate::{camera::look_and_orbit, input::get_car_movement, movement::apply_movement, state::GameState};
use bevy::prelude::*;

pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                get_car_movement,
                apply_movement.after(get_car_movement),
                look_and_orbit.after(apply_movement),
            )
                .run_if(in_state(GameState::Running)),
        );
    }
}

#[derive(Default, Component, Debug)]
pub struct Car {
    pub thrust: Vec3,
    pub drag: Vec3,
}
