use ndarray::Array3;
use palette::{Gradient, LinSrgba};
use std::{ops::AddAssign, path::PathBuf};

use crate::colours::slice;

/// Simulation data.
pub struct Data {
    /// Scattering.
    pub scatters: Array3<f64>,
    /// Distance travelled.
    pub travelled: Array3<f64>,
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
            travelled: Array3::zeros((num_voxels[0], num_voxels[1], num_voxels[2])),
            total: 0,
            absorbed: 0,
            escaped: 0,
        }
    }

    /// Save the data to file.
    #[inline]
    pub fn save(&self, colour_map: &Gradient<LinSrgba>, path: &PathBuf, step: usize) {
        slice(
            self.scatters.view(),
            colour_map,
            &path.join("scatters"),
            &format!("{0:>3}", step),
        );
        slice(
            self.travelled.view(),
            colour_map,
            &path.join("distance"),
            &format!("{0:>3}", step),
        );
    }
}

impl AddAssign<&Self> for Data {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.scatters += &rhs.scatters;
        self.travelled += &rhs.travelled;
        self.total += rhs.total;
        self.absorbed += rhs.absorbed;
        self.escaped += rhs.escaped;
    }
}
