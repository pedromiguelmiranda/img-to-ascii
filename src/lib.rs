use image::{DynamicImage, GenericImageView, imageops::FilterType};
use image::imageops::contrast;

// A more nuanced set of ASCII characters
pub const ASCII_CHARS: &[u8] = b"@%#*+=-:";

pub struct AsciiConverter {
    pub width: u32,
    pub height: u32,
}

impl AsciiConverter {
    pub fn new(width: u32, height: u32) -> Self {
        AsciiConverter { width, height }
    }

    pub fn convert_to_ascii(&self, img: &DynamicImage) -> String {
        // Convert to grayscale
        let mut grayscale = img.grayscale();
        // Enhance contrast
        grayscale = DynamicImage::ImageLuma8(contrast(&grayscale.to_luma8(), 1.5));

        // Adjust aspect ratio during resize to maintain clarity
        let resized = grayscale.resize_exact(self.width, self.height, FilterType::Lanczos3);
        self.image_to_ascii(&resized)
    }

    pub fn image_to_ascii(&self, img: &DynamicImage) -> String {
        let (width, height) = img.dimensions();
        let mut ascii_art = String::new();

        for y in 0..height {
            for x in 0..width {
                let pixel = img.get_pixel(x, y);
                let luma = pixel.0[0]; // Grayscale value (0-255)
                let ascii_char = self.map_luma_to_ascii(luma);
                ascii_art.push(ascii_char);
            }
            ascii_art.push('\n');
        }

        ascii_art
    }

    pub fn map_luma_to_ascii(&self, luma: u8) -> char {
        // Scale luma to the range of the ASCII_CHARS array
        let idx = (luma as f32 / 255.0 * (ASCII_CHARS.len() - 1) as f32).round() as usize;
        ASCII_CHARS[idx] as char
    }
}
