use ndarray::Array3;
use std::ops::AddAssign;

/// Simulation data.
pub struct Data {
    /// Scattering.
    pub scatters: Array3<f64>,
    /// Test
    pub total: usize,
}

impl Data {
    /// Construct an empty object.
    #[inline]
    #[must_use]
    pub fn new(num_voxels: [usize; 3]) -> Self {
        Self {
            scatters: Array3::zeros((num_voxels[0], num_voxels[1], num_voxels[2])),
            total: 0,
        }
    }
}

impl AddAssign<&Self> for Data {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.scatters += &rhs.scatters;
        self.total += rhs.total;
    }
}
