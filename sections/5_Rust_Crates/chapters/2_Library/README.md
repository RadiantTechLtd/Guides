# Library

## sample.rs

Let's create a new file within our library called [`sample.rs`](./src/sample.rs):

```shell
touch src/sample.rs
```

## Link

Now we need to link this file to our core library file, [`lib.rs`](./src/lib.rs):

```rust
pub mod sample;
```

## Code

We'll add a function which samples the mandelbrot set to [`sample.rs`](./src/sample.rs):

```rust
pub fn point(re: f64, im: f64, max_iters: u16) -> u16 {
    let mut z_re = 0.0;
    let mut z_im = 0.0;

    let mut i = 0;
    while ((z_re * z_re) + (z_im * z_im)) < 4.0 && (i < max_iters) {
        let new_re = (z_re * z_re) - (z_im * z_im) + re;
        let new_im = (2.0 * z_re * z_im) + im;
        z_re = new_re;
        z_im = new_im;
        i += 1;
    }

    i as u16
}
```

Check that the code is valid with:

```shell
cargo check
```

## Main

Update [`main.rs`](./src/bin/main.rs) to use the new function:

```rust
use mandy::sample;

fn main() {
    let real = -0.5;
    let imag = 0.0;
    let max_iters = 1000;

    let n = sample::point(real, imag, max_iters);

    println!("{} {} -> {}", real, imag, n);
}
```

Then run the it with:

```shell
cargo run --bin main
```
