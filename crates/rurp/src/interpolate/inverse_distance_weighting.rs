use crate::grid::Grid;
use crate::point::Point;
use kiddo::{KdTree, NearestNeighbour as NearestNeighbor, SquaredEuclidean};
use rayon::prelude::*;

fn calculate_interpolated_value(
    neighbors: &[NearestNeighbor<f64, u64>],
    points: &[Point],
    power: f64,
) -> f64 {
    let mut weights_sum = 0.;
    let weights: Vec<_> = neighbors
        .iter()
        .map(|neighbor| {
            let weight = if neighbor.distance == 0.0 {
                f64::INFINITY
            } else {
                // distance is the squared distance
                // (distance ^ (1/2)) ^ -power == distance ^ -(power / 2)
                neighbor.distance.powf(power / -2.)
            };
            weights_sum += weight;
            weight
        })
        .collect();
    neighbors
        .iter()
        .zip(weights.iter())
        .map(|(neighbor, weight)| {
            let point = &points[neighbor.item as usize];
            point.values[0] * weight / weights_sum
        })
        .sum()
}

/// Interpolates to the grid using the Inverse Distance Weighting method.
///
/// # Arguments
/// `grid` - The grid to interpolate to.
/// `points` - The points to interpolate from.
/// `power` - The power used in the interpolation. Values are weighted by 1 / distance ^ power.
/// `radius` - The radius to search for points to interpolate from for each grid point. Points outside of this radius are ignored.
/// `min_neighbors` - The minimum number of points in radius required to interpolate a value.
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
                *grid_value = calculate_interpolated_value(&neighbors, points, power);
            }
        });
}
