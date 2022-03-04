use bevy::prelude::*;
use bevy_rapier3d::physics::{
    ColliderBundle, RapierConfiguration, RigidBodyBundle, RigidBodyPositionSync,
};
use bevy_rapier3d::prelude::*;

use bevy_research::bevy_rapier::{setup_physics, spawn_physics_plane};
use bevy_research::pbr::create_cube;
use bevy_research::setup_3d;

/// コライダーの範囲とPbrBundleの座標のずれを表示する
fn main() {
    let mut app = App::new();

    setup_3d(&mut app);
    setup_physics(&mut app);

    let rapier_cfg = RapierConfiguration {
        gravity: Vector::y() * -9.81 * 10.,
        ..Default::default()
    };

    app.insert_resource(rapier_cfg)
        .add_startup_system(setup)
        .add_startup_system(spawn_debug_cube);

    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    spawn_physics_plane(&mut commands, &mut meshes, &mut materials);
}

fn spawn_debug_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(2.5, 2.5, 2.5).into(),
        ..Default::default()
    };

    let rb = RigidBodyBundle {
        position: (Vec3::Y * 10.0).into(),
        ..Default::default()
    };

    commands
        .spawn_bundle(collider)
        .insert_bundle(rb)
        .insert(RigidBodyPositionSync::Discrete)
        .insert(ColliderDebugRender::from(Color::rgba(0., 0.5, 0., 0.1)))
        .with_children(|p| {
            let mut cube = create_cube(&mut meshes, &mut materials);
            cube.transform = Transform::from_translation(Vec3::Y * 2.5);
            cube.transform.scale = Vec3::splat(0.25);
            p.spawn_bundle(cube);
        });
}