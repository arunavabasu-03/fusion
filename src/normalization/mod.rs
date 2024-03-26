// src/normalization/mod.rs

mod borda;
mod max;
mod min_max;
mod rank;
mod sum;
mod zumv;

pub use min_max::min_max_normalization;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pymodule]
pub fn normalization(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(min_max_normalization))?;
    Ok(())
}