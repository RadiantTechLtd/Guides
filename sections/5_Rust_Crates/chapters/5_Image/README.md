# Images

## Dependencies

We're going to make use of colours from the `palette` crate:

```shell
cargo add image
cargo add palette
```

## Add colour.rs

Add a new file called [`colour.rs`](./src/colour.rs) to the `src` directory:

```shell
touch src/colour.rs
```

Inside it add the following code:

```rust
use image::RgbImage;
use ndarray::{arr1, s, Array2, Array3};
use palette::{Gradient, LinSrgb, Pixel};

fn hex_to_rgb(hex: &str) -> (f32, f32, f32) {
    let hex = hex.trim_start_matches('#');

    let hex_val: u32 = (u32::from_str_radix(hex, 16).ok()).unwrap();

    let red = ((hex_val >> 16) & 0xFF) as f32 / 255.0;
    let green = ((hex_val >> 8) & 0xFF) as f32 / 255.0;
    let blue = (hex_val & 0xFF) as f32 / 255.0;

    (red, green, blue)
}

pub fn image(data: Array2<u16>, cols: Vec<&str>, max_iter: u16) -> Array3<u8> {
    let cs: Vec<_> = cols
        .iter()
        .map(|col| {
            let (r, g, b) = hex_to_rgb(col);
            LinSrgb::new(r, g, b)
        })
        .collect();
    let cmap: Gradient<LinSrgb> = Gradient::new(cs);

    let mut cols = Array3::<u8>::zeros((data.shape()[0], data.shape()[1], 3));
    let max_inv = 1.0 / max_iter as f32;
    let (width, height) = data.dim();
    for yi in 0..height {
        for xi in 0..width {
            let col = cmap.get(data[(xi, yi)] as f32 * max_inv);
            let u8s: [u8; 3] = col.into_format().into_raw();
            cols.slice_mut(s![xi, yi, ..]).assign(&arr1(&u8s));
        }
    }

    cols
}

pub fn encode(arr: &Array3<u8>) -> RgbImage {
    let (width, height, _) = arr.dim();
    RgbImage::from_vec(
        height as u32,
        width as u32,
        arr.as_slice().unwrap().to_vec(),
    )
    .expect("Container should have the right size for the image dimensions.")
}
```
