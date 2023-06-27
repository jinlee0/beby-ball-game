use bevy::app::App;
use bevy::prelude::*;

use self::enemy::EnemyPlugin;
use self::player::PlayerPlugin;
use self::score::ScorePlugin;
use self::star::StarPlugin;
use self::systems::*;

pub mod enemy;
pub mod player;
pub mod score;
pub mod star;
pub mod systems;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            .add_plugin(GameSystemPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(StarPlugin)
            .add_plugin(ScorePlugin);
    }
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}
