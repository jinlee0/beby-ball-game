use {
    super::components::*,
    bevy::{prelude::*, window::PrimaryWindow},
};

use crate::game::{enemy::components::Enemy, systems::MovementSystemSet};
use crate::game::{star::components::Star, SimulationState};
use crate::global::consts::{ENEMY_SIZE, PLAYER_SIZE, PLAYER_SPEED, STAR_SIZE};
use crate::global::events::GameOver;
use crate::{game::score::resources::Score, AppState};

pub struct PlayerSystemPlugin;

impl Plugin for PlayerSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
            .add_system(despawn_player.in_schedule(OnExit(AppState::Game)))
            .add_systems(
                (
                    player_movement.in_set(MovementSystemSet::Movement),
                    confine_player_movement.in_set(MovementSystemSet::Confinement),
                    player_hit_star,
                    enemy_hit_player,
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            );
    }
}

fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player {},
    ));
}

fn despawn_player(mut commands: Commands, player_entity_query: Query<Entity, With<Player>>) {
    if let Ok(player_entity) = player_entity_query.get_single() {
        commands.entity(player_entity).despawn();
    }
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        macro_rules! handle_key_input {
            ($fun: expr, $($code: expr),*) => {
                for c in [$($code),*] {
                    if keyboard_input.pressed(c) {
                        $fun();
                        break;
                    }
                }
            };
        }

        handle_key_input!(
            || direction += Vec3::new(-1.0, 0.0, 0.0),
            KeyCode::Left,
            KeyCode::A
        );
        handle_key_input!(
            || direction += Vec3::new(1.0, 0.0, 0.0),
            KeyCode::Right,
            KeyCode::D
        );
        handle_key_input!(
            || direction += Vec3::new(0.0, 1.0, 0.0),
            KeyCode::Up,
            KeyCode::W
        );
        handle_key_input!(
            || direction += Vec3::new(0.0, -1.0, 0.0),
            KeyCode::Down,
            KeyCode::S
        );

        direction = direction.normalize_or_zero();

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();
        let half_player_size = PLAYER_SIZE / 2.0;
        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = x_min;
        let y_max = window.height() - half_player_size;

        player_transform.translation = {
            let mut translation = player_transform.translation;
            translation.x = translation.x.max(x_min).min(x_max);
            translation.y = translation.y.max(y_min).min(y_max);
            translation
        }
    }
}

fn enemy_hit_player(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    score: Res<Score>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;
            if distance < player_radius + enemy_radius {
                let sound = asset_server.load("audio/explosionCrunch_000.ogg");
                audio.play(sound);
                commands.entity(player_entity).despawn();
                game_over_event_writer.send(GameOver { score: score.val });
            }
        }
    }
}

fn player_hit_star(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (star_entity, star_transform) in star_query.iter() {
            let distance = player_transform
                .translation
                .distance(star_transform.translation);

            if distance < PLAYER_SIZE / 2.0 + STAR_SIZE / 2.0 {
                score.val += 1;
                let sound = asset_server.load("audio/laserLarge_000.ogg");
                audio.play(sound);
                commands.entity(star_entity).despawn();
            }
        }
    }
}
