use {
    crate::global::{events::GlobalEventRegistrationPlugin, systems::GlobalSystemPlugin},
    bevy::app::App,
    bevy::prelude::Plugin,
};

pub mod consts;
pub mod events;
pub mod systems;

pub struct GlobalPlugin;

impl Plugin for GlobalPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(GlobalEventRegistrationPlugin)
            .add_plugin(GlobalSystemPlugin);
    }
}
