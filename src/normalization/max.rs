use pyo3::prelude::*;
use pyo3::types::PyList;

/// Normalizes a list of scores by dividing each by the maximum score.
#[pyfunction]
pub fn max_norm(py: Python, scores: &PyList) -> PyResult<Vec<f64>> {
    // Extract scores from PyList to Vec<f64>
    let scores: Vec<f64> = scores.extract()?;
    
    // Find the maximum score in the list
    let max_score = match scores.iter().max_by(|x, y| x.partial_cmp(y).unwrap()) {
        Some(&max) => max,
        None => {
            // Handle empty list case by returning an empty Vec
            return Ok(vec![]);
        }
    };

    // Check for a max score of 0 to prevent division by zero
    if max_score == 0.0 {
        return Ok(vec![0.0; scores.len()]);
    }

    // Calculate max normalization for each score
    let normalized_scores: Vec<f64> = scores.iter().map(|&s| s / max_score).collect();

    Ok(normalized_scores)
}
