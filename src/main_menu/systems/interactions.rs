use bevy::{app::AppExit, prelude::*};

use crate::{
    main_menu::{
        components::*,
        styles::{HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR},
    },
    AppState,
};

pub struct MainMenuInteractionPlugin;

impl Plugin for MainMenuInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (interactive_with_play_button, interactive_with_quit_button)
                .in_set(OnUpdate(AppState::MainMenu)),
        );
    }
}

macro_rules! interaction_query { // TODO move to utils
    ($w: ty, $s: ty) => {
        Query<
            (&Interaction, &mut $w),
            (Changed<Interaction>, With<$s>),
        >
    };
}

pub fn interactive_with_play_button(
    mut button_query: interaction_query!(BackgroundColor, PlayButton),
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                next_app_state.set(AppState::Game);
            }
            Interaction::Hovered => *background_color = HOVERED_BUTTON_COLOR.into(),
            Interaction::None => *background_color = NORMAL_BUTTON_COLOR.into(),
        }
    }
}

pub fn interactive_with_quit_button(
    mut button_query: interaction_query!(BackgroundColor, QuitButton),
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                app_exit_event_writer.send_default();
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}
