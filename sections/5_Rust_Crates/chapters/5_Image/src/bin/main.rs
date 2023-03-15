use mandy::{colour, sample};

fn main() {
    let real = -0.5;
    let imag = 0.0;
    let max_iters = 1000;
    let res = [1920, 1080];
    let scale = 3.0;
    let cmap = vec!["#000000", "#FFFFFF"];

    let data = sample::area(real, imag, scale, res, max_iters);
    let img = colour::image(data, cmap, max_iters);
    colour::encode(&img).save("mandelbrot.png").unwrap();
}
