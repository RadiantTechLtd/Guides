use ndarray::Array3;

/// Simulation data.
pub struct Data {
    /// Scattering.
    pub scatters: Array3<f64>,
}

impl Data {
    /// Construct an empty object.
    #[inline]
    #[must_use]
    pub fn new(num_voxels: [usize; 3]) -> Self {
        Self {
            scatters: Array3::zeros((num_voxels[0], num_voxels[1], num_voxels[2])),
        }
    }
}
