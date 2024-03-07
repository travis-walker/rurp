use rstest::rstest;
use rurp::grid::Grid;
use rurp::interpolate::nearest_neighbor::apply_nearest_neighbor_interpolation;
pub mod utils;
use rurp::grid::Bounds;
use utils::{build_stub_points, CONUS_BOUNDS, STUB_BOUNDS};

#[rstest]
#[case(1, STUB_BOUNDS, 1, 10)]
#[case(2, CONUS_BOUNDS, 4000, 8000)]
#[case(3, CONUS_BOUNDS, 2000, 16000)]
fn test_apply_nearest_neighbor_interpolation(
    #[case] case_number: usize,
    #[case] bounds: Bounds,
    #[case] resolution: usize,
    #[case] point_count: usize,
) {
    let (left, bottom, right, top) = bounds;
    let mut grid = Grid::empty_from_bounds(f64::NAN, left, bottom, right, top, resolution);
    let points = build_stub_points(&bounds, &point_count);

    apply_nearest_neighbor_interpolation(&mut grid, &points);

    assert_grid_matches_snapshot!(
        grid,
        format!("test_apply_nearest_neighbor_interpolation_{}", case_number)
    );
}
