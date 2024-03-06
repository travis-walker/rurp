use crate::grid::Grid;
use geo::{EuclideanDistance, Point};
use rayon::prelude::*;

fn weighted_value(point_a: &Point, point_b: &Point, z: f64, power: f64) -> f64 {
    let distance = point_a.euclidean_distance(point_b);
    if distance == 0.0 {
        return z;
    }
    (z / distance).powf(power)
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

#[cfg(test)]
mod tests {
    use crate::draw::write_grid_data;

    use super::*;
    use rand::{distributions::Uniform, rngs::StdRng, Rng, SeedableRng};
    use rstest::rstest;

    fn build_stub_point_data(
        left: f64,
        bottom: f64,
        right: f64,
        top: f64,
        point_count: usize,
    ) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        let mut rng = StdRng::seed_from_u64(43691);
        let x_range = Uniform::new(left, right);
        let y_range = Uniform::new(bottom, top);
        let z_range = Uniform::new(-5.0f64, 120.0f64);
        (
            (0..point_count).map(|_| rng.sample(x_range)).collect(),
            (0..point_count).map(|_| rng.sample(y_range)).collect(),
            (0..point_count).map(|_| rng.sample(z_range)).collect(),
        )
    }

    #[rstest]
    #[case(1, 0.0, 0.0, 10.0, 10.0, 1, 10)]
    #[case(2, -2221060., 523589., 3181702., 3363319., 16000, 10000)]
    fn test_idw(
        #[case] case_number: usize,
        #[case] left: f64,
        #[case] bottom: f64,
        #[case] right: f64,
        #[case] top: f64,
        #[case] resolution: usize,
        #[case] point_count: usize,
    ) {
        let mut grid = Grid::empty_from_bounds(f64::NAN, left, bottom, right, top, resolution);
        let (x, y, z) = build_stub_point_data(left, bottom, right, top, point_count);

        apply_inverse_distance_weighting(&x, &y, &z, &mut grid);

        write_grid_data(
            &grid,
            &format!(
                "interpolate/snapshots/test_apply_inverse_distance_weighting_{case_number}.png"
            ),
        );
    }
}
