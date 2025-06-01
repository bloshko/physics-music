use super::dsp_decoder::DspDecoder;
use bevy::prelude::*;
use fundsp::shared::Shared;

#[derive(Asset, TypePath)]
pub struct DspAudio {
    pub frequency: f32,
    pub control: Shared,
}

impl Decodable for DspAudio {
    type DecoderItem = <DspDecoder as Iterator>::Item;

    type Decoder = DspDecoder;

    fn decoder(&self) -> Self::Decoder {
        DspDecoder::new(self.frequency, self.control.clone())
    }
}
