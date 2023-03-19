use arctk::parse::png;
use ndarray::{s, Array3};
use ndarray_stats::QuantileExt;
use palette::{Gradient, LinSrgba};
use std::{ops::AddAssign, path::PathBuf};

/// Simulation data.
pub struct Data {
    /// Scattering.
    pub scatters: Array3<f64>,
    /// Total samples.
    pub total: usize,
    /// absorbed.
    pub absorbed: usize,
    /// Escaped.
    pub escaped: usize,
}

impl Data {
    /// Construct an empty object.
    #[inline]
    #[must_use]
    pub fn new(num_voxels: [usize; 3]) -> Self {
        Self {
            scatters: Array3::zeros((num_voxels[0], num_voxels[1], num_voxels[2])),
            total: 0,
            absorbed: 0,
            escaped: 0,
        }
    }

    /// Save the data to file.
    #[inline]
    pub fn save(&self, colour_map: &Gradient<LinSrgba>, path: &PathBuf) {
        let shape = self.scatters.shape();

        let max = *self.scatters.max().expect("Failed to find max value.");
        let x_slice = self.scatters.slice(s![.., .., shape[0] / 2]);
        let y_slice = self.scatters.slice(s![.., shape[1] / 2, ..]);
        let z_slice = self.scatters.slice(s![shape[2] / 2, .., ..]);

        png::save(
            x_slice
                .mapv(|value| colour_map.get((value / max) as f32))
                .view(),
            &path.join("x.png"),
        );
        png::save(
            y_slice
                .mapv(|value| colour_map.get((value / max) as f32))
                .view(),
            &path.join("y.png"),
        );
        png::save(
            z_slice
                .mapv(|value| colour_map.get((value / max) as f32))
                .view(),
            &path.join("z.png"),
        );
    }
}

impl AddAssign<&Self> for Data {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.scatters += &rhs.scatters;
        self.total += rhs.total;
        self.absorbed += rhs.absorbed;
        self.escaped += rhs.escaped;
    }
}
