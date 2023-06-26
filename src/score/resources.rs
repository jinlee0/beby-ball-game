use bevy::prelude::*;

pub struct ScoreResourcePlugin;

impl Plugin for ScoreResourcePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>().init_resource::<HighScores>();
    }
}

#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
}

#[derive(Resource, Debug, Default)]
pub struct HighScores {
    pub scores: Vec<(String, u32)>,
}
