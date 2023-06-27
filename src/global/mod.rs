use {bevy::app::App, bevy::prelude::Plugin};

use crate::global::events::GlobalEventRegistrationPlugin;
use crate::global::systems::GlobalSystemPlugin;

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
