use bevy::prelude::*;

use crate::{
    main_menu::{
        components::{MainMenu, PlayButton, QuitButton},
        styles::{
            get_button_text_style, get_title_text_style, BUTTON_STYLE, IMAGE_STYLE,
            MAIN_MENU_STYLE, NORMAL_BUTTON_COLOR, TITLE_STYLE,
        },
    },
    AppState,
};

pub struct MainMenuLayoutPlugin;

impl Plugin for MainMenuLayoutPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(despawn_main_menu.in_schedule(OnExit(AppState::MainMenu)));
    }
}

fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_main_menu(&mut commands, &asset_server);
}

fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

macro_rules! build_button {
    ($parent: expr, $asset_server: expr, $param: expr, $component: expr) => {
        $parent
            .spawn((
                ButtonBundle {
                    style: BUTTON_STYLE,
                    background_color: NORMAL_BUTTON_COLOR.into(),
                    ..default()
                },
                $component,
            ))
            .with_children(|parent| {
                parent.spawn((TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            $param.name,
                            get_button_text_style($asset_server),
                        )],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                },));
            });
    };
}

fn build_main_menu(commads: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    commads
        .spawn((
            NodeBundle {
                style: MAIN_MENU_STYLE,
                ..default()
            },
            MainMenu {},
        ))
        .with_children(|parent| {
            build_title(parent, asset_server);
            build_button!(
                parent,
                asset_server,
                ButtonParam { name: "Play" },
                PlayButton {}
            );
            build_button!(
                parent,
                asset_server,
                ButtonParam { name: "Quit" },
                QuitButton {}
            );
        })
        .id()
}

struct ButtonParam {
    name: &'static str,
}

fn build_title(parent: &mut ChildBuilder<'_, '_, '_>, asset_server: &Res<'_, AssetServer>) {
    parent
        .spawn((NodeBundle {
            style: TITLE_STYLE,
            ..Default::default()
        },))
        .with_children(|parent| {
            build_title_image(
                parent,
                asset_server.load("sprites/ball_blue_large.png").into(),
            );
            build_title_text(parent, asset_server);
            build_title_image(
                parent,
                asset_server.load("sprites/ball_red_large.png").into(),
            );
        });
}

fn build_title_text(parent: &mut ChildBuilder<'_, '_, '_>, asset_server: &Res<'_, AssetServer>) {
    parent.spawn(TextBundle {
        text: Text {
            sections: vec![TextSection::new(
                "Bevy Ball Game",
                get_title_text_style(asset_server),
            )],
            alignment: TextAlignment::Center,
            ..default()
        },
        ..default()
    });
}

fn build_title_image(parent: &mut ChildBuilder, image: UiImage) {
    parent.spawn(ImageBundle {
        style: IMAGE_STYLE,
        image,
        ..default()
    });
}
