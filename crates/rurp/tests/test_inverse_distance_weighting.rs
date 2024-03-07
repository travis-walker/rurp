use rstest::rstest;
use rurp::grid::Grid;
use rurp::interpolate::inverse_distance_weighting::apply_inverse_distance_weighting;
pub mod utils;
use rurp::grid::Bounds;
use utils::{build_stub_points, CONUS_BOUNDS, STUB_BOUNDS};

#[rstest]
#[case(1, STUB_BOUNDS, 1, 10)]
#[case(2, CONUS_BOUNDS, 16000, 10000)]
#[ignore = "slow"]
#[case(3, CONUS_BOUNDS, 4000, 50000)]
fn test_apply_idw(
    #[case] case_number: usize,
    #[case] bounds: Bounds,
    #[case] resolution: usize,
    #[case] point_count: usize,
) {
    let mut grid = Grid::empty_from_bounds(bounds, resolution, f64::NAN);
    let points = build_stub_points(&bounds, &point_count);

    apply_inverse_distance_weighting(&mut grid, &points);

    assert_grid_matches_snapshot!(grid, format!("test_apply_idw_{}", case_number));
}
