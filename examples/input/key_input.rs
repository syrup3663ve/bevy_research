use bevy::input::keyboard::KeyboardInput;
use bevy::input::ElementState;
use bevy::prelude::*;

use bevy_research::setup_default_plugin;

fn main() {
    let mut app = App::new();

    setup_default_plugin(&mut app);

    app.add_system(detect_any_wasd_keys)
        .add_system(detect_all_keys)
        .run();
}

/// 任意の指定したキー入力に対しての出力する
fn detect_any_wasd_keys(keys: Res<Input<KeyCode>>) {
    if keys.any_just_pressed([KeyCode::W, KeyCode::A, KeyCode::S, KeyCode::D]) {
        info!("Any WASD key is pressed");
    }
}

/// すべてのキー入力を検知する
fn detect_all_keys(mut key_ev: EventReader<KeyboardInput>) {
    for ev in key_ev.iter() {
        match ev.state {
            ElementState::Pressed => {
                info!(
                    "key: {:?} is pressed. scan_code: {}",
                    ev.key_code, ev.scan_code
                );
            }
            ElementState::Released => {
                info!(
                    "key: {:?} is released. scan_code: {}",
                    ev.key_code, ev.scan_code
                );
            }
        };
    }
}