use {self::systems::ScoreSystemPlugin, bevy::prelude::Plugin};

pub mod resources;
mod systems;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(ScoreSystemPlugin);
    }
}
