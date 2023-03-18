use serde::Deserialize;
use serde_json;
use std::fs::read_to_string;
use std::path::PathBuf;

/// Input parameters.
#[derive(Debug, Deserialize)]
pub struct Parameters {
    /// Minimum xyz-coordinates for the scattering material.
    pub mins: [f64; 3],
    pub maxs: [f64; 3],
}

impl Parameters {
    /// Load from file.
    #[inline]
    #[must_use]
    pub fn load(path: &PathBuf) -> Option<Self> {
        let contents = read_to_string(path).expect(&format!(
            "Failed to read parameters file at {}",
            path.display(),
        ));
        serde_json::from_str(&contents).expect("Invalid parameters file")
    }
}
