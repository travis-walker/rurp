use rstest::rstest;
use rurp::grid::Grid;
use rurp::interpolate::inverse_distance_weighting::apply_inverse_distance_weighting;
pub mod utils;
use rurp::grid::Bounds;
use utils::{build_stub_point_data, CONUS_BOUNDS, STUB_BOUNDS};

#[rstest]
#[case(1, STUB_BOUNDS, 1, 10)]
#[case(2, CONUS_BOUNDS, 16000, 10000)]
fn test_apply_idw(
    #[case] case_number: usize,
    #[case] bounds: Bounds,
    #[case] resolution: usize,
    #[case] point_count: usize,
) {
    let (left, bottom, right, top) = bounds;
    let mut grid = Grid::empty_from_bounds(f64::NAN, left, bottom, right, top, resolution);
    let (x, y, z) = build_stub_point_data(&bounds, &point_count);

    apply_inverse_distance_weighting(&x, &y, &z, &mut grid);

    assert_grid_matches_snapshot!(grid, format!("test_apply_idw_{}", case_number));
}
