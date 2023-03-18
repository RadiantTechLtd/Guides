use arctk::geom::Cube;

/// Three-dimensional uniform partitioning.
pub struct Grid {
    /// Boundary of the grid.
    pub boundary: Cube,
    /// Number of voxels in each direction.
    pub num_voxels: [usize; 3],
}

impl Grid {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(boundary: Cube, num_voxels: [usize; 3]) -> Self {
        Self {
            boundary,
            num_voxels,
        }
    }
}
