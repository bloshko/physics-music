use fundsp::prelude::*;

/// ADSR envelope settings. Times are in seconds; `sustain` is a level from 0 to 1.
#[derive(Clone, Copy)]
pub struct Envelope {
    pub attack: f32,
    pub decay: f32,
    pub sustain: f32,
    pub release: f32,
}

impl Envelope {
    /// Envelope driven by `gate`: attacks when it goes above 0, releases when it drops to 0.
    fn follow(self, gate: &Shared) -> An<impl AudioNode<Inputs = U0, Outputs = U1>> {
        var(gate) >> adsr_live(self.attack, self.decay, self.sustain, self.release)
    }
}

pub trait Instrument: Send + Sync + 'static {
    /// Builds a new voice whose envelope is controlled by `gate`.
    fn audio_unit(&self, gate: &Shared) -> Box<dyn AudioUnit + Send + Sync + 'static>;
}

pub struct Organ {
    base_freq: f32,
    envelope: Envelope,
}

impl Organ {
    pub fn new(base_freq: f32, envelope: Envelope) -> Self {
        Self {
            base_freq,
            envelope,
        }
    }
}

impl Instrument for Organ {
    fn audio_unit(&self, gate: &Shared) -> Box<dyn AudioUnit + Send + Sync + 'static> {
        let mut c = (0.2 * organ_hz(self.base_freq) * self.envelope.follow(gate))
            >> (pass() ^ pass())
            >> reverb_stereo(0.3, 0.7, 1.0);

        c.set_sample_rate(44_100.);

        Box::new(c)
    }
}

pub struct Snare {
    envelope: Envelope,
}

impl Snare {
    pub fn new(envelope: Envelope) -> Self {
        Self { envelope }
    }
}

impl Instrument for Snare {
    fn audio_unit(&self, gate: &Shared) -> Box<dyn AudioUnit + Send + Sync + 'static> {
        let mut c = 0.2 * white() * self.envelope.follow(gate);

        c.set_sample_rate(44_100.);

        Box::new(c)
    }
}

/// Extra pitch at the start of a kick, in Hz. It drops to 0 over `KICK_PITCH_ENVELOPE`,
/// which gives the kick its punch.
const KICK_PITCH_SWEEP: f32 = 100.;
const KICK_PITCH_ENVELOPE: Envelope = Envelope {
    attack: 0.,
    decay: 0.05,
    sustain: 0.,
    release: 0.01,
};

pub struct Kick {
    base_freq: f32,
    envelope: Envelope,
}

impl Kick {
    pub fn new(base_freq: f32, envelope: Envelope) -> Self {
        Self {
            base_freq,
            envelope,
        }
    }
}

impl Instrument for Kick {
    fn audio_unit(&self, gate: &Shared) -> Box<dyn AudioUnit + Send + Sync + 'static> {
        let pitch = self.base_freq + KICK_PITCH_SWEEP * KICK_PITCH_ENVELOPE.follow(gate);
        let mut c = 0.5 * (pitch >> sine()) * self.envelope.follow(gate);

        c.set_sample_rate(44_100.);

        Box::new(c)
    }
}
