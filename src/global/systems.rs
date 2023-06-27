use crate::{game::SimulationState, AppState};
use bevy::{app::AppExit, prelude::*, window::PrimaryWindow};

use crate::global::events::GameOver;

pub struct GlobalSystemPlugin;

impl Plugin for GlobalSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera)
            .add_system(exit_game)
            .add_system(handle_game_over)
            .add_system(transition_to_game_state)
            .add_system(transition_to_main_menu_state);
    }
}

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

fn exit_game(keyboard_input: Res<Input<KeyCode>>, mut app_exit_event_writer: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

fn handle_game_over(mut game_over_event_reader: EventReader<GameOver>) {
    for event in game_over_event_reader.iter() {
        println!("Your final score is: {}", event.score);
    }
}

fn transition_to_game_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::G) && app_state.0 != AppState::Game {
        next_app_state.set(AppState::Game);
        println!("Entered AppState::Game");
    }
}

fn transition_to_main_menu_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::M) && app_state.0 != AppState::MainMenu {
        next_app_state.set(AppState::MainMenu);
        next_simulation_state.set(SimulationState::Paused);
        println!("Entered AppState::MainMenu");
    }
}
