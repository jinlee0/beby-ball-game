use bevy::prelude::*;
use global::GlobalPlugin;

use crate::game::GamePlugin;
use crate::main_menu::MainMenuPlugin;

mod game;
mod global;
mod main_menu;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_plugin(GlobalPlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        .run();
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
