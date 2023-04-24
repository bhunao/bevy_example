use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

pub const PLAYER_SIZE: f32 = 64.0;// player sprite size
pub const PLAYER_SPEED: f32 = 500.0;
pub const NUMBER_OF_ENEMIES: i32 = 10;
pub const ENEMY_SPEED: i32 = 100;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)          // all default plugins by bevy
        .add_startup_system(spawn_player)           // spawn player once at start
        .add_startup_system(spawn_camera)           // spawn camera once at start
        .add_startup_system(spawn_enemies)           // spawn camera once at start
        .add_system(player_movement)
        .add_system(confine_player_movement)
        .add_system(enemy_movement)
        .run();
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();    // get the window

    commands.spawn((        // `SpriteBundle` and `Player` structs inside the bundle
            SpriteBundle {
                transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0), // spawn in center
                texture: asset_server.load("sprites\\ball_blue_large.png"),     // load sprite image
                ..default()
            },
            Player {},
        ));
}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();    // get the window

    commands.spawn(     // bundle with onle the camera
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0 , window.height() / 2.0, 0.0),      // spawn in center
            ..default()
        }
    );
}


pub fn spawn_enemies (
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

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.0; //32.0
        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min= 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = player_transform.translation;

        if translation.x < x_min {
            translation.x = x_min
        } else if translation.x > x_max {
            translation.x = x_max
        }

        if translation.y < y_min {
            translation.y = y_min
        } else if translation.y > y_max {
            translation.y = y_max
        }

        player_transform.translation = translation;

    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction;
    }
}