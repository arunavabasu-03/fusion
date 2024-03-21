pub fn rank_norm(scores: &[f64], ranks: &[usize]) -> Vec<f64> {
    let n = ranks.len();
    let mut normalized_scores = Vec::with_capacity(n);

    for (i, &r_i) in ranks.iter().enumerate() {
        let normalized_score = 1.0 - ((r_i as f64 - 1.0) / (n as f64 - 1.0));
        normalized_scores.push(normalized_score);
    }

    normalized_scores
}

