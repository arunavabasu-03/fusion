mod normalization;
mod fusion;
use fusion::rrf_kway_list::rrf_k_way;


fn main() {
    let ranked_lists = vec![
        vec!["A", "B", "C", "D"],
        vec!["B", "D", "A", "E"],
        vec!["C", "A", "D", "F"],
    ];

    let fused_ranking = rrf_k_way(&ranked_lists);
    println!("Fused Ranking: {:?}", fused_ranking);
}