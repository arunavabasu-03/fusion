use pyo3::prelude::*;
use pyo3::types::PyList;

#[pyfunction]
pub fn sum_norm(py: Python, values: &PyList) -> PyResult<Vec<f64>> {
    // Convert Python list of values to Rust Vec<f64>
    let values: Vec<f64> = values.extract()?;

    let min_value = values.iter().cloned().fold(f64::INFINITY, |a, b| a.min(b));
    let sum_diff: f64 = values.iter().map(|x| x - min_value).sum();

    let normalized_values: Vec<f64> = values
        .iter()
        .map(|x| (x - min_value) / sum_diff)
        .collect();

    Ok(normalized_values)
}
