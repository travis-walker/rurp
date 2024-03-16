pub mod interpolate;
use pyo3::{prelude::*, wrap_pymodule};

#[pymodule]
fn rurp(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(interpolate::interpolate))?;
    Ok(())
}
