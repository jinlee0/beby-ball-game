use {self::systems::PlayerSystemPlugin, bevy::prelude::*};

pub mod components;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PlayerSystemPlugin);
    }
}
