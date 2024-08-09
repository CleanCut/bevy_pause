/**
 * Press space to pause/unpause virtual time (time in the game world).
 *
 * A simple animation will run so that the effect of pausing can be observed.
 *
 * Run this example with `cargo run --example pause`.
 */
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_pause::PausePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PausePlugin::new(KeyCode::Space)) // Press space to pause/unpause
        .add_systems(Startup, startup)
        .add_systems(Update, animate)
        .run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Capsule2d::new(25.0, 50.0))),
        material: materials.add(Color::srgb(0.2, 0.2, 1.0)),
        ..default()
    });
}

fn animate(mut query: Query<&mut Transform, With<Mesh2dHandle>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_local_z(1.5 * time.delta_seconds());
        transform.translation = Vec3::new(
            100.0 * time.elapsed_seconds_wrapped().sin(),
            100.0 * time.elapsed_seconds_wrapped().cos(),
            0.,
        );
    }
}
