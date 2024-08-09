use bevy::{
    color::palettes::basic::PURPLE, prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow,
};

use crate::cursor_position::CursorWorldPosition;

pub struct ScannerPlugin;

#[allow(dead_code)]
struct Boundaries {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

#[derive(Component)]
struct Scanner {
    scanning_boundaries: Boundaries,
}

#[allow(dead_code)]
fn move_scanner_with_cursor(
    mut q_scanner_transform: Query<&mut Transform, With<Scanner>>,
    cursor_world_position: Res<CursorWorldPosition>,
) {
    let mut scanner_transform = q_scanner_transform.single_mut();

    scanner_transform.translation = cursor_world_position.0.clone().extend(0.);
}

fn scan_from_left_to_right(mut q_scanner: Query<(&mut Transform, &Scanner)>) {
    let (mut transform, scanner) = q_scanner.single_mut();

    if transform.translation.x > scanner.scanning_boundaries.right {
        transform.translation.x = 0.;
    } else {
        transform.translation.x = transform.translation.x + 1.;
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
) {
    let mesh = meshes.add(Rectangle::default());
    let material = materials.add(Color::from(PURPLE));
    let window = q_window.single();

    let scanner_height = window.resolution.height();
    let scanning_boundaries = Boundaries {
        left: 0.,
        right: window.resolution.width(),
        top: scanner_height,
        bottom: 0.,
    };

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: mesh.into(),
            material,
            transform: Transform {
                translation: Vec3::new(0., scanner_height / 2., 0.),
                scale: Vec3::new(10., scanner_height, 0.),
                ..default()
            },
            ..default()
        },
        Scanner {
            scanning_boundaries,
        },
    ));
}

impl Plugin for ScannerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, scan_from_left_to_right);
    }
}
