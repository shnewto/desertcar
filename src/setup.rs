use bevy::prelude::*;

use crate::{camera, lighting, physics, scene, state::GameState};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Setup)
                .with_system(scene::setup.label("scene"))
                .with_system(lighting::setup.after("scene").label("lighting"))
                .with_system(physics::setup.after("lighting").label("physics")),
        )
        .add_system_set(SystemSet::on_update(GameState::Setup).with_system(complete))
        .add_system_set(
            SystemSet::on_enter(GameState::Running).with_system(camera::setup.label("camera")),
        );
    }
}

pub fn complete(mut state: ResMut<State<GameState>>) {
    state.set(GameState::Running).unwrap();
}
