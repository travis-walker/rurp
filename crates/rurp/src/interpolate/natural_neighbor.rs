use std::error::Error;

use crate::grid::Grid;
use crate::point::Point;
use spade::{DelaunayTriangulation, Triangulation};

/// Interpolates to the grid using the Nearest Neighbor method.
///
/// # Arguments
/// `grid` - The grid to interpolate to.
/// `points` - The points to interpolate from.
///
/// # Errors
/// Returns an error if the triangulation fails.
pub fn interpolate(grid: &mut Grid, points: &[Point]) -> Result<(), Box<dyn Error>> {
    let triangulation: DelaunayTriangulation<Point> =
        DelaunayTriangulation::bulk_load(points.to_vec())?;

    let interpolator = triangulation.natural_neighbor();

    let gradient_point = [0., 0.];

    grid.iter_world_mut().for_each(|(x, y, grid_value)| {
        if let Some(interpolated_value) = interpolator.interpolate_gradient(
            |v| v.data().values[0],
            |_| gradient_point,
            1.,
            (x, y).into(),
        ) {
            *grid_value = interpolated_value;
        }
    });

    Ok(())
}
