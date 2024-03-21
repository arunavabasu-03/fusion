mod normalization;
mod fusion;

use fusion::weighted_sum::weighted_sum_fusion;

fn main() {
    let ranked_list1 = vec!["doc1", "doc2", "doc3", "doc4", "doc5"];
    let ranked_list2 = vec!["doc2", "doc4", "doc1", "doc3", "doc6"];
    let ranked_list3 = vec!["doc3", "doc1", "doc5", "doc2", "doc4"];
    let weights = vec![0.4, 0.3, 0.3];

    let fused_ranking = weighted_sum_fusion(
        &[ranked_list1, ranked_list2, ranked_list3],
        &weights,
    );

    println!("{:?}", fused_ranking);
}