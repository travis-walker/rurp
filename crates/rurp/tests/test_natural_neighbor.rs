use rstest::rstest;
use rurp::bounds::Bounds;
use rurp::grid::Grid;
use rurp::interpolate::natural_neighbor::interpolate;
use test_utils::{assert_grid_matches_snapshot, build_stub_points, CONUS_BOUNDS, STUB_BOUNDS};

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
    let mut grid = Grid::empty_from_bounds(bounds, resolution, f64::NAN).unwrap();
    let points = build_stub_points(bounds, &point_count);

    interpolate(&mut grid, &points[..]).unwrap();

    assert_grid_matches_snapshot(&grid, &format!("test_interpolate_natural_{}", case_number));
}
