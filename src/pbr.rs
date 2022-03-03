use bevy::prelude::shape::Cube;
use bevy::prelude::*;

pub fn spawn_pbr(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(Cube::new(5.0))),
        material: materials.add(Color::rgb(0.2, 0.2, 0.2).into()),
        ..Default::default()
    });
}