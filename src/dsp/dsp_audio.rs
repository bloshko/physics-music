use super::dsp_decoder::DspDecoder;
use bevy::prelude::*;
use fundsp::prelude::Shared;

use crate::instruments::Instrument;

/// One voice: an instrument plus the gate that starts and releases its notes.
#[derive(Asset, TypePath)]
pub struct DspAudio {
    gate: Shared,
    instrument: Box<dyn Instrument>,
}

impl Decodable for DspAudio {
    type Decoder = DspDecoder;

    fn decoder(&self) -> Self::Decoder {
        DspDecoder::new(self.instrument.audio_unit(&self.gate), self.gate.clone())
    }
}

impl DspAudio {
    pub fn new(instrument: impl Instrument) -> Self {
        Self {
            gate: Shared::new(0.),
            instrument: Box::new(instrument),
        }
    }

    /// Starts the envelope's attack.
    pub fn note_on(&self) {
        self.gate.set_value(1.);
    }

    /// Starts the envelope's release.
    pub fn note_off(&self) {
        self.gate.set_value(0.);
    }
}
