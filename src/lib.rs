// lib.rs
use pyo3::prelude::*;
use pyo3::wrap_pymodule;

mod fusion;
mod normalization;

#[pymodule]
fn fusion(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(normalization::normalization))?;
    m.add_wrapped(wrap_pymodule!(fusion::fusion))?;
    Ok(())
}
