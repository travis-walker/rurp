use rstest::rstest;
use rurp::bounds::Bounds;
use rurp::grid::Grid;
use rurp::interpolate::inverse_distance_weighting::interpolate;
pub mod utils;
use utils::{build_stub_points, CONUS_BOUNDS, STUB_BOUNDS};

#[rstest]
#[case(1, &*STUB_BOUNDS, 1, 100, 1., 25., 0)]
#[case(2, &*STUB_BOUNDS, 1, 100, 2., 25., 0)]
#[case(3, &*STUB_BOUNDS, 1, 100, 5., 25., 0)]
#[case(4, &*STUB_BOUNDS, 1, 100, 10., 25., 0)]
#[case(5, &*CONUS_BOUNDS, 16000, 10000, 2., 250_000., 0)]
#[case(6, &*CONUS_BOUNDS, 8000, 20000, 1., 250_000., 0)]
#[case(7, &*CONUS_BOUNDS, 8000, 20000, 2., 250_000., 3)]
fn test_interpolate(
    #[case] case_number: usize,
    #[case] bounds: &Bounds,
    #[case] resolution: usize,
    #[case] point_count: usize,
    #[case] power: f64,
    #[case] radius: f64,
    #[case] min_neighbors: usize,
) {
    let mut grid = Grid::empty_from_bounds(bounds, resolution, f64::NAN).unwrap();
    let points = build_stub_points(bounds, &point_count);

    interpolate(&mut grid, &points, power, radius, min_neighbors);

    utils::assert_grid_matches_snapshot(&grid, &format!("test_interpolate_idw_{}", case_number));
}
