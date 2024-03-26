use pyo3::prelude::*;
use pyo3::types::PyList;

#[pyfunction]
pub fn rank_norm(py: Python, ranks: &PyList) -> PyResult<Vec<f64>> {
    // Extract ranks from PyList to Vec<usize>
    let ranks: Vec<usize> = ranks.extract()?;
    let n = ranks.len();
    let mut normalized_scores = Vec::with_capacity(n);

    for (i, &r_i) in ranks.iter().enumerate() {
        let normalized_score = 1.0 - ((r_i as f64 - 1.0) / (n as f64 - 1.0));
        normalized_scores.push(normalized_score);
    }

    Ok(normalized_scores)
}
