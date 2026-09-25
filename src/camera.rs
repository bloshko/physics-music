use bevy::{prelude::*, window::PrimaryWindow};

pub struct MainCameraPlugin;

#[derive(Component)]
pub struct MainCamera;

fn setup(mut commands: Commands, window: Single<&Window, With<PrimaryWindow>>) {
    let window_size = window.resolution.size();

    info!("Window width: {} height: {}", window_size.x, window_size.y);

    commands.spawn((
        MainCamera,
        Camera2d,
        Transform::from_translation((window_size / 2.).extend(0.)),
    ));
}

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
