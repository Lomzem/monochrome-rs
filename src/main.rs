use std::error::Error;

use args::Args;
use clap::Parser;
use image::{GenericImageView, ImageBuffer, Rgb};
use utils::hsl_to_rgb;

mod args;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    if !(0.0..=360.0).contains(&args.hue) {
        return Err("Hue must be between 0 and 360".into());
    }

    if !(0.0..=100.0).contains(&args.saturation) {
        return Err("Saturation must be between 0 and 100".into());
    }

    let img = image::open(&args.input)?;
    let (width, height) = img.dimensions();

    let mut output: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.pixels() {
        let r = pixel[0] as f32;
        let g = pixel[1] as f32;
        let b = pixel[2] as f32;

        let luminance = (0.2126 * r) + (0.7152 * g) + (0.0722 * b);
        let lightness = (luminance / 255.0) * 100.0;
        let rgb = hsl_to_rgb(args.hue, args.saturation, lightness);

        output.put_pixel(x, y, Rgb([rgb.0 as u8, rgb.1 as u8, rgb.2 as u8]))
    }

    output.save(&args.output)?;

    Ok(())
}
