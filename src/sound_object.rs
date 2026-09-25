use crate::{
    cursor_position::CursorWorldPosition,
    dsp::dsp_audio::DspAudio,
    instruments::{Envelope, Kick, Organ, Snare},
    scanner::Scanner,
};
use bevy::{
    audio::AddAudioSource,
    color::palettes::css::{CRIMSON, GOLD, GREEN},
    input::common_conditions::input_just_pressed,
    prelude::*,
};
use bevy_rapier2d::prelude::*;
use std::{f32::consts::TAU, ops::Range};

const RADIUS: f32 = 10.;
/// Range of launch impulse strengths; each sound object gets a random one.
const LAUNCH_IMPULSE: Range<f32> = 1500.0..4500.0;
const PULSE_DURATION: f32 = 0.2;
const PULSE_GROWTH: f32 = 0.2;

const SNARE_ENVELOPE: Envelope = Envelope {
    attack: 0.,
    decay: 0.2,
    sustain: 0.,
    release: 0.05,
};
const SNARE_NOTE_LENGTH: f32 = 0.2;

const KICK_FREQUENCY: f32 = 50.;
const KICK_ENVELOPE: Envelope = Envelope {
    attack: 0.,
    decay: 0.3,
    sustain: 0.,
    release: 0.05,
};
const KICK_NOTE_LENGTH: f32 = 0.3;

const ORGAN_FREQUENCY: f32 = 440.;
const ORGAN_ENVELOPE: Envelope = Envelope {
    attack: 0.1,
    decay: 0.2,
    sustain: 0.4,
    release: 0.2,
};
const ORGAN_NOTE_LENGTH: f32 = 0.3;

pub struct SoundObjectPlugin;

#[derive(Component)]
#[require(
    GravityScale(0.0),
    Collider::ball(RADIUS),
    RigidBody::Dynamic,
    Restitution::coefficient(0.7),
    ActiveEvents::COLLISION_EVENTS
)]
pub struct SoundObject {
    /// How long the note is held before the envelope releases, in seconds.
    note_length: f32,
}

#[derive(Resource)]
struct SoundObjectHandles {
    mesh: Handle<Mesh>,
    snare_material: Handle<ColorMaterial>,
    kick_material: Handle<ColorMaterial>,
    organ_material: Handle<ColorMaterial>,
}

/// Present while a sound object's note is held. Releases the note when it finishes.
#[derive(Component)]
struct HeldNote(Timer);

/// Present while a sound object is playing its hit animation.
#[derive(Component)]
struct Pulse(Timer);

impl Default for Pulse {
    fn default() -> Self {
        Self(Timer::from_seconds(PULSE_DURATION, TimerMode::Once))
    }
}

/// Creates the mesh shared by all sound objects, and one material per instrument.
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(SoundObjectHandles {
        mesh: meshes.add(Circle::new(RADIUS)),
        snare_material: materials.add(Color::from(GOLD)),
        kick_material: materials.add(Color::from(CRIMSON)),
        organ_material: materials.add(Color::from(GREEN)),
    });
}

/// Spawns a sound object with its own voice at the cursor when Z (snare),
/// X (kick) or C (organ) is pressed, and launches it in a random direction
/// with a random strength.
fn spawn_on_key_press(
    mut commands: Commands,
    sound_object_handles: Res<SoundObjectHandles>,
    cursor_position: Res<CursorWorldPosition>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut dsp_assets: ResMut<Assets<DspAudio>>,
) {
    let (voice, note_length, material) = if keyboard.just_pressed(KeyCode::KeyZ) {
        (
            DspAudio::new(Snare::new(SNARE_ENVELOPE)),
            SNARE_NOTE_LENGTH,
            &sound_object_handles.snare_material,
        )
    } else if keyboard.just_pressed(KeyCode::KeyX) {
        (
            DspAudio::new(Kick::new(KICK_FREQUENCY, KICK_ENVELOPE)),
            KICK_NOTE_LENGTH,
            &sound_object_handles.kick_material,
        )
    } else if keyboard.just_pressed(KeyCode::KeyC) {
        (
            DspAudio::new(Organ::new(ORGAN_FREQUENCY, ORGAN_ENVELOPE)),
            ORGAN_NOTE_LENGTH,
            &sound_object_handles.organ_material,
        )
    } else {
        return;
    };

    commands.spawn((
        SoundObject { note_length },
        AudioPlayer(dsp_assets.add(voice)),
        Mesh2d(sound_object_handles.mesh.clone()),
        MeshMaterial2d(material.clone()),
        Transform::from_translation(cursor_position.0.extend(0.)),
        ExternalImpulse {
            impulse: random_launch_impulse(),
            ..default()
        },
    ));
}

