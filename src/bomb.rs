use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_rapier2d::prelude::*;
use std::f32::consts::TAU;

use crate::{cursor_position::CursorWorldPosition, sound_object::SoundObject};

/// Balls further than this from the blast (px) aren't affected.
const BLAST_RADIUS: f32 = 150.;
/// Impulse given to a ball right at the center of the blast.
const BLAST_IMPULSE: f32 = 100000.;

const FLASH_DURATION: f32 = 0.3;
const FLASH_COLOR: Color = Color::srgb(1., 0.6, 0.2);

pub struct BombPlugin;

/// The expanding ring drawn where a bomb went off.
#[derive(Component)]
struct Explosion {
    center: Vec2,
    timer: Timer,
}

/// Sets off a bomb at the cursor: every sound object within `BLAST_RADIUS` is
/// pushed away from it, harder the closer it is. Runs when F is pressed.
fn detonate(
    mut commands: Commands,
    cursor_position: Res<CursorWorldPosition>,
    mut sound_objects: Query<(&Transform, &mut ExternalImpulse), With<SoundObject>>,
) {
    let center = cursor_position.0;

    for (transform, mut impulse) in &mut sound_objects {
        let offset = transform.translation.truncate() - center;
        let distance = offset.length();
        if distance >= BLAST_RADIUS {
            continue;
        }

        // Full strength at the center, fading linearly to 0 at the edge of the blast.
        let strength = BLAST_IMPULSE * (1. - distance / BLAST_RADIUS);
        // A ball exactly at the center has no "away" direction, so it gets a random one.
        let direction = offset
            .try_normalize()
            .unwrap_or_else(|| Vec2::from_angle(rand::random_range(0.0..TAU)));

        // Added rather than replaced, so a push already queued this frame isn't lost.
        impulse.impulse += direction * strength;
    }

    commands.spawn(Explosion {
        center,
        timer: Timer::from_seconds(FLASH_DURATION, TimerMode::Once),
    });
}

/// Draws each explosion as a ring that grows to the blast radius while fading
/// out, and removes it when done.
fn draw_explosions(
    time: Res<Time>,
    mut commands: Commands,
    mut gizmos: Gizmos,
    mut explosions: Query<(Entity, &mut Explosion)>,
) {
    for (entity, mut explosion) in &mut explosions {
        explosion.timer.tick(time.delta());

        let progress = explosion.timer.fraction();
        gizmos.circle_2d(
            explosion.center,
            BLAST_RADIUS * progress,
            FLASH_COLOR.with_alpha(1. - progress),
        );

        if explosion.timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}

impl Plugin for BombPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                detonate.run_if(input_just_pressed(KeyCode::KeyF)),
                draw_explosions,
            ),
        );
    }
}
