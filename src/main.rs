use {
    crate::global::GlobalPlugin, bevy::prelude::*, enemy::EnemyPlugin, player::PlayerPlugin,
    score::ScorePlugin, star::StarPlugin,
};

mod enemy;
mod global;
mod player;
mod score;
mod star;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GlobalPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(StarPlugin)
        .add_plugin(ScorePlugin)
        .run();
}
