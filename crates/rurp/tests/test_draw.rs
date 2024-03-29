use rstest::rstest;
use rurp::bounds::Bounds;
use rurp::draw::write_grid_data;
use rurp::grid::Grid;
use std::env;

#[rstest]
fn test_draw_a_grid() {
    let bounds = Bounds::new(0., 0., 500., 250.).unwrap();
    let mut grid = Grid::empty_from_bounds(&bounds, 1, f64::NAN).unwrap();
    grid.data_mut()
        .indexed_iter_mut()
        .for_each(|((y, x, _), value)| *value = { x + y } as f64);

    let file_path = env::current_dir()
        .unwrap()
        .join("tests/snapshots")
        .join("test_plot_a_grid_1.png");
    let _ = std::fs::remove_file(&file_path);
    write_grid_data(&grid, file_path.to_str().unwrap()).unwrap();
}
