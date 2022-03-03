use bevy::prelude::*;

use bevy_research::pbr::spawn_pbr;
use bevy_research::setup_3d;

fn main() {
    let mut app = App::new();
    setup_3d(&mut app);

    app.add_startup_system(spawn_pbr);

    app.run();
}