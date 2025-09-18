use fundsp::prelude::*;

pub trait Instrument: Send + Sync + 'static {
    fn audio_unit(&self) -> Box<dyn AudioUnit + Send + Sync + 'static>;
}

pub struct Organ {
    base_freq: f32,
}

impl Default for Organ {
    fn default() -> Self {
        Self { base_freq: 440.0 }
    }
}

impl Organ {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Instrument for Organ {
    fn audio_unit(&self) -> Box<dyn AudioUnit + Send + Sync + 'static> {
        let c = 0.2 * organ_hz(self.base_freq);
        let mut c = (c * (impulse() >> adsr_live(0.1, 0.2, 0.4, 0.2)))
            >> (pass() ^ pass())
            >> reverb_stereo(0.3, 0.7, 1.0);

        c.set_sample_rate(44_100.);

        Box::new(c)
    }
}

pub struct Snare;

impl Instrument for Snare {
    fn audio_unit(&self) -> Box<dyn AudioUnit + Send + Sync + 'static> {
        let mut c = 0.2 * white() * (impulse() >> adsr_live(0.0, 0.2, 0.0, 0.0));

        c.set_sample_rate(44_100.);

        Box::new(c)
    }
}

impl Default for Snare {
    fn default() -> Self {
        Self
    }
}

impl Snare {
    pub fn new() -> Self {
        Self::default()
    }
}
