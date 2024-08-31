use bevy::{
    color::palettes::css::WHITE, prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow,
};
use bevy_rapier2d::prelude::*;

const BORDER_THICKNESS: f32 = 6.;

pub struct BordersPlugin;

#[derive(Component)]
pub struct Border;

fn setup(
    mut commands: Commands,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = meshes.add(Rectangle::default());
    let material = materials.add(Color::from(WHITE));
    let window = q_window.single();
    let window_size = window.resolution.size();

    let top_border_transform = Transform {
        translation: Vec3::new(
            window_size.x / 2.,
            window_size.y + BORDER_THICKNESS * 1.5,
            0.,
        ),
        scale: Vec3::new(window_size.x, BORDER_THICKNESS, 0.),
        ..default()
    };

    let bottom_border_transform = Transform {
        translation: Vec3::new(window_size.x / 2., BORDER_THICKNESS * -1.5, 0.),
        scale: Vec3::new(window_size.x, BORDER_THICKNESS, 0.),
        ..default()
    };

    let left_border_transform = Transform {
        translation: Vec3::new(BORDER_THICKNESS * -1.5, window_size.y / 2., 0.),
        scale: Vec3::new(BORDER_THICKNESS, window_size.y, 0.),
        ..default()
    };

    let right_border_transform = Transform {
        translation: Vec3::new(
            window_size.x + BORDER_THICKNESS * 1.5,
            window_size.y / 2.,
            0.,
        ),
        scale: Vec3::new(BORDER_THICKNESS, window_size.y, 0.),
        ..default()
    };

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: mesh.clone().into(),
            material: material.clone().into(),
            transform: top_border_transform,
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(0.5, 0.5),
        Border,
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: mesh.clone().into(),
            material: material.clone().into(),
            transform: bottom_border_transform,
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(0.5, 0.5),
        Border,
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: mesh.clone().into(),
            material: material.clone().into(),
            transform: left_border_transform,
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(0.5, 0.5),
        Border,
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: mesh.clone().into(),
            material: material.clone().into(),
            transform: right_border_transform,
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(0.5, 0.5),
        Border,
    ));
}

impl Plugin for BordersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
