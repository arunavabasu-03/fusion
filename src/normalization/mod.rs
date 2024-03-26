// src/normalization/mod.rs

mod borda;
mod max;
mod min_max;
mod rank;
mod sum;
mod zumv;

pub use borda::borda_norm;
pub use max::max_norm;
pub use min_max::min_max_normalization;
pub use rank::rank_norm;
pub use sum::sum_norm;
pub use zumv::zmuv_norm;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pymodule]
pub fn normalization(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(min_max_normalization))?;
      m.add_function(wrap_pyfunction!(max_norm, m)?)?;
    m.add_wrapped(wrap_pyfunction!(borda_norm))?;
    m.add_wrapped(wrap_pyfunction!(rank_norm))?;
    m.add_wrapped(wrap_pyfunction!(sum_norm))?;
    m.add_wrapped(wrap_pyfunction!(zmuv_norm))?;
    Ok(())
}
