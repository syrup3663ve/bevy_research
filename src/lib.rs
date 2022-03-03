pub mod bevy_rapier;
pub mod pbr;

use bevy::prelude::*;

pub fn setup_default_plugin(app: &mut App) {
    app.add_plugins(DefaultPlugins);
}

pub fn setup_2d(app: &mut App) {
    app.add_plugins(DefaultPlugins)
        .add_startup_system(setup_2d_cameras);
}

fn setup_2d_cameras(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

pub fn setup_3d(app: &mut App) {
    app.add_plugins(DefaultPlugins)
        .add_startup_system(setup_3d_cameras)
        .add_startup_system(setup_directional_light);
}

fn setup_3d_cameras(mut commands: Commands) {
    let cam = PerspectiveCameraBundle {
        transform: Transform::from_xyz(50.0, 50.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    };
    commands.spawn_bundle(cam);
}

fn setup_directional_light(mut commands: Commands) {
    commands.spawn_bundle(DirectionalLightBundle::default());
}