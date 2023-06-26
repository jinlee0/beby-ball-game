use crate::consts::*;
use bevy::prelude::*;

pub struct StarResourcePlugin;

impl Plugin for StarResourcePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>();
    }
}

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> Self {
        StarSpawnTimer {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}
