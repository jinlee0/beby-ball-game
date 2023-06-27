use crate::{game::SimulationState, AppState};
use bevy::prelude::*;

pub struct GameSystemPlugin;

impl Plugin for GameSystemPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            (
                MovementSystemSet::Movement,
                MovementSystemSet::Update,
                MovementSystemSet::Confinement,
            )
                .chain(),
        )
        .add_system(toggle_simulation.run_if(in_state(AppState::Game)));
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum MovementSystemSet {
    Movement,
    Update,
    Confinement,
}

fn toggle_simulation(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let toggled = match simulation_state.0 {
            SimulationState::Running => SimulationState::Paused,
            SimulationState::Paused => SimulationState::Running,
        };
        commands.insert_resource(NextState(Some(toggled.clone())));
        println!("Game {toggled:?}");
    }
}
