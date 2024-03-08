use rstest::rstest;
use rurp::bounds::Bounds;
use rurp::grid::Grid;
use rurp::interpolate::nearest_neighbor::interpolate;
pub mod utils;
use utils::{build_stub_points, CONUS_BOUNDS, STUB_BOUNDS};

#[rstest]
#[case(1, &*STUB_BOUNDS, 1, 10)]
#[case(2, &*CONUS_BOUNDS, 4000, 8000)]
#[case(3, &*CONUS_BOUNDS, 2000, 16000)]
fn test_interpolate(
    #[case] case_number: usize,
    #[case] bounds: &Bounds,
    #[case] resolution: usize,
    #[case] point_count: usize,
) {
    let mut grid = Grid::empty_from_bounds(bounds, resolution, f64::NAN);
    let points = build_stub_points(bounds, &point_count);

    interpolate(&mut grid, &points).unwrap();

    assert_grid_matches_snapshot!(grid, format!("test_interpolate_nearest_{}", case_number));
}

#[rstest]
fn test_error_on_empty_points() {
    let mut grid = Grid::empty_from_bounds(&STUB_BOUNDS, 1, f64::NAN);
    let points = vec![];

    let result = interpolate(&mut grid, &points);

    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("No points to interpolate"));
}
