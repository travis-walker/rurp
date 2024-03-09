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

/// Interpolates to the grid using the Inverse Distance Weighting method.
pub fn interpolate(grid: &mut Grid, points: &[Point], power: f64) {
    let x = grid.x().to_owned();
    let y = grid.y().to_owned();

    grid.data_mut()
        .iter_mut()
        .zip(x.iter().zip(y.iter()))
        .par_bridge()
        .for_each(|(grid_value, (x, y))| {
            let grid_point = geo::Point::new(*x, *y);
            let weights: Vec<_> = points
                .iter()
                .map(|point| calculate_weight(&point.into(), &grid_point, power))
                .collect();
            let weights_total: f64 = weights.iter().sum();
            *grid_value = points
                .iter()
                .zip(weights.iter())
                .fold(0., |acc, (point, weight)| {
                    acc + point.values[0] * weight / weights_total
                });
        });
}
