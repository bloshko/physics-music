use crate::camera;
use bevy::{prelude::*, window::PrimaryWindow};

pub struct CursorPositionPlugin;

#[derive(Resource, Default)]
pub struct CursorWorldPosition(pub Vec2);

fn setup(mut commands: Commands) {
    commands.init_resource::<CursorWorldPosition>();
}

fn update_cursor_coordinates(
    mut cursor_coords: ResMut<CursorWorldPosition>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<camera::MainCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        cursor_coords.0 = world_position;
    }
}

impl Plugin for CursorPositionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, update_cursor_coordinates);
    }
}
