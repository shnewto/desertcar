use bevy::prelude::*;

use crate::{camera, lighting, scene, state::GameState};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Setup),
            (
                scene::setup,
                lighting::setup.after(scene::setup),
            ),
        )
        .add_systems(Update, complete.run_if(in_state(GameState::Setup)))
        .add_systems(OnEnter(GameState::Running), camera::setup);
    }
}

pub fn complete(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Running);
}
