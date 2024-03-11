mod normalization;
use normalization::min_max::min_max_normalization;



fn main() {
    let scores = vec![10.0, 25.0, 5.0, 15.0, 20.0];
    let normalized_scores = min_max_normalization(&scores);
    println!("{:?}", normalized_scores);
}