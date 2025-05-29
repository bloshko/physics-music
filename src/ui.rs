use crate::cursor_position;
use bevy::prelude::*;

pub struct UIPlugin;

#[derive(Component)]
struct Label;

fn setup_ui(mut commands: Commands) {
    commands.spawn((
        Text("x: 0 y: 0".to_string()),
        TextFont::from_font_size(15.0),
        Label,
    ));
}

fn update_ui(
    text_query: Query<Entity, With<Label>>,
    mut writer: TextUiWriter,
    cursor_coords: Res<cursor_position::CursorWorldPosition>,
) {
    let text_component = text_query.single().unwrap();

    *writer.text(text_component, 0) = format!("x: {}, y: {}", cursor_coords.0.x, cursor_coords.0.y);
}

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
            .add_systems(Update, update_ui);
    }
}
