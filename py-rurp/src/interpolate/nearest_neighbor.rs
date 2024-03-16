use numpy::{PyArray, PyArrayDyn};
use pyo3::prelude::*;
use rurp::{bounds, grid, interpolate::nearest_neighbor, point};
fn build_points() -> Vec<point::Point> {
    // TODO
    vec![
        point::Point::new(25.0, 75.0, vec![0.0]),
        point::Point::new(75.0, 25.0, vec![100.0]),
    ]
}

#[pyfunction]
pub fn apply_nearest_neighbor_interpolation(py: Python) -> &PyArrayDyn<f64> {
    let bounds = bounds::Bounds::new(0.0, 0.0, 100.0, 100.0).unwrap();
    let resolution = 1;

    let mut grid = grid::Grid::empty_from_bounds(&bounds, resolution, f64::NAN).unwrap();
    let points = build_points();

    nearest_neighbor::interpolate(&mut grid, &points).unwrap();

    PyArray::from_array(py, &grid.data().into_dyn())
}
