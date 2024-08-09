use bevy::{prelude::*, window::PrimaryWindow};

pub struct MainCameraPlugin;

#[derive(Component)]
pub struct MainCamera;

fn setup(mut commands: Commands, q_window: Query<&Window, With<PrimaryWindow>>) {
    let window = q_window.single();

    info!(
        "Window width: {} height: {}",
        window.resolution.width(),
        window.resolution.height()
    );

    commands.spawn((
        Camera2dBundle {
            transform: Transform {
                translation: Vec3::new(
                    window.resolution.width() / 2.,
                    window.resolution.height() / 2.,
                    0.,
                ),
                ..default()
            },
            ..default()
        },
        MainCamera,
    ));
}

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
