use crate::grid::Grid;
use crate::point::Point;
use geo::EuclideanDistance;
use rayon::prelude::*;

fn weighted_value(point_a: &geo::Point, point_b: &geo::Point, z: f64, power: f64) -> f64 {
    let distance = point_a.euclidean_distance(point_b);
    if distance == 0.0 {
        return z;
    }
    z / distance.powf(power)
}

/// Interpolates to the grid using the Inverse Distance Weighting method.
pub fn interpolate(grid: &mut Grid, points: &[Point]) {
    let x = grid.x().to_owned();
    let y = grid.y().to_owned();

    grid.data_mut()
        .iter_mut()
        .zip(x.iter().zip(y.iter()))
        .par_bridge()
        .for_each(|(grid_value, (x, y))| {
            let grid_point = geo::Point::new(*x, *y);
            *grid_value = points.iter().fold(0.0, |acc, point| {
                acc + weighted_value(&point.into(), &grid_point, point.values[0], 1.)
            });
        });
}
