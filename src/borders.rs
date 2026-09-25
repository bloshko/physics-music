use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

const BORDER_THICKNESS: f32 = 6.;

pub struct BordersPlugin;

#[derive(Component)]
#[require(RigidBody::Fixed, Collider::cuboid(0.5, 0.5))]
pub struct Border;

fn setup(
    mut commands: Commands,
    window: Single<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = meshes.add(Rectangle::default());
    let material = materials.add(Color::WHITE);

    let window_size = window.resolution.size();
    let center = window_size / 2.;

    let offset = BORDER_THICKNESS * 1.5;

    let horizontal = Vec2::new(window_size.x, BORDER_THICKNESS);
    let vertical = Vec2::new(BORDER_THICKNESS, window_size.y);

    let borders = [
        (Vec2::new(center.x, window_size.y + offset), horizontal), // top
        (Vec2::new(center.x, -offset), horizontal),                // bottom
        (Vec2::new(-offset, center.y), vertical),                  // left
        (Vec2::new(window_size.x + offset, center.y), vertical),   // right
    ];

    for (position, size) in borders {
        commands.spawn((
            Border,
            Mesh2d(mesh.clone()),
            MeshMaterial2d(material.clone()),
            Transform::from_translation(position.extend(0.)).with_scale(size.extend(1.)),
        ));
    }
}

impl Plugin for BordersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
