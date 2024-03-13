use crate::grid::Grid;
use crate::point::Point;
use geo::EuclideanDistance;
use kiddo::{KdTree, SquaredEuclidean};
use rayon::prelude::*;

fn calculate_weight(point_a: &geo::Point, point_b: &geo::Point, power: f64) -> f64 {
    let distance = point_a.euclidean_distance(point_b);
    if distance == 0.0 {
        return 1.;
    }
    distance.powf(-power)
}

fn calculate_interpolated_value(x: f64, y: f64, points: &[&Point], power: f64) -> f64 {
    let grid_point = geo::Point::new(x, y);
    let mut weights_sum = 0.;
    let weights: Vec<_> = points
        .iter()
        .map(|&point| {
            let weight = calculate_weight(&point.into(), &grid_point, power);
            weights_sum += weight;
            weight
        })
        .collect();
    points
        .par_iter()
        .zip(weights.par_iter())
        .map(|(point, weight)| point.values[0] * weight / weights_sum)
        .sum()
}

/// Interpolates to the grid using the Inverse Distance Weighting method.
pub fn interpolate(
    grid: &mut Grid,
    points: &[Point],
    power: f64,
    radius: f64,
    min_neighbors: usize,
) {
    let point_tree = KdTree::from_iter(
        points
            .iter()
            .enumerate()
            .map(|(idx, point)| ([point.x, point.y], idx as u64)),
    );

    let radius_squared = radius.powi(2);

    grid.iter_world_mut()
        .par_bridge()
        .for_each(|(x, y, grid_value)| {
            let neighbors = point_tree.within::<SquaredEuclidean>(&[x, y], radius_squared);
            if !neighbors.is_empty() && neighbors.len() >= min_neighbors {
                let neighbor_points: Vec<&Point> = neighbors
                    .iter()
                    .map(|neighbor| &points[neighbor.item as usize])
                    .collect();
                *grid_value = calculate_interpolated_value(x, y, &neighbor_points, power);
            }
        });
}
