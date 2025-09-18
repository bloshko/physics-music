use crate::dsp::dsp_audio::DspAudio;
use crate::instruments;
use bevy::prelude::*;

#[derive(Resource)]
pub struct InstrumentHandles {
    pub snare: Handle<DspAudio>,
    pub organ: Handle<DspAudio>,
}

#[derive(Component)]
pub enum InstrumentType {
    Organ,
    Snare,
}

pub struct InstrumentPlugin;

fn setup(mut commands: Commands, mut dsp_assets: ResMut<Assets<DspAudio>>) {
    let organ_handle = dsp_assets.add(DspAudio::new(instruments::Organ::new()));
    let snare_handle = dsp_assets.add(DspAudio::new(instruments::Snare::new()));

    commands.insert_resource(InstrumentHandles {
        organ: organ_handle,
        snare: snare_handle,
    });
}

impl Plugin for InstrumentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
