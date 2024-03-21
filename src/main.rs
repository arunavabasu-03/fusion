mod normalization;
mod fusion;
use fusion::rrf_kway_list::rrf_k_way;
use normalization::max::max_norm_rank_fusion;

fn main() {
    let ranked_list1 = vec!["item1", "item2", "item3", "item4", "item5"];
    let ranked_list2 = vec!["item2", "item4", "item1", "item3", "item6"];
    let ranked_list3 = vec!["item3", "item1", "item5", "item2", "item4"];

    let fused_ranking = max_norm_rank_fusion(&[ranked_list1, ranked_list2, ranked_list3]);

    println!("{:?}", fused_ranking);
}


