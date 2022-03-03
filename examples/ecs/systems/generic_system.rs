use bevy::prelude::*;

use bevy_research::setup_default_plugin;

fn main() {
    let mut app = App::new();

    setup_default_plugin(&mut app);

    app.add_startup_system(setup)
        .add_system(clean_component::<CleanerA>)
        .add_system(clean_component::<CleanerB>);

    app.run();
}

trait Named {
    fn tell_name(&self);
}

#[derive(Component)]
struct CleanerA(String);

impl Named for CleanerA {
    fn tell_name(&self) {
        info!("I am cleanerA. name: {}", self.0);
    }
}

#[derive(Component)]
struct CleanerB(String);

impl Named for CleanerB {
    fn tell_name(&self) {
        info!("I am cleanerB. name: {}", self.0);
    }
}

fn setup(mut commands: Commands) {
    for _i in 0..2 {
        let cleaners = (CleanerA("A".to_string()), CleanerB("B".to_string()));
        commands.spawn_bundle(cleaners);
    }
}

fn clean_component<T: Component + Named>(mut commands: Commands, query: Query<(Entity, &T)>) {
    for (e, t) in query.iter() {
        t.tell_name();
        commands.entity(e).despawn_recursive();
    }
}