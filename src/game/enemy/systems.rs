use bevy::{prelude::*, window::PrimaryWindow};
use rand::{seq::SliceRandom, *};

use crate::{
    game::{systems::MovementSystemSet, SimulationState},
    global::consts::{ENEMY_SIZE, ENEMY_SPEED, NUMBER_OF_ENEMIES},
    AppState,
};

use super::{components::*, resources::*};

pub struct EnemySystemPlugin;

impl Plugin for EnemySystemPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_system(spawn_enemies.in_schedule(OnEnter(AppState::Game)))
            .add_system(despawn_all_enemies.in_schedule(OnExit(AppState::Game)))
            .add_systems(
                (
                    enemy_movement.in_set(MovementSystemSet::Movement),
                    update_enemy_direction.in_set(MovementSystemSet::Update),
                    confine_enemy_movement.in_set(MovementSystemSet::Confinement),
                    tick_enemy_spawn_timer,
                    spawn_enemies_over_time,
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            );
    }
}

fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }
}

fn enemy_movement(mut enemy_query: Query<(&mut Transform, &mut Enemy)>, time: Res<Time>) {
    for (mut transform, mut enemy) in enemy_query.iter_mut() {
        move_enemy(&time, &mut transform, &mut enemy);
    }
}

fn move_enemy(time: &Res<Time>, transform: &mut Mut<Transform>, enemy: &mut Mut<Enemy>) {
    let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
    transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
}

fn update_enemy_direction(
    window: Query<&Window, With<PrimaryWindow>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
) {
    let window = window.get_single().unwrap();
    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = x_min;
    let y_max = window.height() - half_enemy_size;
    for (transform, mut enemy) in enemy_query.iter_mut() {
        let origin_direction = enemy.direction;
        enemy.direction = {
            let mut d = enemy.direction;
            if transform.translation.x < x_min || transform.translation.x > x_max {
                d.x *= -1.0;
            }
            if transform.translation.y < y_min || transform.translation.y > y_max {
                d.y *= -1.0;
            }
            d
        };

        if origin_direction != enemy.direction {
            let sounds = vec![
                asset_server.load("audio/pluck_001.ogg"),
                asset_server.load("audio/pluck_002.ogg"),
            ];
            audio.play(sounds.choose(&mut thread_rng()).unwrap().to_owned());
        }
    }
}

fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = x_min;
    let y_max = window.height() - half_enemy_size;

    for mut transform in enemy_query.iter_mut() {
        transform.translation = {
            let mut translation = transform.translation;
            translation.x = translation.x.max(x_min).min(x_max);
            translation.y = translation.y.max(y_min).min(y_max);
            translation
        }
    }
}

fn tick_enemy_spawn_timer(mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    enemy_spawn_timer.timer.tick(time.delta());
}

fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
) {
    if enemy_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    random::<f32>() * window.width(),
                    random::<f32>() * window.height(),
                    0.0,
                ),
                texture: asset_server.load("/sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }
}

fn despawn_all_enemies(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>) {
    for enemy in enemy_query.iter() {
        commands.entity(enemy).despawn();
    }
}
