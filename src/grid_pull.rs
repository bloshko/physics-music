use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::{
    grid::Grid,
    sound_object::{random_launch_impulse, SoundObject},
};

/// Strength of each grid line's gravity, in px³/s².
const GRID_GRAVITY: f32 = 2_000_000.;
/// Smooths out the pull near a line so it doesn't grow without limit as the
/// distance approaches 0. Larger values give a softer, wider snap, in px.
const GRID_SOFTENING: f32 = 10.;
/// Air resistance while pulled, so balls settle on a line instead of swinging
/// through it forever.
const PULLED_DAMPING: Damping = Damping {
    linear_damping: 40.,
    angular_damping: 40.,
};

pub struct GridPullPlugin;

/// Whether sound objects are moving freely or being pulled onto the grid.
#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum BallMotion {
    #[default]
    Free,
    PulledToGrid,
}

/// Switches between moving freely and being pulled onto the grid. Runs when A is pressed.
fn toggle_grid_pull(state: Res<State<BallMotion>>, mut next_state: ResMut<NextState<BallMotion>>) {
    next_state.set(match state.get() {
        BallMotion::Free => BallMotion::PulledToGrid,
        BallMotion::PulledToGrid => BallMotion::Free,
    });
}

/// Stops every sound object, so only grid gravity moves them from here on.
fn stop_sound_objects(mut velocities: Query<&mut Velocity, With<SoundObject>>) {
    for mut velocity in &mut velocities {
        *velocity = Velocity::zero();
    }
}

/// Pulls every sound object toward its nearest grid line, like gravity: the
/// pull gets stronger as the ball gets closer, then fades out right at the line.
fn apply_grid_gravity(
    grid: Res<Grid>,
    mut sound_objects: Query<
        (
            &Transform,
            &ReadMassProperties,
            &mut ExternalForce,
            &mut Damping,
        ),
        With<SoundObject>,
    >,
) {
    for (transform, mass, mut force, mut damping) in &mut sound_objects {
        // Only the nearest line pulls: summing every line would drag balls near
        // the edges off their line, since more lines lie on one side than the other.
        let distance = grid.nearest_line_x(transform.translation.x) - transform.translation.x;
        // Softened inverse-square gravity: about 1/distance² further out, and
        // fading smoothly to 0 right on the line.
        let acceleration = GRID_GRAVITY * distance
            / (distance * distance + GRID_SOFTENING * GRID_SOFTENING).powf(1.5);

        // Force = mass × acceleration, so every ball falls the same way regardless of mass.
        force.force = Vec2::new(mass.get().mass * acceleration, 0.);
        // Also covers balls spawned while the pull is on.
        damping.set_if_neq(PULLED_DAMPING);
    }
}

/// Turns grid gravity off and launches every sound object again with a random impulse.
fn release_sound_objects(
    mut commands: Commands,
    mut sound_objects: Query<(Entity, &mut ExternalForce, &mut Damping), With<SoundObject>>,
) {
    for (entity, mut force, mut damping) in &mut sound_objects {
        *force = ExternalForce::default();
        *damping = Damping::default();
        commands.entity(entity).insert(ExternalImpulse {
            impulse: random_launch_impulse(),
            ..default()
        });
    }
}

impl Plugin for GridPullPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<BallMotion>()
            .add_systems(OnEnter(BallMotion::PulledToGrid), stop_sound_objects)
            .add_systems(OnExit(BallMotion::PulledToGrid), release_sound_objects)
            .add_systems(
                Update,
                (
                    toggle_grid_pull.run_if(input_just_pressed(KeyCode::KeyA)),
                    apply_grid_gravity.run_if(in_state(BallMotion::PulledToGrid)),
                ),
            );
    }
}
