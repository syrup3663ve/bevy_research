use bevy::prelude::*;

use bevy_research::pbr::spawn_plane;
use bevy_research::setup_3d;

fn main() {
    let mut app = App::new();
    setup_3d(&mut app);

    app.add_startup_system(setup_plane);

    app.run();
}

fn setup_plane(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    spawn_plane(commands, meshes, materials);
}