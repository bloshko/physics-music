use bevy::{color::palettes::basic::PURPLE, prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

const SCANNER_SPEED: f32 = 200.;
const SCANNER_WIDTH: f32 = 10.;

pub struct ScannerPlugin;

#[derive(Component)]
#[require(Sensor, Collider::cuboid(0.5, 0.5))]
pub struct Scanner {
    start_x: f32,
    end_x: f32,
}

fn setup(
    mut commands: Commands,
    window: Single<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window_size = window.resolution.size();

    commands.spawn((
        Scanner {
            start_x: 0.,
            end_x: window_size.x,
        },
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::from(PURPLE))),
        Transform::from_xyz(0., window_size.y / 2., 0.).with_scale(Vec3::new(
            SCANNER_WIDTH,
            window_size.y,
            1.,
        )),
    ));
}

fn scan_from_left_to_right(scanner: Single<(&mut Transform, &Scanner)>, time: Res<Time>) {
    let (mut transform, scanner) = scanner.into_inner();

    if transform.translation.x > scanner.end_x {
        transform.translation.x = scanner.start_x;
    } else {
        transform.translation.x += SCANNER_SPEED * time.delta_secs();
    }
}

impl Plugin for ScannerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, scan_from_left_to_right);
    }
}
