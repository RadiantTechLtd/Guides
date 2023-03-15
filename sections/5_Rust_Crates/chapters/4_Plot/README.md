# Plotting

## Dependencies

We're going to make use of multi-dimensional arrays from the `ndarray` crate:

```shell
cargo add ndarray
```

Adding it this way will add the latest compatible version to the `dependencies` section of the `Cargo.toml` file.

## sample.rs

Add the following function to the `sample.rs` file:

```rust
use ndarray::Array2; // At the top of the file.

fn area(real: f64, imag: f64, scale: f64, res: [usize; 2], max_iters: u16, data: &mut Array2<f64>) {
    let aspect_ratio = res[0] as f64 / res[1] as f64;
    let real_start = real - (scale * 0.5);
    let imag_start = imag - (scale / aspect_ratio * 0.5);

    let delta = scale / (res[0] - 1).max(1) as f64;

    for yi in 0..res[1] {
        let imag = start.im + (delta * yi as f64);
        for xi in 0..res[0] {
            let real = real_start + (delta * xi as f64);
            data[(xi, yi)] = point(real, imag, max_iters);
        }
    }
}
```

This function will be used to generate a 2D array of values that represent the number of iterations it takes to escape the Mandelbrot set for each point in the area.

## main.rs

Update the [`main.rs`](./src/bin/main.rs) main function to use the `area()` function:

```rust
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
```

## Try it out

Run the `main.rs` binary and check the output:

```shell
cargo run --bin main
```

You should see a 2D array of values that represent the number of iterations it takes to escape the Mandelbrot set for each point in the area:

```text
   1   1   1   1   1   2   2   2   2   2   3   3   3   3   3   3   3   3   3   3   3   4   4   5   5   5   4   4   3   3   2   2   2   2   2   2   2   2   2   2
   1   1   1   1   2   2   2   2   3   3   3   3   3   3   3   3   3   3   3   4   4   4   5   6  13   6   5   4   4   3   3   3   2   2   2   2   2   2   2   2
   1   1   1   1   2   2   2   3   3   3   3   3   3   3   3   3   3   4   4   4   4   5   5   7   9  27   7   5   4   4   4   3   3   2   2   2   2   2   2   2
   1   1   1   2   2   2   3   3   3   3   3   3   3   3   3   3   4   4   4   4   5   5   6   9  27  17   8   5   5   4   4   3   3   3   2   2   2   2   2   2
   1   1   1   2   2   3   3   3   3   3   3   3   3   3   3   4   4   4   4   5   5   6   8  8410001000  27   7   5   5   4   4   3   3   3   2   2   2   2   2
   1   1   2   2   3   3   3   3   3   3   3   3   3   4   4   4   4   4   5   6   6   7   9  3110001000  19   8   6   6   5   4   4   3   3   3   2   2   2   2
   1   1   2   3   3   3   3   3   3   3   3   3   4   4   4   4   5   6  12  29   9  24  21  1810001000  16  12  16   7   7   9   4   3   3   3   3   2   2   2
   1   1   3   3   3   3   3   3   3   3   3   4   4   5   5   5   6   6   9 109  281000100010001000100010001000  96  14  19  14   5   4   3   3   3   3   2   2
   1   2   3   3   3   3   3   3   3   4   4   5   5   5   5   6   6   8  11  2110001000100010001000100010001000100010001000   8   5   4   3   3   3   3   2   2
   1   2   3   3   3   3   3   4   4   6   6   6   6   6   6   7   7  13100010001000100010001000100010001000100010001000  61   8   6   4   3   3   3   3   2   2
   1   3   3   3   4   4   4   5   5   6  10   8   8  10   8   8   9  7710001000100010001000100010001000100010001000100010001000  11   4   4   3   3   3   3   2
   1   3   4   4   4   4   5   5   6   7  10 191  291000  23  10  1110001000100010001000100010001000100010001000100010001000  23   7   4   4   3   3   3   3   2
   1   4   4   4   4   5   5   5   7   8  12  77100010001000 232  1510001000100010001000100010001000100010001000100010001000 141   7   4   4   3   3   3   3   2
   1   4   4   4   5   6   6   7  11  11  2910001000100010001000  2510001000100010001000100010001000100010001000100010001000  14   6   4   4   3   3   3   3   2
   1   5   6  11   7   7   8  10  261000100010001000100010001000100010001000100010001000100010001000100010001000100010001000   7   5   4   4   3   3   3   3   2
   1   5   6  11   8   9   9  11  24100010001000100010001000100010001000100010001000100010001000100010001000100010001000  29   7   5   4   4   3   3   3   3   2
   1   4   4   4   6   6   6   8  15  13100010001000100010001000  5910001000100010001000100010001000100010001000100010001000  18   6   4   4   3   3   3   3   2
   1   4   4   4   4   5   5   5   7   9  2010001000100010001000  1610001000100010001000100010001000100010001000100010001000  55   6   4   4   3   3   3   3   2
   1   3   4   4   4   4   5   5   6   7  101000  221000  56  12  1210001000100010001000100010001000100010001000100010001000  27   7   4   4   3   3   3   3   2
   1   3   3   3   4   4   4   5   5   7  12  18   9  11   9   8   9 12110001000100010001000100010001000100010001000100010001000  10   4   4   3   3   3   3   2
   1   3   3   3   3   3   4   4   5   7   8   6   6   6   7   7   8  111000100010001000100010001000100010001000100010001000  10   7   4   4   3   3   3   3   2
   1   2   3   3   3   3   3   3   3   4   5   5   5   5   5   6   6   9  1210001000100010001000100010001000100010001000  37   8   5   4   3   3   3   3   2   2
   1   1   3   3   3   3   3   3   3   3   3   4   5   5   5   5   6   6   91000 25710001000100010001000100010001000  17 188  25   5   4   3   3   3   3   2   2
   1   1   2   3   3   3   3   3   3   3   3   3   4   4   4   5   5   6  10  18  11  26  29100010001000  29  31  27   8   9   8   5   3   3   3   3   2   2   2
   1   1   2   2   3   3   3   3   3   3   3   3   3   4   4   4   4   5   6   7   7   7   9  4510001000  14   8   7   6   6   5   4   3   3   3   2   2   2   2
   1   1   1   2   3   3   3   3   3   3   3   3   3   3   4   4   4   4   4   5   6   6   8  5510001000  19   7   5   5   5   4   3   3   3   3   2   2   2   2
   1   1   1   2   2   3   3   3   3   3   3   3   3   3   3   3   4   4   4   4   5   5   6  16  21  23   9   5   5   4   4   4   3   3   3   2   2   2   2   2
   1   1   1   1   2   2   3   3   3   3   3   3   3   3   3   3   3   4   4   4   4   5   5   7  10  12   7   5   4   4   4   3   3   3   2   2   2   2   2   2
   1   1   1   1   2   2   2   2   3   3   3   3   3   3   3   3   3   3   4   4   4   4   5   6  30   7   7   4   4   4   3   3   2   2   2   2   2   2   2   2
   1   1   1   1   1   2   2   2   2   3   3   3   3   3   3   3   3   3   3   3   4   4   4   7  10   5   4   4   3   3   3   2   2   2   2   2   2   2   2   2
```
