use bevy::prelude::*;

use crate::{game::SimulationState, global::events::GameOver, AppState};

use super::resources::*;

pub struct ScoreSystemPlugin;

impl Plugin for ScoreSystemPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HighScores>().add_system(insert_score.in_schedule(OnEnter(AppState::Game)))
            .add_system(remove_score.in_schedule(OnExit(AppState::Game)))
            .add_systems(
                (update_high_scores, high_scores_updated, update_score)
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            );
    }
}

fn insert_score(mut commands: Commands) {
    commands.insert_resource(Score::default());
}

fn remove_score(mut commands: Commands) {
    commands.remove_resource::<Score>();
}

fn update_high_scores(
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_scores: ResMut<HighScores>,
) {
    for event in game_over_event_reader.iter() {
        high_scores.scores.push(("Player".to_string(), event.score));
    }
}

fn high_scores_updated(high_scores: Res<HighScores>) {
    if high_scores.is_changed() {
        println!("High Scores: {:?}", high_scores);
    }
}

fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.val);
    }
}
