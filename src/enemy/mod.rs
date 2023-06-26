use bevy::prelude::*;

use self::{resources::EnemyResourcePlugin, systems::EnemySystemPlugin};

pub mod components;
pub mod resources;
mod systems;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EnemyResourcePlugin)
            .add_plugin(EnemySystemPlugin);
    }
}
