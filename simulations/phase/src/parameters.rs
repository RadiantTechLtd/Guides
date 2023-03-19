use serde::Deserialize;
use serde_json;
use std::{f64::consts::PI, fs::read_to_string, path::PathBuf};

/// Input parameters.
#[derive(Debug, Deserialize)]
pub struct Parameters {
    /// Number of threads to use.
    pub num_threads: usize,
    /// Colour map.
    pub colour_map: Vec<String>,

    /// Number of neutrons to simulate.
    pub num_neutrons: usize,
    /// Number of neutrons to simulate per block.
    pub block_size: usize,
    /// Bump distance,
    pub bump_dist: f64,
    /// Minimum weight.
    pub min_weight: f64,

    /// Neutron gun position.
    pub gun_pos: [f64; 3],
    /// Neutron gun target point.
    pub gun_target: [f64; 3],
    /// Neutron gun angular spread.
    pub gun_spread: f64,

    /// Scattering coefficient.
    pub scat_coeff: f64,
    /// Absorption coefficient.
    pub abs_coeff: f64,

    /// Minimum xyz-coordinates for the scattering material.
    pub mins: [f64; 3],
    /// Maximum xyz-coordinates for the scattering material.
    pub maxs: [f64; 3],
    /// Number of voxels in each direction.
    pub num_voxels: [usize; 3],
}

impl Parameters {
    /// Load from file.
    #[inline]
    #[must_use]
    pub fn load(path: &PathBuf) -> Self {
        let contents = read_to_string(path).expect(&format!(
            "Failed to read parameters file at {}",
            path.display(),
        ));
        let mut params: Parameters =
            serde_json::from_str(&contents).expect("Invalid parameters file");
        params.gun_spread *= PI / 180.0;
        params
    }
}
