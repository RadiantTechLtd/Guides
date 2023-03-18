# Movies

It would be great to dive into the depths of the Mandelbrot set.
Let's create a wrapper around our imaging code that will zoom the length scale of the image by a given factor each frame.

## Edit main.rs

Edit the `Args` struct to accommodate the new arguments:

```rust
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

    #[arg(short, long, default_value = "0.99")]
    rate: f64,

    #[arg(short, long, default_value = "1920")]
    width: usize,

    #[arg(short, long, default_value = "1080")]
    height: usize,

    #[clap(short, long, value_parser, num_args = 2.., value_delimiter = ' ', default_value = "000000 ffffff")]
    cmap: Vec<String>,
}
```

Then replace the `main` function with the following:

```rust
fn main() {
    let args = Args::parse();
    let cmap: Vec<_> = args.cmap.iter().map(|s| &**s).collect();

    if !std::path::Path::new("output").exists() {
        create_dir("output").unwrap();
    }

    let mut scale = args.scale;
    for n in 0..args.frames {
        let data = sample::area(
            args.real,
            args.imag,
            scale,
            [args.width, args.height],
            args.max_iters,
        );
        let f_iters = (args.max_iters as f64) + (1.0 / scale as f64).log(args.rate);
        let mut img = colour::image(data, &cmap, f_iters as u16);
        colour::encode(&mut img)
            .save(format!("output/mandy_{:0>6}.png", n))
            .unwrap();
        scale *= args.zoom;
    }
}
```

## Add output to .gitignore

Add the following to the `.gitignore` file:

```shell
# Output
/output
```

## Try it out

Then run the program with the following command:

```shell
cargo run --bin main --release -- --real 0.428832585319999 --imag 0.231349121850911 --scale 1.0e-1 --zoom 0.95 --frames 1000 --max-iters 100 --rate 10.0 --width 1920 --height 1080 --cmap 3f007a 7b0079 aa0072 d10065 f01055 ff4a42 ff762b ffa004 ffc900 fff000
```

You can then stitch the frames together with the `convert` tool:

```shell
convert -delay 5 -loop 0 *.png output.mov
```

or use `ffmpeg`:

```shell
ffmpeg -framerate 30 -i output/mandy_%06d.png -c:v libx264 -profile:v high -crf 20 -pix_fmt yuv420p output/mandy.mov
```
