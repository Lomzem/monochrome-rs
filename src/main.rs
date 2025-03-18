use std::{error::Error, path::PathBuf};

use args::{Args, ColorMode};
use clap::Parser;
use hsl::HSL;
use image::{GenericImageView, ImageBuffer, Pixel, Rgb};

mod args;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let arg_hsl = match args.color_mode {
        ColorMode::Hsl { hue, saturation } => HSL {
            h: hue,
            s: saturation / 100.0,
            l: 1.0,
        },
        ColorMode::Hex { hex, saturation } => {
            let hex = hex.trim_matches('#');

            let red = u8::from_str_radix(&hex[0..2], 16)?;
            let green = u8::from_str_radix(&hex[2..4], 16)?;
            let blue = u8::from_str_radix(&hex[4..6], 16)?;

            let mut hsl = HSL::from_rgb(&[red, green, blue]);

            if let Some(saturation) = saturation {
                hsl.s = saturation / 100.0;
            }
            hsl
        }
    };

    if !(0.0..=360.0).contains(&arg_hsl.h) {
        return Err("Hue must be between 0 and 360".into());
    }

    if !(0.0..=100.0).contains(&arg_hsl.s) {
        return Err("Saturation must be between 0 and 100".into());
    }

    let img = image::open(&args.input)?;
    let (width, height) = img.dimensions();

    let mut output: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.pixels() {
        let mut pixel_hsl = HSL::from_rgb(&pixel.to_rgb().0);

        pixel_hsl.h = arg_hsl.h;
        pixel_hsl.s = arg_hsl.s;

        let pixel_rgb = HSL::to_rgb(&pixel_hsl);

        output.put_pixel(x, y, Rgb(pixel_rgb.into()));
    }

    let output_path = args.output.unwrap_or_else(|| {
        let input_basename = args.input.file_stem().unwrap();
        let input_ext = args
            .input
            .extension()
            .expect("Previous parsers should have caught improper image file extension");
        PathBuf::from(format!(
            "{}_{}{}.{}",
            input_basename.to_str().unwrap(),
            arg_hsl.h.round(),
            arg_hsl.s.round(),
            input_ext.to_str().unwrap()
        ))
    });
    output.save(&output_path)?;

    Ok(())
}
