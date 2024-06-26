use image::{GrayImage, Luma};
use img_to_ascii::AsciiConverter;

#[test]
fn test_map_luma_to_ascii() {
    let converter = AsciiConverter::new(80, 40);

    // Test for darkest and brightest values
    assert_eq!(
        converter.map_luma_to_ascii(0),
        '@',
        "Failed for luma value 0"
    );
    assert_eq!(
        converter.map_luma_to_ascii(255),
        ' ',
        "Failed for luma value 255"
    );

    // Test for middle value
    assert_eq!(
        converter.map_luma_to_ascii(127),
        '+',
        "Failed for luma value 127"
    );
}

#[test]
fn test_image_to_ascii_single_pixel() {
    let converter = AsciiConverter::new(1, 1);

    // Create a 1x1 grayscale image with a specific value
    let img = GrayImage::from_pixel(1, 1, Luma([128])); // Middle grayscale value
    let dynamic_img = image::DynamicImage::ImageLuma8(img);

    // Convert to ASCII
    let ascii_art = converter.image_to_ascii(&dynamic_img);

    // Assert the correct ASCII character
    assert_eq!(ascii_art, "=\n", "Failed for single pixel with luma 128");
}

#[test]
fn test_image_to_ascii_small_image() {
    let converter = AsciiConverter::new(2, 2);
    
    // Create a 2x2 grayscale image with specific values
    let img = GrayImage::from_raw(2, 2, vec![0, 255, 128, 64]).expect("Failed to create image");
    let dynamic_img = image::DynamicImage::ImageLuma8(img);

    // Convert to ASCII
    let ascii_art = converter.image_to_ascii(&dynamic_img);

    // Assert the correct ASCII art
    let expected_art = "@ \n=#\n"; // Values map to "@", " ", "=", "#"
    assert_eq!(ascii_art, expected_art, "Failed for 2x2 grayscale image");
}
