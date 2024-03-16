use once_cell::sync::Lazy;
use rand::{distributions::Uniform, rngs::StdRng, Rng, SeedableRng};
use rurp::bounds::Bounds;
use rurp::grid::Grid;
use rurp::point::Point;

pub static STUB_BOUNDS: Lazy<Bounds> = Lazy::new(|| Bounds::new(0., 0., 100., 100.).unwrap());

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

// #[macro_export]
// macro_rules! assert_grid_matches_snapshot {
//     ($grid:expr, $sid:expr) => {
//         let cwd = std::env::current_dir().unwrap();
//         let file_path = cwd.clone().join(format!("tests/snapshots/{}.png", $sid));
//         let new_file_path = cwd.join(format!("tests/snapshots/{}.new.png", $sid));
//         if new_file_path.is_file() {
//             std::fs::remove_file(&new_file_path).unwrap();
//         }
//         rurp::draw::write_grid_data($grid, new_file_path.to_str().unwrap()).unwrap();

//         if !file_path.is_file() {
//             std::fs::rename(&new_file_path, &file_path).unwrap();
//             panic!("New snapshot created");
//         }

//         let existing_file = std::fs::read(&file_path).unwrap();
//         let new_file = std::fs::read(&new_file_path).unwrap();
//         if existing_file == new_file {
//             std::fs::remove_file(&new_file_path).unwrap();
//         } else {
//             std::fs::remove_file(&file_path).unwrap();
//             std::fs::rename(&new_file_path, &file_path).unwrap();
//             panic!("Snapshot did not match");
//         }
//     };
// }

pub fn assert_grid_matches_snapshot(grid: &Grid, snapshot_id: &str) {
    let cwd = std::env::current_dir().unwrap();
    let file_path = cwd
        .clone()
        .join(format!("tests/snapshots/{}.png", snapshot_id));
    let new_file_path = cwd.join(format!("tests/snapshots/{}.new.png", snapshot_id));
    if new_file_path.is_file() {
        std::fs::remove_file(&new_file_path).unwrap();
    }
    rurp::draw::write_grid_data(grid, new_file_path.to_str().unwrap()).unwrap();

    if !file_path.is_file() {
        std::fs::rename(&new_file_path, &file_path).unwrap();
        panic!("New snapshot created");
    }

    let existing_file = std::fs::read(&file_path).unwrap();
    let new_file = std::fs::read(&new_file_path).unwrap();
    if existing_file == new_file {
        std::fs::remove_file(&new_file_path).unwrap();
    } else {
        std::fs::remove_file(&file_path).unwrap();
        std::fs::rename(&new_file_path, &file_path).unwrap();
        panic!("Snapshot did not match");
    }
}
