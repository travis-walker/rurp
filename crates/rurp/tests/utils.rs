use once_cell::sync::Lazy;
use rand::{distributions::Uniform, rngs::StdRng, Rng, SeedableRng};
use rurp::bounds::Bounds;
use rurp::point::Point;

pub static STUB_BOUNDS: Lazy<Bounds> = Lazy::new(|| Bounds::new(0., 0., 10., 10.).unwrap());

pub static CONUS_BOUNDS: Lazy<Bounds> =
    Lazy::new(|| Bounds::new(-2_221_060., 523_589., 3_181_702., 3_363_319.).unwrap());

#[must_use]
pub fn build_stub_points(bounds: &Bounds, point_count: &usize) -> Vec<Point> {
    let mut rng = StdRng::seed_from_u64(43691);

    let x_range = Uniform::new(bounds.left(), bounds.right());
    let y_range = Uniform::new(bounds.bottom(), bounds.top());
    let z_range = Uniform::new(-5.0f64, 120.0f64);
    (0..*point_count)
        .map(|_| {
            Point::new(
                rng.sample(x_range),
                rng.sample(y_range),
                vec![rng.sample(z_range)],
            )
        })
        .collect()
}

#[macro_export]
macro_rules! assert_grid_matches_snapshot {
    ($grid:expr, $id:expr) => {
        let file_path = std::env::current_dir()
            .unwrap()
            .join("tests/snapshots")
            .join(format!("{}.png", $id));
        rurp::draw::write_grid_data(&$grid, file_path.to_str().unwrap()).unwrap();
    };
}
