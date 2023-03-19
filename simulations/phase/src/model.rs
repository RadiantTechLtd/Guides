use arctk::{geom::Cube, rt::Ray};
use nalgebra::{Point3, Unit, Vector3};
use palette::{Gradient, LinSrgba};
use rand::{rngs::ThreadRng, Rng};
use std::f64::consts::PI;

use crate::colours::create_cmap;
use crate::grid::Grid;
use crate::neutron::Neutron;
use crate::parameters::Parameters;

/// Simulation model.
pub struct Model {
    /// Number of threads to use.
    pub num_threads: usize,
    /// Colour map.
    pub colour_map: Gradient<LinSrgba>,

    /// Bump distance,
    pub bump_dist: f64,
    /// Minimum weight.
    pub min_weight: f64,

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
            num_threads: params.num_threads,
            colour_map: create_cmap(&params.colour_map),

            bump_dist: params.bump_dist,
            min_weight: params.min_weight,

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
            albedo: 1.0 - (params.abs_coeff / (params.scat_coeff + params.abs_coeff)),

            grid: Grid::new(
                Cube::new(
                    Point3::new(params.mins[0], params.mins[1], params.mins[2]),
                    Point3::new(params.maxs[0], params.maxs[1], params.maxs[2]),
                ),
                params.num_voxels,
            ),
        }
    }

    /// Generate a neutron.
    #[inline]
    #[must_use]
    pub fn generate_neutron(&self, rng: &mut ThreadRng) -> Neutron {
        let mut ray = self.gun_ray.clone();
        let pitch = rng.gen_range(0.0..=self.gun_spread); // TODO: This spread is bias to 0.0
        let roll = rng.gen_range(0.0..(2.0 * PI));
        ray.rotate(pitch, roll);
        Neutron::new(ray)
    }
}
