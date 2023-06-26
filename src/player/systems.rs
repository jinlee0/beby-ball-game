use {
    super::components::*,
    crate::{
        enemy::components::Enemy,
        global::{consts::*, events::GameOver},
        score::resources::*,
        star::components::*,
    },
    bevy::{prelude::*, window::PrimaryWindow},
};

pub struct PlayerSystemPlugin;

impl Plugin for PlayerSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_movement)
            .add_system(confine_player_movement)
            .add_system(player_hit_star)
            .add_system(enemy_hit_player);
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

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        macro_rules! hadle_key_input {
            ($st: expr, $($code: expr),*) => {
                for con in [$($code),*] {
                    if keyboard_input.pressed(con) {
                        $st();
                        break;
                    }
                }
            };
        }

        hadle_key_input!(
            || direction += Vec3::new(-1.0, 0.0, 0.0),
            KeyCode::Left,
            KeyCode::A
        );
        hadle_key_input!(
            || direction += Vec3::new(1.0, 0.0, 0.0),
            KeyCode::Right,
            KeyCode::D
        );
        hadle_key_input!(
            || direction += Vec3::new(0.0, 1.0, 0.0),
            KeyCode::Up,
            KeyCode::W
        );
        hadle_key_input!(
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

pub fn enemy_hit_player(
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

pub fn player_hit_star(
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
