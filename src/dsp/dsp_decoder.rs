use bevy::audio::Source;
use core::num::NonZero;
use core::time::Duration;
use fundsp::prelude::*;

pub struct DspDecoder {
    audio_unit: Box<dyn AudioUnit>,
    gate: Shared,
    has_started: bool,
    sample_rate: NonZero<u32>,
    pending_r_sample: Option<f32>,
}

impl DspDecoder {
    pub fn new(audio_unit: Box<dyn AudioUnit>, gate: Shared) -> Self {
        DspDecoder {
            audio_unit,
            gate,
            has_started: false,
            sample_rate: NonZero::new(44_100).unwrap(),
            pending_r_sample: None,
        }
    }
}

impl Iterator for DspDecoder {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        // `adsr_live` starts out as if a note were already held, so running the
        // graph before the first note would play a short blip. Stay silent until then.
        if !self.has_started {
            if self.gate.value() <= 0. {
                return Some(0.);
            }
            self.has_started = true;
        }

        if let Some(r_sample) = self.pending_r_sample.take() {
            return Some(r_sample);
        }

        let (l_sample, r_sample) = self.audio_unit.get_stereo();
        self.pending_r_sample = Some(r_sample);

        Some(l_sample)
    }
}

impl Source for DspDecoder {
    fn current_span_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> NonZero<u16> {
        NonZero::new(2).unwrap()
    }

    fn sample_rate(&self) -> NonZero<u32> {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}
