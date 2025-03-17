use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(short, long)]
    pub input: PathBuf,

    #[clap(short, long)]
    pub output: PathBuf,

    #[clap(long)]
    pub hue: f32,

    #[clap(short, long)]
    pub saturation: f32,
}
