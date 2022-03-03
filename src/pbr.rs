use bevy::prelude::shape::Cube;
use bevy::prelude::*;

use crate::shape::Plane;

pub fn spawn_pbr(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(create_cube(&mut meshes, &mut materials));
}

pub fn create_cube(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> PbrBundle {
    PbrBundle {
        mesh: meshes.add(Mesh::from(Cube::new(5.0))),
        material: materials.add(Color::rgb(0.2, 0.2, 0.2).into()),
        ..Default::default()
    }
}

pub fn spawn_plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(create_plane(&mut meshes, &mut materials));
}

pub fn create_plane(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> PbrBundle {
    PbrBundle {
        mesh: meshes.add(Mesh::from(Plane { size: 50.0 })),
        material: materials.add(Color::rgb(0.5, 0.5, 0.1).into()),
        ..Default::default()
    }
}