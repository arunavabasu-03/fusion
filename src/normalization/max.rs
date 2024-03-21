use std::collections::{HashMap, HashSet};
use core::hash::Hash;
pub fn max_norm_rank_fusion<T: Eq + Hash + Clone>(ranked_lists: &[Vec<T>]) -> Vec<T> {
    let num_lists = ranked_lists.len();
    let mut item_ranks: HashMap<T, Vec<usize>> = HashMap::new();
    let mut all_items: HashSet<T> = HashSet::new();

    // Populate item_ranks and all_items
    for (list_idx, ranked_list) in ranked_lists.iter().enumerate() {
        for (rank, item) in ranked_list.iter().enumerate() {
            item_ranks.entry(item.clone()).or_default().push(rank + 1);
            all_items.insert(item.clone());
        }
    }

    let mut fused_ranks: Vec<(T, f64)> = all_items
        .into_iter()
        .map(|item| {
            let binding = Vec::new();
            let ranks = item_ranks.get(&item).unwrap_or(&binding);
            let max_rank = *ranks.iter().max().unwrap_or(&0) as f64;
            let normalized_ranks: Vec<f64> = ranks
                .iter()
                .map(|rank| (*rank as f64) / max_rank)
                .collect();
            let fused_rank = normalized_ranks.iter().sum::<f64>() / (num_lists as f64);
            (item, fused_rank)
        })
        .collect();

    fused_ranks.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap().then(ranked_lists.iter().flatten().position(|x| x == &a.0).cmp(&ranked_lists.iter().flatten().position(|x| x == &b.0))));

    fused_ranks.into_iter().map(|x| x.0).collect()
}

