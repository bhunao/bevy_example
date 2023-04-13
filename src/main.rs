use bevy::{prelude::*, window::PrimaryWindow};


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)          // all default plugins by bevy
        .add_startup_system(spawn_player)           // spawn player once at start
        .add_startup_system(spawn_camera)           // spawn camera once at start
        .run();
}

#[derive(Component)]
pub struct Player {}

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

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec::ZERO;

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

        if direction.lenght() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.detal_seconds();
    };
}