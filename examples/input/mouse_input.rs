use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

use bevy_research::setup_default_plugin;

fn main() {
    let mut app = App::new();

    setup_default_plugin(&mut app);

    app.add_system(detect_mouse_pressed)
        .add_system(detect_mouse_released)
        .add_system(detect_mouse_wheel)
        .add_system(print_mouse_position)
        .run();
}

/// マウスのボタンが押されたのを検知する
fn detect_mouse_pressed(mouse_buttons: Res<Input<MouseButton>>) {
    if mouse_buttons.just_pressed(MouseButton::Left) {
        info!("Left mouse button is pressed");
    }
    if mouse_buttons.just_pressed(MouseButton::Right) {
        info!("Right mouse button is pressed");
    }
}

/// マウスのボタンが離されたのを検知する
fn detect_mouse_released(mouse_buttons: Res<Input<MouseButton>>) {
    if mouse_buttons.just_released(MouseButton::Left) {
        info!("Left mouse button is released");
    }
    if mouse_buttons.just_released(MouseButton::Right) {
        info!("Right mouse button is released");
    }
}

/// マウスホイールの入力状態を出力する
fn detect_mouse_wheel(mut mouse_wheel_ev: EventReader<MouseWheel>) {
    for ev in mouse_wheel_ev.iter() {
        info!("unit: {:?}, x: {}, y: {}", ev.unit, ev.x, ev.y);
        // unit: Line, x: 1, y: 0
    }
}

/// カーソル移動時に座標を出力する
fn print_mouse_position(mut cursor_moved_ev: EventReader<CursorMoved>) {
    for ev in cursor_moved_ev.iter() {
        info!(
            "cursor position: (x: {:.1}, y: {:.1}),",
            ev.position.x, ev.position.y
        );
    }
}