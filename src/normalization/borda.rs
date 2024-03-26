use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::exceptions::PyValueError;

#[pyfunction]
pub fn borda_norm(py: Python, scores: &PyList, ranks: &PyList, num_candidates: usize) -> PyResult<Vec<f64>> {
    // Convert Python list of scores to Rust Vec<f64>
    let scores: Vec<f64> = scores.extract()?;
    
    // Convert Python list of ranks to Rust Vec<usize>
    let ranks: Vec<usize> = ranks.extract()?;

    if scores.len() != ranks.len() {
        return Err(PyValueError::new_err("scores and ranks must have the same length"));
    }

    let n = ranks.len();
    let mut normalized_scores = Vec::with_capacity(n);

    for &rank in &ranks {
        let normalized_score = if rank > 0 {
            ((num_candidates - rank) as f64) / ((num_candidates - 1) as f64)
        } else {
            1.0 / (2.0 * num_candidates as f64)
        };
        normalized_scores.push(normalized_score);
    }

    Ok(normalized_scores)
}
