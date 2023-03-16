# Command Line Interface

Now we have a working program, let's add a command line interface to it.
This will allow us to pass in the parameters we want to use when generating the image, instead of having to edit the source code each time.

## Add clap

Add `clap`, a command line argument parser library, to the project:

```shell
cargo add clap --features derive
```

## Edit main.rs

Add the following imports at the top of the file:

```rust
use clap::Parser; // Add this line to the top of the file.

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
```

Then replace the `main` function with the following:

```rust
fn main() {
    let args = Args::parse();

    let data = sample::area(
        args.real,
        args.imag,
        args.scale,
        [args.width, args.height],
        args.max_iters,
    );
    let mut img = colour::image(data, args.cmap, args.max_iters);
    colour::encode(&mut img).save("mandy.png").unwrap();
}
```

## Try it out

Then run the program with the following command:

```shell
cargo run --bin main --release -- --real 0.428832585319999 --imag 0.231349121850911 --scale 1.0e-8 --max-iters 1000 --width 1920 --height 1080 --cmap 062B79 16498A 5995B7 FAFBBD FDE050 F1B351 FF0000
```
