use arctk::rt::Ray;
use nalgebra::{Unit, Vector3};
use rand::{rngs::ThreadRng, Rng};
use std::f64::consts::PI;

/// Ballistic localised particle.
pub struct Particle {
    /// Ray.
    pub ray: Ray,
    /// Weight.
    pub weight: f64,
    /// Number of scatterings.
    pub num_scatters: i32,
    /// Total distance traveled.
    pub dist_traveled: f64,
}

impl Particle {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(ray: Ray) -> Self {
        Self {
            ray,
            weight: 1.0,
            num_scatters: 0,
            dist_traveled: 0.0,
        }
    }

    /// Travel the given distance.
    #[inline]
    pub fn travel(&mut self, dist: f64) {
        self.ray.travel(dist);
        self.dist_traveled += dist;
    }

    /// Scatter the neutron.
    #[inline]
    pub fn scatter(&mut self, rng: &mut ThreadRng) {
        let theta = rng.gen_range(0.0..=PI);
        let phi = rng.gen_range(0.0..=2.0 * PI);

        self.ray.dir = Unit::new_normalize(Vector3::new(
            theta.sin() * phi.cos(),
            theta.sin() * phi.sin(),
            theta.cos(),
        ));

        self.num_scatters += 1;
    }
}
