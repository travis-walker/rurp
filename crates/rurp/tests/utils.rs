use euclid::approxeq::ApproxEq;
use rand::{distributions::Uniform, rngs::StdRng, Rng, SeedableRng};
use rurp::grid::Bounds;

pub const STUB_BOUNDS: Bounds = (0., 0., 10., 10.);

pub const CONUS_BOUNDS: Bounds = (-2221060., 523589., 3181702., 3363319.);

pub fn equivalent(left: &f64, right: &f64) -> bool {
    left == right || left.is_nan() && right.is_nan() || left.approx_eq(right)
}

pub fn build_stub_point_data(
    bounds: &Bounds,
    point_count: &usize,
) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let mut rng = StdRng::seed_from_u64(43691);

    let (left, bottom, right, top) = bounds;
    let x_range = Uniform::new(left, right);
    let y_range = Uniform::new(bottom, top);
    let z_range = Uniform::new(-5.0f64, 120.0f64);
    (
        (0..*point_count).map(|_| rng.sample(x_range)).collect(),
        (0..*point_count).map(|_| rng.sample(y_range)).collect(),
        (0..*point_count).map(|_| rng.sample(z_range)).collect(),
    )
}

#[macro_export]
macro_rules! assert_grid_matches_snapshot {
    ($grid:expr, $id:expr) => {
        let file_path = std::env::current_dir()
            .expect("could not access file system")
            .join("tests/snapshots")
            .join(format!("{}.png", $id));
        rurp::draw::write_grid_data(&$grid, file_path.to_str().expect("could not convert path"));
    };
}
