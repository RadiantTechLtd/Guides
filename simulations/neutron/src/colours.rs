use arctk::parse::png;
use ndarray::{s, ArrayView3};
use ndarray_stats::QuantileExt;
use palette::{Gradient, LinSrgba};
use std::path::PathBuf;

/// Construct a colour gradient from a list of hex colours.
#[inline]
#[must_use]
pub fn create_cmap(colours: &[String]) -> Gradient<LinSrgba> {
    let cs: Vec<_> = colours
        .iter()
        .map(|col| {
            let (r, g, b, a) = hex_to_rgb(col);
            LinSrgba::new(r, g, b, a)
        })
        .collect();

    Gradient::new(cs)
}

/// Convert a hex colour to an RGB tuple.
#[inline]
#[must_use]
fn hex_to_rgb(hex: &str) -> (f32, f32, f32, f32) {
    let hex = hex.trim_start_matches('#');

    let red = u8::from_str_radix(&hex[0..2], 16).unwrap() as f32 / 255.0;
    let green = u8::from_str_radix(&hex[2..4], 16).unwrap() as f32 / 255.0;
    let blue = u8::from_str_radix(&hex[4..6], 16).unwrap() as f32 / 255.0;

    if hex.len() == 6 {
        return (red, green, blue, 1.0);
    }
    let alpha = u8::from_str_radix(&hex[6..8], 16).unwrap() as f32 / 255.0;

    (red, green, blue, alpha)
}

/// Slice a datacube.
#[inline]
pub fn slice(data: ArrayView3<f64>, colour_map: &Gradient<LinSrgba>, path: &PathBuf, label: &str) {
    let shape = data.shape();

    let max = *data.max().expect("Failed to find max value.");
    let x_slice = data.slice(s![.., .., shape[0] / 2]);
    let y_slice = data.slice(s![.., shape[1] / 2, ..]);
    let z_slice = data.slice(s![shape[2] / 2, .., ..]);

    png::save(
        x_slice
            .mapv(|value| colour_map.get((value / max) as f32))
            .view(),
        &PathBuf::from(&format!("{}_{}", path.to_str().unwrap(), "x"))
            .join(&format!("{}.png", label)),
    );
    png::save(
        y_slice
            .mapv(|value| colour_map.get((value / max) as f32))
            .view(),
        &PathBuf::from(&format!("{}_{}", path.to_str().unwrap(), "y"))
            .join(&format!("{}.png", label)),
    );
    png::save(
        z_slice
            .mapv(|value| colour_map.get((value / max) as f32))
            .view(),
        &PathBuf::from(&format!("{}_{}", path.to_str().unwrap(), "z"))
            .join(&format!("{}.png", label)),
    );
}
