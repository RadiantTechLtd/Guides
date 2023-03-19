use ndarray::Array3;
use netcdf;
use std::ops::AddAssign;

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
    pub fn save(&self, path: &str) {
        let mut file = netcdf::create(path).expect("Failed to create file.");

        let shape = self.scatters.shape();

        let dim1_name = "x";
        file.add_dimension(dim1_name, shape[0])
            .expect("Failed to add dimension.");
        let dim2_name = "y";
        file.add_dimension(dim2_name, shape[1])
            .expect("Failed to add dimension.");
        let dim3_name = "z";
        file.add_dimension(dim3_name, shape[2])
            .expect("Failed to add dimension.");

        let mut var = file
            .add_variable::<f64>("data", &[dim1_name, dim2_name, dim3_name])
            .expect("Failed to add variable.");
        var.put_values(
            self.scatters
                .as_slice()
                .ok_or("Missing slice data.")
                .expect("Failed to get slice data."),
            shape,
        )
        .expect("Failed to write data.");
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
