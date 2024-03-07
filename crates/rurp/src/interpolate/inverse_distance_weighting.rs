use crate::grid::Grid;
use geo::{EuclideanDistance, Point};
use rayon::prelude::*;

fn weighted_value(point_a: &Point, point_b: &Point, z: f64, power: f64) -> f64 {
    let distance = point_a.euclidean_distance(point_b);
    if distance == 0.0 {
        return z;
    }
    z / distance.powf(power)
}

pub fn apply_inverse_distance_weighting(x: &[f64], y: &[f64], z: &[f64], grid: &mut Grid) {
    let points = x
        .par_iter()
        .zip(y.par_iter())
        .zip(z.par_iter())
        .map(|((x, y), z)| {
            let point = Point::new(*x, *y);
            (point, *z)
        })
        .collect::<Vec<_>>();

    grid.data
        .iter_mut()
        .zip(grid.x.iter().zip(grid.y.iter()))
        .par_bridge()
        .for_each(|(grid_value, (x, y))| {
            let grid_point = Point::new(*x, *y);
            *grid_value = points.iter().fold(0.0, |acc, (point, z)| {
                acc + weighted_value(point, &grid_point, *z, 1.)
            })
        });
}
