pub fn borda_norm(scores: &[f64], ranks: &[usize], num_candidates: usize) -> Vec<f64> {
    let n = ranks.len();
    let mut normalized_scores = Vec::with_capacity(n);

    for &rank in ranks {
        let normalized_score = if rank > 0 {
            ((num_candidates - rank) as f64) / ((num_candidates - 1) as f64)
        } else {
            1.0 / (2.0 * num_candidates as f64)
        };
        normalized_scores.push(normalized_score);
    }

    normalized_scores
}

