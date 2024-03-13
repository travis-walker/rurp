use crate::grid::Grid;
use crate::point::Point;
use geo::EuclideanDistance;
use rayon::prelude::*;

fn calculate_weight(point_a: &geo::Point, point_b: &geo::Point, power: f64) -> f64 {
    let distance = point_a.euclidean_distance(point_b);
    if distance == 0.0 {
        return 1.;
    }
    distance.powf(-power)
}

fn calculate_interpolated_value(x: f64, y: f64, points: &[Point], power: f64) -> f64 {
    let grid_point = geo::Point::new(x, y);
    let weights: Vec<_> = points
        .iter()
        .map(|point| calculate_weight(&point.into(), &grid_point, power))
        .collect();
    let weights_total: f64 = weights.iter().sum();
    points
        .iter()
        .zip(weights.iter())
        .fold(0., |acc, (point, weight)| {
            acc + point.values[0] * weight / weights_total
        })
}

/// Interpolates to the grid using the Inverse Distance Weighting method.
pub fn interpolate(grid: &mut Grid, points: &[Point], power: f64) {
    grid.iter_world_mut()
        .par_bridge()
        .for_each(|(x, y, grid_value)| {
            *grid_value = calculate_interpolated_value(x, y, points, power);
        });
}
