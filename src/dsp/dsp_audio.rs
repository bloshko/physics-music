use super::dsp_decoder::DspDecoder;
use bevy::prelude::*;

#[derive(Asset, TypePath)]
pub struct DspAudio {
    pub frequency: f32,
}

impl Decodable for DspAudio {
    type DecoderItem = <DspDecoder as Iterator>::Item;

    type Decoder = DspDecoder;

    fn decoder(&self) -> Self::Decoder {
        DspDecoder::new(self.frequency)
    }
}
