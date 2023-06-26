use bevy::prelude::Plugin;

use self::{resources::StarResourcePlugin, systems::StarSystemPlugin};

pub mod components;
pub mod resources;
mod systems;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(StarResourcePlugin)
            .add_plugin(StarSystemPlugin);
    }
}
