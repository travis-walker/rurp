use crate::grid::Grid;

#[allow(unused_variables)]
pub fn apply_linear_interpolation(x: &[f64], y: &[f64], z: &[f64], grid: &mut Grid) {}

#[cfg(test)]
mod tests {
    use crate::draw::wite_grid_data;

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
    fn test_it() {
        let (left, bottom, right, top, resolution) = (0.0, 0.0, 10.0, 10.0, 1);
        let mut grid = Grid::empty_from_bounds(f64::NAN, left, bottom, right, top, resolution);
        let (x, y, z) = build_stub_point_data(left, bottom, right, top, 10);

        apply_linear_interpolation(&x, &y, &z, &mut grid);

        wite_grid_data(
            &grid,
            "interpolate/snapshots/test_apply_linear_interpolation-1.png",
        );
    }
}
