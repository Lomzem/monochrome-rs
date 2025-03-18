use std::error::Error;

use args::Args;
use clap::Parser;
use hsl::HSL;
use image::{GenericImageView, ImageBuffer, Pixel, Rgb};

mod args;

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
        let mut pixel_hsl = HSL::from_rgb(&pixel.to_rgb().0);

        pixel_hsl.h = args.hue;
        pixel_hsl.s = args.saturation / 100.0;

        let pixel_rgb = HSL::to_rgb(&pixel_hsl);

        output.put_pixel(x, y, Rgb(pixel_rgb.into()));
    }

    output.save(&args.output)?;

    Ok(())
}
