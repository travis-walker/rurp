use rurp::draw::write_grid_data;
use rurp::grid::Grid;
use std::env;

#[test]
fn test_draw_a_grid() {
    let (left, bottom, right, top, resolution) = (0., 0., 500., 250., 1);
    let mut grid = Grid::empty_from_bounds(f64::NAN, left, bottom, right, top, resolution);
    grid.data
        .indexed_iter_mut()
        .for_each(|((y, x, _), value)| *value = { x + y } as f64);

    let file_path = env::current_dir()
        .unwrap()
        .join("tests/snapshots")
        .join("test_plot_a_grid_1.png");
    write_grid_data(&grid, file_path.to_str().unwrap());
}
