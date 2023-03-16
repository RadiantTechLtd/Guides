use clap::Parser;

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

    #[arg(short, long, default_value = "100")]
    max_iters: u16,

    #[arg(short, long, default_value = "1920")]
    width: usize,

    #[arg(short, long, default_value = "1080")]
    height: usize,

    #[clap(short, long, value_parser, num_args = 2.., value_delimiter = ' ')]
    cmap: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let data = sample::area(
        args.real,
        args.imag,
        args.scale,
        [args.width, args.height],
        args.max_iters,
    );
    let mut img = colour::image(
        data,
        args.cmap.iter().map(|s| &**s).collect(),
        args.max_iters,
    );
    colour::encode(&mut img).save("mandy.png").unwrap();
}
