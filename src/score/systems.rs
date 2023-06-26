use crate::events::GameOver;

use super::resources::*;
use bevy::prelude::*;
pub struct ScoreSystemPlugin;

impl Plugin for ScoreSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_high_scores)
            .add_system(high_scores_updated)
            .add_system(update_score);
    }
}

pub fn update_high_scores(
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_scores: ResMut<HighScores>,
) {
    for event in game_over_event_reader.iter() {
        high_scores.scores.push(("Player".to_string(), event.score));
    }
}

pub fn high_scores_updated(high_scores: Res<HighScores>) {
    if high_scores.is_changed() {
        println!("High Scores: {:?}", high_scores);
    }
}

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value);
    }
}
