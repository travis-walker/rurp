pub mod nearest_neighbor;
use nearest_neighbor::apply_nearest_neighbor_interpolation;
use pyo3::prelude::*;

#[pymodule]
pub fn interpolate(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(apply_nearest_neighbor_interpolation, m)?)?;
    Ok(())
}
