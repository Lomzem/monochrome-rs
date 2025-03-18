use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(short, long)]
    pub input: PathBuf,

    #[clap(short, long)]
    pub output: Option<PathBuf>,

    #[clap(subcommand)]
    pub color_mode: ColorMode,
}

#[derive(Subcommand, Debug)]
pub enum ColorMode {
    Hsl {
        #[clap(long)]
        hue: f64,
        #[clap(short, long)]
        saturation: f64,
    },

    Rgb {
        rgb: String,
        #[clap(short, long)]
        saturation: Option<f64>,
    },
}
