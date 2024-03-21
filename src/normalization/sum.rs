pub fn sum_norm(values: &[f64]) -> Vec<f64> {
    let min_value = values.iter().cloned().fold(f64::INFINITY, |a, b| a.min(b));
    let sum_diff: f64 = values.iter().map(|x| x - min_value).sum();

    values
        .iter()
        .map(|x| (x - min_value) / sum_diff)
        .collect()
}
