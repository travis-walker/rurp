use geo::{coord, polygon};
use rstest::rstest;
use rurp::grid::{Grid, ScreenSpace, WorldSpace};
pub mod utils;
use rurp::grid::Bounds;
use utils::{equivalent, CONUS_BOUNDS, STUB_BOUNDS};

#[rstest]
#[case(4000, 710, 1351, f64::NAN)]
#[case(8000, 355, 676, f64::NEG_INFINITY)]
#[case(1200, 2367, 4503, f64::MAX)]
fn test_from_bounds(
    #[case] resolution: usize,
    #[case] expected_height: usize,
    #[case] expected_width: usize,
    #[case] nodata: f64,
) {
    let grid = Grid::empty_from_bounds(CONUS_BOUNDS, resolution, nodata);

    let (left, bottom, right, top) = CONUS_BOUNDS;

    assert_eq!(grid.width(), expected_width);
    assert_eq!(grid.height(), expected_height);

    assert!(equivalent(&grid.data()[[0, 0, 0]], &nodata));
    assert_eq!(grid.x()[[0, 0]], left);
    assert_eq!(grid.y()[[0, 0]], bottom);

    let max_y = expected_height - 1;
    let max_x = expected_width - 1;
    assert!(equivalent(&grid.data()[[max_y, max_x, 0]], &nodata));
    assert_eq!(grid.x()[[max_y, max_x]], right);
    assert_eq!(grid.y()[[max_y, max_x]], top);

    let mid_y = expected_height / 2;
    let mid_x = expected_width / 2;
    assert!(equivalent(&grid.data()[[mid_y, mid_x, 0]], &nodata));
    assert!((grid.x()[[mid_y, mid_x]] - ((right + left) / 2.)).abs() < resolution as f64);
    assert!((grid.y()[[mid_y, mid_x]] - ((top + bottom) / 2.)).abs() < resolution as f64);
}

#[rstest]
#[case(STUB_BOUNDS, 1, 10, 10)]
#[case(CONUS_BOUNDS, 8000, 355, 676)]
fn test_properties(
    #[case] bounds: Bounds,
    #[case] resolution: usize,
    #[case] expected_height: usize,
    #[case] expected_width: usize,
) {
    let (left, bottom, right, top) = bounds;
    let grid = Grid::empty_from_bounds(bounds, resolution, f64::NAN);

    assert_eq!(grid.width(), expected_width);
    assert_eq!(grid.height(), expected_height);
    assert_eq!(grid.bounds(), (left, bottom, right, top));

    assert_eq!(grid.world_width(), right - left);
    assert_eq!(grid.world_height(), top - bottom);
}
#[rstest]
#[case(STUB_BOUNDS, 1, [0., 0.].into(), [0., 0.].into())]
#[case(CONUS_BOUNDS, 8000, [0., 0.].into(), [-2_221_060., 523_589.].into())]
fn test_transform(
    #[case] bounds: Bounds,
    #[case] resolution: usize,
    #[case] test_point_screen: euclid::Point2D<f64, ScreenSpace>,
    #[case] test_point_world: euclid::Point2D<f64, WorldSpace>,
) {
    let grid = Grid::empty_from_bounds(bounds, resolution, f64::NAN);

    let s_w_transform = grid.screen_to_world_transform();

    let sw_transformed_point = s_w_transform.transform_point(test_point_screen);
    assert_eq!(sw_transformed_point, test_point_world);

    let w_s_transform = grid.world_to_screen_transform();

    let ws_transformed_point = w_s_transform.transform_point(test_point_world);
    assert_eq!(ws_transformed_point, test_point_screen);
}

#[rstest]
#[case(1, STUB_BOUNDS, 1, polygon![(x: 2., y: 2.), (x: 7., y: 2.), (x: 7., y: 7.), (x: 2., y: 7.), (x: 2., y: 2.)],  -3.0)]
#[case(2, CONUS_BOUNDS, 2000, polygon![
            coord! {x: -1_951_222.716_269_676_6, y: 2_354_912.258_633_185},
            coord! {x: -2_041_264.291_279_755_5, y: 2_023_308.620_816_362_3},
            coord! {x: -1_678_141.122_611_409_3, y: 1_486_297.116_022_01},
            coord! {x: -1_665_558.550_028_715, y: 1_608_253.256_388_631_6},
            coord! {x: -1_601_328.924_522_811_5, y: 1_609_401.564_378_187_3},
            coord! {x: -1_476_039.396_968_355_9, y: 2_250_299.151_204_386_3},
            coord! {x: -1_951_222.716_269_676_6, y: 2_354_912.258_633_185},
        ], 50.12345)]
fn test_rasterize_polygon(
    #[case] case_number: usize,
    #[case] bounds: Bounds,
    #[case] resolution: usize,
    #[case] test_polygon: geo::Polygon<f64>,
    #[case] raster_label: f64,
) {
    let mut grid = Grid::empty_from_bounds(bounds, resolution, f64::NAN);

    grid.rasterize_polygons(&[test_polygon], &[raster_label]);

    assert_grid_matches_snapshot!(grid, format!("test_rasterize_polygon_{}", case_number));
}
