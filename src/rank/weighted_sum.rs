use std::collections::{HashMap, HashSet};
use core::hash::Hash;

pub fn weighted_sum_fusion<T: Eq + Hash + Clone>(
    ranked_lists: &[Vec<T>],
    weights: &[f64],
) -> Vec<T> {
    assert_eq!(ranked_lists.len(), weights.len());

    let mut documents: HashSet<T> = HashSet::new();
    for ranked_list in ranked_lists {
        for doc in ranked_list {
            documents.insert(doc.clone());
        }
    }

    let mut fusion_scores: HashMap<T, f64> = HashMap::new();
    for doc in &documents {
        let mut score = 0.0;
        for (ranked_list, weight) in ranked_lists.iter().zip(weights.iter()) {
            if let Some(rank) = get_rank(doc, ranked_list) {
                score += weight * rank as f64;
            }
        }
        fusion_scores.insert(doc.clone(), score);
    }

    let mut fused_ranking: Vec<_> = fusion_scores.into_iter().collect();
    fused_ranking.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    fused_ranking.into_iter().map(|(doc, _)| doc).collect()
}

fn get_rank<T: Eq + Hash>(doc: &T, ranked_list: &[T]) -> Option<usize> {
    ranked_list.iter().position(|x| x == doc).map(|pos| pos + 1)
}

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