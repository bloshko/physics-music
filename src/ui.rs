use crate::cursor_position;
use bevy::prelude::*;

pub struct UIPlugin;

#[derive(Component)]
struct Label;

fn setup_ui(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "x: 0 y: 0",
            TextStyle {
                font_size: 15.0,
                ..default()
            },
        ),
        Label,
    ));
}

fn update_ui(
    mut text_query: Query<&mut Text, With<Label>>,
    cursor_coords: Res<cursor_position::CursorWorldPosition>,
) {
    let mut text_component = text_query.single_mut();

    text_component.sections[0].value =
        format!("x: {}, y: {}", cursor_coords.0.x, cursor_coords.0.y);
}

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
            .add_systems(Update, update_ui);
    }
}
