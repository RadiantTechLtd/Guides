use palette::{Gradient, LinSrgba};

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
