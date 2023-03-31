use bevy::prelude::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup_cam)
    .add_startup_system(spawn_cubes)
    .run()
}

fn setup_cam(mut commands: Commands) {
    commands.spawn(Camera3dBundle::default());
}

fn spawn_cubes(mut commands: Commands, mut mesh_assets: ResMut<Assets<Mesh>>) {
    let mesh = mesh_assets.add(shape::Box::new(1., 1., 1.,).into());
    for x in -10..10 {
        for z in -10..10 {
            commands.spawn(PbrBundle {
                mesh: mesh.clone(),
                transform: Transform::from_translation(Vec3::new(x as f32 * 2.0, 0.0, z as f32 * 2.0)),
                ..Default::default()
            });
        }
    }
}