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
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::G) && app_state.0 != AppState::Game {
        commands.insert_resource(NextState(Some(AppState::Game)));
        println!("Entered AppState::Game");
    }
}

fn transition_to_main_menu_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::M) && app_state.0 != AppState::MainMenu {
        commands.insert_resource(NextState(Some(AppState::MainMenu)));
        commands.insert_resource(NextState(Some(SimulationState::Paused)));
        println!("Entered AppState::MainMenu");
    }
}
