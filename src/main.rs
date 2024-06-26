use clap::Parser;
use image::open;
use img_to_ascii::AsciiConverter;

#[derive(Parser, Debug)]
#[command(name = "Image to ASCII Converter")]
#[command(about = "Converts an image to ASCII art", long_about = None)]
pub struct Cli {
    /// Path to the image file
    #[arg(short = 'i', long)]
    pub image_path: String,

    /// Width of the output ASCII art
    #[arg(short = 'w', long, default_value_t = 80)]
    pub width: u32,

    /// Height of the output ASCII art
    #[arg(short = 'e', long, default_value_t = 40)]
    pub height: u32,
}

fn main() {
    let args = Cli::parse();

    // Load the image
    let img = open(args.image_path).expect("Failed to open image");

    // Create the converter
    let converter = AsciiConverter::new(args.width, args.height);

    // Convert to ASCII
    let ascii_art = converter.convert_to_ascii(&img);

    // Print the ASCII art
    println!("{}", ascii_art);
}
