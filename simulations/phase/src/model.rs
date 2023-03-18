use arctk::{geom::Cube, rt::Ray};
use nalgebra::{Point3, Unit, Vector3};

use crate::grid::Grid;
use crate::parameters::Parameters;

/// Simulation model.
pub struct Model {
    /// Neutron gun Ray.
    pub gun_ray: Ray,
    /// Neutron gun angular spread.
    pub gun_spread: f64,

    /// Interaction coefficient.
    pub interaction_coeff: f64,
    /// Single scattering albedo.
    pub albedo: f64,

    /// Measurement grid.
    pub grid: Grid,
}

impl Model {
    /// Construct a new model from a set of parameters.
    #[inline]
    #[must_use]
    pub fn new(params: &Parameters) -> Self {
        Self {
            gun_ray: Ray::new(
                Point3::new(params.gun_pos[0], params.gun_pos[1], params.gun_pos[2]),
                Unit::new_normalize(Vector3::new(
                    params.gun_target[0] - params.gun_pos[0],
                    params.gun_target[1] - params.gun_pos[1],
                    params.gun_target[2] - params.gun_pos[2],
                )),
            ),
            gun_spread: params.gun_spread,

            interaction_coeff: params.scat_coeff + params.abs_coeff,
            albedo: params.abs_coeff / (params.scat_coeff + params.abs_coeff),

            grid: Grid::new(
                Cube::new(
                    Point3::new(params.mins[0], params.mins[1], params.mins[2]),
                    Point3::new(params.maxs[0], params.maxs[1], params.maxs[2]),
                ),
                params.num_voxels,
            ),
        }
    }
}
