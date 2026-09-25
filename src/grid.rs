use bevy::{prelude::*, window::PrimaryWindow};

/// One scanner sweep across the window is one bar of 1/16 steps.
const STEPS_PER_BAR: u32 = 16;
const STEPS_PER_BEAT: u32 = 4;

const STEP_LINE_WIDTH: f32 = 1.;
const BEAT_LINE_WIDTH: f32 = 2.;
const STEP_LINE_COLOR: Color = Color::srgb_u8(60, 62, 66);
const BEAT_LINE_COLOR: Color = Color::srgb_u8(95, 98, 104);

/// Behind the scanner and sound objects, which are drawn at z = 0.
const GRID_Z: f32 = -1.;

pub struct GridPlugin;

/// Where the grid lines are, for anything that needs to line up with them.
#[derive(Resource)]
pub struct Grid {
    step_width: f32,
}

impl Grid {
    /// The x position of the grid line closest to `x`.
    pub fn nearest_line_x(&self, x: f32) -> f32 {
        let step = (x / self.step_width)
            .round()
            .clamp(0., (STEPS_PER_BAR - 1) as f32);
        step * self.step_width
    }
}

/// A background line marking one 1/16 step. It has no collider, so it's only visual.
#[derive(Component)]
pub struct GridLine;

/// Spawns a vertical line at the start of every 1/16 step across the window,
/// with the first step of each beat drawn wider and brighter.
fn setup(
    mut commands: Commands,
    window: Single<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window_size = window.resolution.size();
    let step_width = window_size.x / STEPS_PER_BAR as f32;
    commands.insert_resource(Grid { step_width });

    let mesh = meshes.add(Rectangle::default());
    let step_material = materials.add(STEP_LINE_COLOR);
    let beat_material = materials.add(BEAT_LINE_COLOR);

    for step in 0..STEPS_PER_BAR {
        let (width, material) = if step % STEPS_PER_BEAT == 0 {
            (BEAT_LINE_WIDTH, &beat_material)
        } else {
            (STEP_LINE_WIDTH, &step_material)
        };

        commands.spawn((
            GridLine,
            Mesh2d(mesh.clone()),
            MeshMaterial2d(material.clone()),
            Transform::from_xyz(step as f32 * step_width, window_size.y / 2., GRID_Z)
                .with_scale(Vec3::new(width, window_size.y, 1.)),
        ));
    }
}

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
