use bevy::prelude::*;
use bevy_rapier3d::physics::{ColliderBundle, ColliderPositionSync, RigidBodyBundle};
use bevy_rapier3d::prelude::{ColliderMaterial, ColliderShape};

use bevy_research::bevy_rapier::setup_physics;
use bevy_research::pbr::{create_cube, create_plane};
use bevy_research::setup_3d;

fn main() {
    let mut app = App::new();
    setup_3d(&mut app);
    setup_physics(&mut app);

    app.add_startup_system(spawn_physics_plane)
        .add_startup_system(spawn_physics_cube);

    app.run();
}

fn spawn_physics_plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(50.0, 0.1, 50.0).into(),
        ..Default::default()
    };
    let plane = create_plane(&mut meshes, &mut materials);

    commands.spawn_bundle(collider).with_children(|p| {
        p.spawn_bundle(plane);
    });
}

fn spawn_physics_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let rb = RigidBodyBundle {
        position: Vec3::new(0.0, 25.0, 0.0).into(), // bevyとrapierでは座標が異なるのでinto()で変換
        ..Default::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(2.5, 2.5, 2.5).into(),
        material: ColliderMaterial {
            friction: 0.1,
            ..Default::default()
        }
        .into(),
        ..Default::default()
    };

    let cube = create_cube(&mut meshes, &mut materials);

    commands
        .spawn_bundle(cube)
        .insert_bundle(collider)
        .insert_bundle(rb)
        .insert(ColliderPositionSync::Discrete); // rapier側の座標をTransformコンポーネントに同期
}