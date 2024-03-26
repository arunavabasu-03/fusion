use pyo3::prelude::*;
use pyo3::types::PyList;
use std::f64::consts::SQRT_2;

#[pyfunction]
pub fn zmuv_norm(py: Python, scores: &PyList) -> PyResult<Vec<f64>> {
    let scores: Vec<f64> = scores.extract()?;
    let mean_score = calculate_mean(&scores);
    let std_score = calculate_std_dev(&scores, mean_score);

    let normalized_scores = if std_score == 0.0 {
        vec![0.0; scores.len()]
    } else {
        scores
            .iter()
            .map(|score| (score - mean_score) / (std_score * SQRT_2))
            .collect()
    };

    Ok(normalized_scores)
}

fn calculate_mean(scores: &[f64]) -> f64 {
    scores.iter().sum::<f64>() / (scores.len() as f64)
}

fn calculate_std_dev(scores: &[f64], mean: f64) -> f64 {
    let squared_diffs: Vec<f64> = scores.iter().map(|score| (score - mean).powi(2)).collect();
    let variance = squared_diffs.iter().sum::<f64>() / (scores.len() as f64);
    variance.sqrt()
}
