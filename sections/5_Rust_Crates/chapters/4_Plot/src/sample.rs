use ndarray::Array2;

pub fn point(real: f64, imag: f64, max_iters: u16) -> u16 {
    let mut z_real = 0.0;
    let mut z_imag = 0.0;

    let mut i = 0;
    while (((z_real * z_real) + (z_imag * z_imag)) < 4.0) && (i < max_iters) {
        let new_real = (z_real * z_real) - (z_imag * z_imag) + real;
        let new_imag = (2.0 * z_real * z_imag) + imag;
        z_real = new_real;
        z_imag = new_imag;
        i += 1;
    }

    i
}

pub fn area(real: f64, imag: f64, scale: f64, res: [usize; 2], max_iters: u16) -> Array2<u16> {
    let mut data = Array2::zeros((res[1], res[0]));

    let aspect_ratio = res[0] as f64 / res[1] as f64;
    let real_start = real - (scale * 0.5);
    let imag_start = imag - (scale / aspect_ratio * 0.5);

    let delta = scale / (res[0] - 1).max(1) as f64;

    for yi in 0..res[1] {
        let imag = imag_start + (delta * yi as f64);
        for xi in 0..res[0] {
            let real = real_start + (delta * xi as f64);
            data[(yi, xi)] = point(real, imag, max_iters);
        }
    }

    data
}
