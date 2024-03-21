use std::f64::consts::SQRT_2;

pub fn zmuv_norm(scores: &[f64]) -> Vec<f64> {
    let mean_score = calculate_mean(scores);
    let std_score = calculate_std_dev(scores, &mean_score);

    if std_score == 0.0 {
        return vec![0.0; scores.len()];
    }

    scores
        .iter()
        .map(|score| (score - mean_score) / std_score)
        .collect()
}

fn calculate_mean(scores: &[f64]) -> f64 {
    scores.iter().sum::<f64>() / (scores.len() as f64)
}

fn calculate_std_dev(scores: &[f64], mean: &f64) -> f64 {
    let squared_diffs: Vec<f64> = scores.iter().map(|score| (score - mean).powi(2)).collect();
    let variance = squared_diffs.iter().sum::<f64>() / (scores.len() as f64);
    variance.sqrt() / SQRT_2
}

