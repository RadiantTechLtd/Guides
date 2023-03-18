use clap::Parser;
use ndarray_stats::QuantileExt;
use std::fs::create_dir;

use mandy::{colour, sample};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    real: f64,

    #[arg(short, long)]
    imag: f64,

    #[arg(short, long)]
    scale: f64,

    #[arg(short, long)]
    zoom: f64,

    #[arg(short, long, default_value = "100")]
    frames: u16,

    #[arg(short, long, default_value = "100")]
    max_iters: u16,

    #[arg(short, long, default_value = "0.9")]
    rate: f64,

    #[arg(short, long, default_value = "1920")]
    width: usize,

    #[arg(short, long, default_value = "1080")]
    height: usize,

    #[clap(short, long, value_parser, num_args = 2.., value_delimiter = ' ', default_value = "000000 ffffff")]
    cmap: Vec<String>,
}

fn main() {
    let args = Args::parse();
    let cmap: Vec<_> = args.cmap.iter().map(|s| &**s).collect();

    let contrast = 255.0;

    if !std::path::Path::new("output").exists() {
        create_dir("output").unwrap();
    }

    let mut scale = args.scale;
    for n in 0..args.frames {
        let mut data = sample::area(
            args.real,
            args.imag,
            scale,
            [args.width, args.height],
            args.max_iters,
        );
        let min = *data.min().unwrap();
        data -= min;

        let max = *data.max().unwrap();
        let inv_max = 1.0 / max as f64;

        let mut values = data.mapv(|x| x as f64 * inv_max);
        values = values.mapv(|x| x.sqrt());
        data = values.mapv(|x| (x * 255.0) as u16);

        println!(
            "Frame {} of {} ({} iterations)",
            n, args.frames, args.max_iters
        );
        let mut img = colour::image(data, &cmap, contrast as u16);
        colour::encode(&mut img)
            .save(format!("output/mandy_{:0>6}.png", n))
            .unwrap();
        scale *= args.zoom;
    }
}
