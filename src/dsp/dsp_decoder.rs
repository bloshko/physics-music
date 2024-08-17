use bevy::audio::Source;
use bevy::utils::Duration;
use fundsp::prelude::*;

fn get_audio_unit() -> Box<dyn AudioUnit> {
    let c = 0.2 * (organ_hz(midi_hz(57.0)) + organ_hz(midi_hz(61.0)) + organ_hz(midi_hz(64.0)));
    let mut c = c >> pan(0.0);

    c.set_sample_rate(44_100.);

    return Box::new(c);
}

pub struct DspDecoder {
    audio_unit: Box<dyn AudioUnit>,
    current_process: f32,
    progress_per_frame: f32,
    period: f32,
    sample_rate: u32,
}

impl DspDecoder {
    pub fn new(frequency: f32) -> Self {
        let sample_rate = 44_100;
        let audio_unit = get_audio_unit();

        DspDecoder {
            audio_unit,
            current_process: 0.,
            progress_per_frame: frequency / sample_rate as f32,
            period: std::f32::consts::PI * 2.,
            sample_rate,
        }
    }
}

impl Iterator for DspDecoder {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let next_sample = self.audio_unit.get_mono();

        Some(next_sample)
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
