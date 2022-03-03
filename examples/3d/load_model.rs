use bevy::prelude::*;
use bevy_research::pbr::spawn_plane;
use bevy_research::setup_3d;
use std::ops::Mul;

fn main() {
    let mut app = App::new();

    setup_3d(&mut app);
    app.add_startup_system(spawn_plane)
        .add_startup_system(spawn_model)
        .add_system(rotate);

    app.run();
}

#[derive(Component)]
struct Model;

fn spawn_model(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon_add = asset_server.load("models/SM_Icon_Add_01.glb#Scene0");
    let icon_arrow = asset_server.load("models/SM_Icon_Arrow_01.glb#Scene0");

    for i in -2..=2 {
        commands
            .spawn_bundle((
                Transform::from_translation(Vec3::new(i as f32 * 10.0, 5.0, 0.0))
                    .with_scale(Vec3::ONE * 10.0),
                GlobalTransform::identity(),
                Model,
            ))
            .with_children(|p| {
                p.spawn_scene(icon_add.clone());
            });
    }

    for i in -2..=2 {
        commands
            .spawn_bundle((
                Transform::from_translation(Vec3::new(0.0, 2.5, i as f32 * 10.0))
                    .with_scale(Vec3::ONE * 10.0),
                GlobalTransform::identity(),
                Model,
            ))
            .with_children(|p| {
                p.spawn_scene(icon_arrow.clone());
            });
    }
}

fn rotate(time: Res<Time>, mut transform_query: Query<&mut Transform, With<Model>>) {
    for mut transform in transform_query.iter_mut() {
        transform.rotation = transform.rotation.mul(Quat::from_rotation_y(
            (45.0f32 * time.delta_seconds()).to_radians(),
        ));
    }
}