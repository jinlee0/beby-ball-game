use bevy::prelude::*;

use self::systems::{interactions::MainMenuInteractionPlugin, layout::MainMenuLayoutPlugin};

mod components;
mod styles;
mod systems;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MainMenuLayoutPlugin)
            .add_plugin(MainMenuInteractionPlugin);
    }
}
