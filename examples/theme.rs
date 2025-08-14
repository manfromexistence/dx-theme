use material_color_utilities_rs::{
    image::Image,
    theme::{theme_from_image, theme_from_source},
};

fn main() {
    println!("Theme from source color:");
    let theme = theme_from_source([255, 66, 135, 245]); // Google Blue
    println!("{:#?}", theme);

    println!("\nTheme from image:");
    // Create a 2x2 image with red, green, blue, and white pixels
    let image_buffer = image::RgbaImage::from_raw(
        2,
        2,
        vec![
            255, 0, 0, 255, // Red
            0, 255, 0, 255, // Green
            0, 0, 255, 255, // Blue
            255, 255, 255, 255, // White
        ],
    )
    .unwrap();

    let pixels: Vec<[u8; 4]> = image_buffer
        .pixels()
        .map(|p| [p[3], p[0], p[1], p[2]])
        .collect();

    let image = Image {
        width: 2,
        height: 2,
        pixels: &pixels,
    };

    let theme = theme_from_image(&image);
    println!("{:#?}", theme);
}
