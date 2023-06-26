use bevy::{app::App, prelude::Plugin};

pub struct GlobalEventRegistrationPlugin;

impl Plugin for GlobalEventRegistrationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameOver>();
    }
}

pub struct GameOver {
    pub score: u32,
}
