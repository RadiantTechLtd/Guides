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
