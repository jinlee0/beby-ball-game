mod global;
mod consts;
mod enemy;
mod events;
mod player;
mod score;
mod star;
mod systems;

use enemy::EnemyPlugin;
use events::*;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(StarPlugin)
        .add_plugin(ScorePlugin)
        .add_event::<GameOver>()
        .add_startup_system(spawn_camera)
        .add_system(exit_game)
        .add_system(handle_game_over)
        .run();
}
