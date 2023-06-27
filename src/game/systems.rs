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
        .add_system(toggle_simulation.run_if(in_state(AppState::Game)))
        .add_system(pause_simulation.in_schedule(OnEnter(AppState::Game)))
        .add_system(resume_simulation.in_schedule(OnExit(AppState::Game)));
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum MovementSystemSet {
    Movement,
    Update,
    Confinement,
}

fn pause_simulation(mut next_simulation_state: ResMut<NextState<SimulationState>>) {
    next_simulation_state.set(SimulationState::Paused)
}

fn resume_simulation(mut next_simulation_state: ResMut<NextState<SimulationState>>) {
    next_simulation_state.set(SimulationState::Running)
}

fn toggle_simulation(
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let toggled = match simulation_state.0 {
            SimulationState::Running => SimulationState::Paused,
            SimulationState::Paused => SimulationState::Running,
        };
        next_simulation_state.set(toggled.clone());
        println!("Game {toggled:?}");
    }
}
