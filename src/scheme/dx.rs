use bigcolor::BigColor;
use crate::scheme::Scheme;

fn oklch(l: f32, c: f32, h: f32) -> [u8; 4] {
    let color = BigColor::from_oklch(l, c, h, 1.0);
    let rgb = color.to_rgb();
    [rgb.r as u8, rgb.g as u8, rgb.b as u8, 255]
}

pub fn dx() -> Scheme {
    Scheme {
        primary: oklch(0.0, 0.0, 0.0),
        on_primary: oklch(1.0, 0.0, 0.0),
        primary_container: oklch(0.0, 0.0, 0.0),
        on_primary_container: oklch(1.0, 0.0, 0.0),
        secondary: oklch(0.94, 0.0, 0.0),
        on_secondary: oklch(0.0, 0.0, 0.0),
        secondary_container: oklch(0.94, 0.0, 0.0),
        on_secondary_container: oklch(0.0, 0.0, 0.0),
        tertiary: oklch(0.94, 0.0, 0.0),
        on_tertiary: oklch(0.0, 0.0, 0.0),
        tertiary_container: oklch(0.94, 0.0, 0.0),
        on_tertiary_container: oklch(0.0, 0.0, 0.0),
        error: oklch(0.63, 0.19, 23.03),
        on_error: oklch(1.0, 0.0, 0.0),
        error_container: oklch(0.63, 0.19, 23.03),
        on_error_container: oklch(1.0, 0.0, 0.0),
        background: oklch(0.99, 0.0, 0.0),
        on_background: oklch(0.0, 0.0, 0.0),
        surface: oklch(1.0, 0.0, 0.0),
        on_surface: oklch(0.0, 0.0, 0.0),
        surface_variant: oklch(0.92, 0.0, 0.0),
        on_surface_variant: oklch(0.0, 0.0, 0.0),
        outline: oklch(0.92, 0.0, 0.0),
        outline_variant: oklch(0.92, 0.0, 0.0),
        shadow: oklch(0.0, 0.0, 0.0),
        scrim: oklch(0.0, 0.0, 0.0),
        inverse_surface: oklch(0.13, 0.0, 0.0),
        inverse_on_surface: oklch(1.0, 0.0, 0.0),
        inverse_primary: oklch(1.0, 0.0, 0.0),
    }
}
