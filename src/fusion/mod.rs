pub mod rrf_kway_list;
pub mod weighted_sum;

use pyo3::prelude::*;
// pub use rrf_kway_list::rrf_k_way;

#[pymodule]
pub fn fusion(py: Python, m: &PyModule) -> PyResult<()> {
    // m.add_function(wrap_pyfunction!(rrf_k_way, m)?)?;
    Ok(())
}