/// Picks a random direction and a random strength from `LAUNCH_IMPULSE`.
fn random_launch_impulse() -> Vec2 {
    let direction = Vec2::from_angle(rand::random_range(0.0..TAU));
    direction * rand::random_range(LAUNCH_IMPULSE)
}

/// Despawns every sound object. Runs when Backspace is pressed.
fn clean_all_sound_objects(
    mut commands: Commands,
    sound_objects: Query<Entity, With<SoundObject>>,
) {
    for entity in &sound_objects {
        commands.entity(entity).despawn();
    }
}

/// Scales pulsing sound objects up and back down, and removes `Pulse` once
/// the animation is done.
fn animate_pulse(
    time: Res<Time>,
    mut commands: Commands,
    mut pulsing: Query<(Entity, &mut Pulse, &mut Transform)>,
) {
    for (entity, mut pulse, mut transform) in &mut pulsing {
        pulse.0.tick(time.delta());

        // Grows to the peak halfway through, then shrinks back: 0 → 1 → 0.
        let progress = 1. - (2. * pulse.0.fraction() - 1.).abs();
        transform.scale = Vec2::splat(1. + PULSE_GROWTH * progress).extend(1.);

        if pulse.0.is_finished() {
            commands.entity(entity).remove::<Pulse>();
        }
    }
}

/// Starts a note when the scanner reaches a sound object, unless its previous
/// note is still held.
fn handle_scanner_collision(
    mut commands: Commands,
    mut collision_events: MessageReader<CollisionEvent>,
    scanners: Query<(), With<Scanner>>,
    sound_objects: Query<(&SoundObject, &AudioPlayer<DspAudio>, Has<HeldNote>)>,
    dsp_assets: Res<Assets<DspAudio>>,
) {
    for event in collision_events.read() {
        // Notes end after their own length, so only the start of contact matters.
        let CollisionEvent::Started(a, b, _) = *event else {
            continue;
        };

        let Some(entity) = other_than_scanner(a, b, &scanners) else {
            continue;
        };
        let Ok((sound_object, voice, is_held)) = sound_objects.get(entity) else {
            continue;
        };
        if is_held {
            continue;
        }
        let Some(dsp) = dsp_assets.get(&voice.0) else {
            continue;
        };

        dsp.note_on();
        commands.entity(entity).insert((
            HeldNote(Timer::from_seconds(
                sound_object.note_length,
                TimerMode::Once,
            )),
            Pulse::default(),
        ));
    }
}

/// Releases each held note once its note length has passed.
fn release_notes(
    time: Res<Time>,
    mut commands: Commands,
    mut held_notes: Query<(Entity, &mut HeldNote, &AudioPlayer<DspAudio>)>,
    dsp_assets: Res<Assets<DspAudio>>,
) {
    for (entity, mut held_note, voice) in &mut held_notes {
        if !held_note.0.tick(time.delta()).is_finished() {
            continue;
        }

        if let Some(dsp) = dsp_assets.get(&voice.0) {
            dsp.note_off();
        }
        commands.entity(entity).remove::<HeldNote>();
    }
}

/// Returns the entity colliding with the scanner, if either of them is the scanner.
fn other_than_scanner(a: Entity, b: Entity, scanners: &Query<(), With<Scanner>>) -> Option<Entity> {
    if scanners.contains(a) {
        Some(b)
    } else if scanners.contains(b) {
        Some(a)
    } else {
        None
    }
}

impl Plugin for SoundObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_audio_source::<DspAudio>()
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    spawn_on_key_press,
                    handle_scanner_collision,
                    release_notes,
                    animate_pulse,
                    clean_all_sound_objects.run_if(input_just_pressed(KeyCode::Backspace)),
                ),
            );
    }
}
