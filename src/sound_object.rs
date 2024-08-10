use bevy::{color::palettes::css::GREEN, prelude::*, sprite::MaterialMesh2dBundle};

const RADIUS: f32 = 10.;

use crate::cursor_position;

pub struct SoundObjectPlugin;

#[derive(Component)]
pub struct SoundObject;

#[derive(Resource)]
struct SoundObjectHandles {
    mesh_handle: Handle<Mesh>,
    material_handle: Handle<ColorMaterial>,
}

fn spawn_on_click(
    mut commands: Commands,
    sound_object_handles: Res<SoundObjectHandles>,
    cursor_position: Res<cursor_position::CursorWorldPosition>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    let position = cursor_position.0;

    if buttons.just_pressed(MouseButton::Left) {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: sound_object_handles.mesh_handle.clone().into(),
                material: sound_object_handles.material_handle.clone(),
                transform: Transform::from_xyz(position.x, position.y, 0.),
                ..default()
            },
            SoundObject,
        ));
    };
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = meshes.add(Circle::new(RADIUS));
    let material = materials.add(Color::from(GREEN));

    commands.insert_resource(SoundObjectHandles {
        mesh_handle: mesh.clone(),
        material_handle: material.clone().into(),
    });
}

impl Plugin for SoundObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, spawn_on_click);
    }
}
