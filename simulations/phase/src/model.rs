use arctk::geom::Cube;
use nalgebra::Point3;

use crate::parameters::Parameters;

/// Simulation model.
pub struct Model {
    /// Measurement grid.
    pub dom: Cube,
}

impl Model {
    /// Construct a new model from a set of parameters.
    #[inline]
    #[must_use]
    pub fn new(params: &Parameters) -> Self {
        Self {
            dom: Cube::new(
                Point3::new(params.mins[0], params.mins[1], params.mins[2]),
                Point3::new(params.maxs[0], params.maxs[1], params.maxs[2]),
            ),
        }
    }
}
