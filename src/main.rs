fn min_max_normalization(scores: &Vec<f64>) -> Vec<f64> {
    /*
    min max normalization algorithm
    */
    
    let min_value = scores.iter().cloned().fold(f64::MAX, f64::min);
    let max_value = scores.iter().cloned().fold(f64::MIN, f64::max);

    let mut normalized_scores = Vec::with_capacity(scores.len());

    for score in scores {
        let normalized_score = if max_value == min_value {
            0.5
        } else {
            (score - min_value) / (max_value - min_value)
        };
        normalized_scores.push(normalized_score);
    }

    normalized_scores
}


fn main() {
    let scores = vec![10.0, 25.0, 5.0, 15.0, 20.0];
    let normalized_scores = min_max_normalization(&scores);
    println!("{:?}", normalized_scores);
}