use serde::Deserialize;
use serde_json;
use std::f64::consts::PI;
use std::fs::read_to_string;
use std::path::PathBuf;

/// Input parameters.
#[derive(Debug, Deserialize)]
pub struct Parameters {
    /// Number of neutrons to simulate.
    pub num_neutrons: usize,
    /// Neutron gun position.
    pub gun_pos: [f64; 3],
    /// Neutron gun target point.
    pub gun_target: [f64; 3],
    /// Neutron gun angular spread.
    pub gun_spread: f64,

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
