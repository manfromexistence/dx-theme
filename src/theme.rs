use crate::{
    image::Image,
    palettes::core::CorePalette,
    scheme::Scheme,
};

pub fn theme_from_source(source: [u8; 4]) -> Scheme {
    let mut palette = CorePalette::new(source, false);

    Scheme::light_from_core_palette_mut(&mut palette)
}

pub fn theme_from_image(image: &Image) -> Scheme {
    let source = image.source_color_from_image();
    let mut palette = CorePalette::new(source, false);

    Scheme::light_from_core_palette_mut(&mut palette)
}
