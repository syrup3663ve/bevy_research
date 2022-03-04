use crate::pbr::{create_cube, create_plane};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn setup_physics(app: &mut App) {
    app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierRenderPlugin);
}

pub fn spawn_physics_plane(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(50.0, 0.1, 50.0).into(),
        ..Default::default()
    };
    let plane = create_plane(meshes, materials);

    commands.spawn_bundle(collider).with_children(|p| {
        p.spawn_bundle(plane);
    });
}

#[allow(dead_code)]
fn spawn_physics_cube(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
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

    let cube = create_cube(meshes, materials);

    commands
        .spawn_bundle(cube)
        .insert_bundle(collider)
        .insert_bundle(rb)
        .insert(ColliderPositionSync::Discrete); // rapier側の座標をTransformコンポーネントに同期
}