use bevy::prelude::*;

mod camera;
mod cursor_position;
mod scanner;
mod sound_object;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("Title"),
                resolution: (500., 300.).into(),
                decorations: false,
                focused: false,
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(camera::MainCameraPlugin)
        .add_plugins(cursor_position::CursorPositionPlugin)
        .add_plugins(ui::UIPlugin)
        .add_plugins(scanner::ScannerPlugin)
        .add_plugins(sound_object::SoundObjectPlugin)
        .run();
}
