use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(short, long, help = "Path to the input image file to process")]
    pub input: PathBuf,

    #[clap(short, long, help = "Path to the output image file")]
    pub output: Option<PathBuf>,

    #[clap(subcommand)]
    pub color_mode: ColorMode,
}

#[derive(Subcommand, Debug)]
pub enum ColorMode {
    #[clap(about = "Convert the image using hue and saturation")]
    Hsl {
        #[clap(long, help = "The hue value for the HSL color mode (0.0 to 360.0)")]
        hue: f64,
        #[clap(
            short,
            long,
            help = "The saturation value for the HSL color mode (0.0 to 100.0)"
        )]
        saturation: f64,
    },

    #[clap(about = "Convert the image using Hex values")]
    Hex {
        hex: String,
        #[clap(short, long)]
        saturation: Option<f64>,
    },
}
