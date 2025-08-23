use bevy::audio::Source;
use core::time::Duration;
use fundsp::prelude::*;

fn get_audio_unit(base_freq: f32, control: Shared) -> Box<dyn AudioUnit> {
    let c = 0.2 * (organ_hz(base_freq) + organ_hz(base_freq + 30.0) + organ_hz(base_freq + 54.0));
    let mut c = (c * (var(&control) >> adsr_live(0.1, 0.2, 0.4, 0.2)))
        >> (pass() ^ pass())
        >> reverb_stereo(1.0, 0.5, 1.0);

    c.set_sample_rate(44_100.);

    Box::new(c)
}

pub struct DspDecoder {
    audio_unit: Box<dyn AudioUnit>,
    current_process: f32,
    progress_per_frame: f32,
    period: f32,
    sample_rate: u32,
    pending_r_sample: Option<f32>,
}

impl DspDecoder {
    pub fn new(frequency: f32, control: Shared) -> Self {
        let sample_rate = 44_100;
        let audio_unit = get_audio_unit(frequency, control.clone());

        DspDecoder {
            audio_unit,
            current_process: 0.,
            progress_per_frame: frequency / sample_rate as f32,
            period: std::f32::consts::PI * 2.,
            sample_rate,
            pending_r_sample: None,
        }
    }
}

impl Iterator for DspDecoder {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(pending_r_sample) = self.pending_r_sample {
            self.pending_r_sample = None;

            return Some(pending_r_sample);
        }

        let (left, right) = self.audio_unit.get_stereo();
        self.pending_r_sample = Some(right);

        Some(left)
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
