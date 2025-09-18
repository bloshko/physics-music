use bevy::audio::Source;
use core::time::Duration;
use fundsp::prelude::*;

use std::sync::{atomic::AtomicBool, atomic::Ordering::Acquire, Arc};

pub struct DspDecoder {
    audio_unit: Box<dyn AudioUnit>,
    // current_process: f32,
    // progress_per_frame: f32,
    // period: f32,
    sample_rate: u32,
    pending_r_sample: Option<f32>,
    trigger: Arc<AtomicBool>,
}

impl DspDecoder {
    pub fn new(audio_unit: Box<dyn AudioUnit>, trigger: Arc<AtomicBool>) -> Self {
        let sample_rate = 44_100;

        DspDecoder {
            audio_unit,
            trigger,
            // current_process: 0.,
            // progress_per_frame: frequency / sample_rate as f32,
            // period: std::f32::consts::PI * 2.,
            sample_rate,
            pending_r_sample: None,
        }
    }
}

impl Iterator for DspDecoder {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.trigger.swap(false, Acquire) {
            self.audio_unit.reset();
        }

        if let Some(pending_r_sample) = self.pending_r_sample {
            self.pending_r_sample = None;

            return Some(pending_r_sample);
        }

        let (l_sample, r_sample) = self.audio_unit.get_stereo();
        self.pending_r_sample = Some(r_sample);

        Some(l_sample)
    }
}

impl Source for DspDecoder {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        2
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}
