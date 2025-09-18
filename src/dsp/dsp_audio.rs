use super::dsp_decoder::DspDecoder;
use bevy::prelude::*;
use std::sync::{atomic::AtomicBool, atomic::Ordering, Arc};

use crate::instruments::Instrument;

#[derive(Asset, TypePath)]
pub struct DspAudio {
    trigger: Arc<AtomicBool>,
    pub instrument: Box<dyn Instrument + Send + Sync + 'static>,
}

impl Decodable for DspAudio {
    type DecoderItem = <DspDecoder as Iterator>::Item;

    type Decoder = DspDecoder;

    fn decoder(&self) -> Self::Decoder {
        DspDecoder::new(self.instrument.audio_unit(), self.trigger.clone())
    }
}

impl DspAudio {
    pub fn new<I>(instrument: I) -> Self
    where
        I: Instrument + Send + Sync + 'static,
    {
        Self {
            trigger: Arc::new(AtomicBool::new(false)),
            instrument: Box::new(instrument),
        }
    }
    pub fn note_on(&self) {
        self.trigger.store(true, Ordering::Release);
    }

    pub fn note_off(&self) {
        self.trigger.store(false, Ordering::Release);
    }
}
