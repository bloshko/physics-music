use crate::cursor_position;
use crate::dsp::dsp_audio::DspAudio;
use crate::scanner::Scanner;
use bevy::{
    audio::AddAudioSource, color::palettes::css::GREEN, prelude::*, sprite::MaterialMesh2dBundle,
};
use bevy_rapier2d::prelude::*;

const RADIUS: f32 = 10.;

pub struct SoundObjectPlugin;

#[derive(Component)]
pub struct SoundObject;

#[derive(Resource)]
struct SoundObjectHandles {
    mesh_handle: Handle<Mesh>,
    material_handle: Handle<ColorMaterial>,
}

#[derive(Component)]
struct PulsateTimer(Timer);

#[derive(Component, PartialEq)]
enum SoundObjectUIState {
    PulsatingUp,
    PulsatingDown,
    Idle,
}

impl SoundObjectUIState {
    fn is_active(&self) -> bool {
        self == &SoundObjectUIState::PulsatingUp || self == &SoundObjectUIState::PulsatingDown
    }
}

impl Default for SoundObjectUIState {
    fn default() -> Self {
        SoundObjectUIState::Idle
    }
}

impl Default for PulsateTimer {
    fn default() -> Self {
        PulsateTimer(Timer::from_seconds(0.1, TimerMode::Once))
    }
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
            RigidBody::Dynamic,
            GravityScale(0.0),
            Restitution::coefficient(0.7),
            Collider::ball(RADIUS),
            SoundObject,
            ActiveEvents::COLLISION_EVENTS,
            PulsateTimer::default(),
            SoundObjectUIState::default(),
        ));
    };
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut assets: ResMut<Assets<DspAudio>>,
) {
    let mesh = meshes.add(Circle::new(RADIUS));
    let material = materials.add(Color::from(GREEN));

    commands.insert_resource(SoundObjectHandles {
        mesh_handle: mesh.clone(),
        material_handle: material.clone().into(),
    });

    let audio_handle = assets.add(DspAudio { frequency: 440. });

    commands.spawn(AudioSourceBundle {
        source: audio_handle,
        ..default()
    });
}

fn handle_pulse(
    time: Res<Time>,
    mut commands: Commands,
    mut q_sound_object: Query<
        (
            &mut PulsateTimer,
            &mut Transform,
            &SoundObjectUIState,
            Entity,
        ),
        With<SoundObject>,
    >,
) {
    for (mut pulsate_timer, mut transform, state, entity) in &mut q_sound_object {
        if state.is_active() {
            pulsate_timer.0.tick(time.delta());
        }

        match state {
            SoundObjectUIState::PulsatingUp => {
                if pulsate_timer.0.just_finished() {
                    commands
                        .entity(entity)
                        .insert(SoundObjectUIState::PulsatingDown)
                        .insert(PulsateTimer::default());
                } else {
                    transform.scale =
                        transform.scale + Vec3::new(2.0, 2.0, 0.) * time.delta_seconds();
                }
            }
            SoundObjectUIState::PulsatingDown => {
                if pulsate_timer.0.just_finished() {
                    transform.scale = Vec3::new(1., 1., 0.);
                    commands
                        .entity(entity)
                        .insert(SoundObjectUIState::default())
                        .insert(PulsateTimer::default());
                } else {
                    transform.scale =
                        transform.scale - Vec3::new(2.0, 2.0, 0.) * time.delta_seconds();
                }
            }
            _ => (),
        }
    }
}

fn handle_scanner_collision(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    q_sound_object: Query<&SoundObjectUIState, With<SoundObject>>,
    q_scanner: Query<Entity, With<Scanner>>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(ent1, ent2, _) => {
                let is_collision_with_scanner =
                    q_scanner.get(*ent1).is_ok() || q_scanner.get(*ent1).is_ok();

                if !is_collision_with_scanner {
                    ()
                }

                let sound_object = if q_sound_object.get(*ent1).is_ok() {
                    Some(ent1)
                } else if q_sound_object.get(*ent2).is_ok() {
                    Some(ent2)
                } else {
                    None
                };

                if let Some(&sound_object_entity) = sound_object {
                    let state = q_sound_object.get(sound_object_entity).unwrap();

                    if *state == SoundObjectUIState::default() {
                        commands
                            .entity(sound_object_entity)
                            .insert(SoundObjectUIState::PulsatingUp);
                    }
                }
            }
            CollisionEvent::Stopped(_, _, _) => (),
        }
    }
}

impl Plugin for SoundObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_audio_source::<DspAudio>()
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (spawn_on_click, handle_scanner_collision, handle_pulse),
            );
    }
}
