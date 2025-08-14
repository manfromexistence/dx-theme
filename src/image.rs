use crate::{quantize::quantizer_celebi::QuantizerCelebi, score};

pub struct Image<'a> {
    pub width: u32,
    pub height: u32,
    pub pixels: &'a [[u8; 4]],
}

impl<'a> Image<'a> {
    pub fn source_color_from_image(&self) -> [u8; 4] {
        let mut quantizer = QuantizerCelebi;
        let pixels_vec: Vec<[u8; 4]> = self.pixels.to_vec();
        let result = quantizer.quantize(&pixels_vec, 128);

        score::score(&result)[0]
    }
}
