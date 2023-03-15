use mandy::sample;

fn main() {
    let real = -0.5;
    let imag = 0.0;
    let max_iters = 1000;
    let res = [40, 30];
    let scale = 3.0;

    let data = sample::area(real, imag, scale, res, max_iters);
    for y in 0..res[1] {
        for x in 0..res[0] {
            print!("{:4}", data[(x, y)]);
        }
        println!();
    }
}
