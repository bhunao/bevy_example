use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

pub const PLAYER_SIZE: f32 = 64.0;// player sprite size
pub const PLAYER_SPEED: f32 = 500.0;
pub const NUMBER_OF_ENEMIES: i32 = 3;
pub const ENEMY_SPEED: f32 = 200.0;
pub const START_COUNT: usize = 10;
pub const STAR_SIZE: f32 = 10.0;
pub const STAR_SPAWN_TIME: f32 = 1.0;


fn main() {
    App::new()
        // plugins
        .add_plugins(DefaultPlugins)          // all default plugins by bevy
        // resources
        .init_resource::<Score>()
        .init_resource::<StarSpawnTimer>()
        // startup systems
        .add_startup_system(spawn_player)           // spawn player once at start
        .add_startup_system(spawn_camera)           // spawn camera once at start
        .add_startup_system(spawn_enemies)           // spawn enemies once at start
        .add_startup_system(spawn_stars)           // spawn enemies once at start
        // systems
        .add_system(player_movement)
        .add_system(confine_player_movement)
        .add_system(enemy_movement)
        .add_system(update_enemy_direction)
        .add_system(confine_enemy_movement)
        .add_system(enemy_hit_player)
        .add_system(player_hit_star)
        .add_system(udpate_score)
        .add_system(respawn_stars)
        .add_system(update_star_tick)
        .run();
}

#[derive(Component)]
pub struct Star {}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

#[derive(Resource)]
pub struct Score {
    pub value: u32
}

impl Default for Score {
    fn default() -> Score {
        Score { value: 0}
    }
}

impl Default for StarSpawnTimer {
    fn default() -> StarSpawnTimer {
        StarSpawnTimer {
             timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
            }
    }
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


pub fn spawn_stars ( mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>, asset_server: Res<AssetServer>,) {
    let window = window_query.get_single().unwrap();
    for _ in 0..START_COUNT {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(random_x, random_y, 0.0),
                    texture: asset_server.load("sprites/star.png"),
                    ..default()
                },
                Star {},
            ));
    }
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
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn update_enemy_direction(
    mut enemies_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let half_player_size = PLAYER_SIZE / 2.0; // 32.0
    let x_min = 0.0 + half_player_size;
    let x_max = window.width() - half_player_size;
    let y_min= 0.0 + half_player_size;
    let y_max = window.height() - half_player_size;

    for (enemy_transform, mut enemy) in enemies_query.iter_mut() {
        let translation = enemy_transform.translation;
        let mut direction_changed = false;

        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.0;
            direction_changed = true;
        }
        if translation.y < y_min || translation.y > y_max  {
            enemy.direction.y *= -1.0;
            direction_changed = true;
        }

        if direction_changed {
            let sound_effect = if random::<f32>() > 0.5 {
                asset_server.load("audio/pluck_001.ogg")
            } else {
                asset_server.load("audio/pluck_002.ogg")
            };
            audio.play(sound_effect);
        }
    }
}



pub fn confine_enemy_movement(
    mut enemies_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let half_player_size = PLAYER_SIZE / 2.0; // 32.0
    let x_min = 0.0 + half_player_size;
    let x_max = window.width() - half_player_size;
    let y_min= 0.0 + half_player_size;
    let y_max = window.height() - half_player_size;

    for mut enemy_transform in enemies_query.iter_mut() {
        let mut translation = enemy_transform.translation;

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
        enemy_transform.translation = translation;
    }
}


pub fn enemy_hit_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    ) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = PLAYER_SIZE / 2.0;

            if distance < player_radius + enemy_radius {
                println!("explode");
                let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
                audio.play(sound_effect);
                commands.entity(player_entity).despawn();
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
    mut score: ResMut<Score>,) {
        if let Ok(player_transform) = player_query.get_single() {
            for (star_entity, start_transform) in star_query.iter() {
                let distance = player_transform
                .translation.distance(start_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let star_radius = STAR_SIZE / 2.0;

            if distance < player_radius + star_radius {
                println!("star!");
                score.value += 1;
                let sound_effect = asset_server.load("audio/impactMining_004.ogg");
                audio.play(sound_effect);
                commands.entity(star_entity).despawn();
            }
        }
}
     }


pub fn udpate_score (score: Res<Score>) {
    if score.is_changed() {
        println!("score: {}", score.value.to_string());
    }
}

pub fn update_star_tick(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn respawn_stars(
        mut commands: Commands,
        window_query: Query<&Window, With<PrimaryWindow>>,
        star_query: Query<(Entity, &Transform), With<Star>>,
        asset_server: Res<AssetServer>,
        star_spawn_timer: ResMut<StarSpawnTimer>,
    ) {


    let window = window_query.get_single().unwrap();
    if star_spawn_timer.timer.finished() && star_query.iter().count() < START_COUNT {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(random_x, random_y, 0.0),
                    texture: asset_server.load("sprites/star.png"),
                    ..default()
                },
                Star {},
            ));
    }
}
