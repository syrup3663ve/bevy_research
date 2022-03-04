use bevy::prelude::*;
use bevy_rapier3d::physics::{ColliderBundle, ColliderPositionSync, RigidBodyBundle};
use bevy_rapier3d::prelude::{ColliderMaterial, ColliderShape};

use bevy_research::bevy_rapier::{setup_physics, spawn_physics_plane};
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