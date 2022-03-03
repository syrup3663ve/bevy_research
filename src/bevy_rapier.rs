use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn setup_physics(app: &mut App) {
    app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default());
}