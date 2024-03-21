use core::hash::Hash;
use std::collections::HashSet;
pub fn rrf_k_way<T: Eq + Hash + Clone>(ranked_lists: &[Vec<T>]) -> Vec<T> {
    let mut all_docs: HashSet<T> = HashSet::new();
    for ranked_list in ranked_lists.iter() {
        for doc in ranked_list.iter() {
            all_docs.insert(doc.clone());
        }
    }

    let mut fusion_scores: std::collections::HashMap<T, f64> = std::collections::HashMap::new();
    let k = ranked_lists.len() as f64;

    for doc in all_docs.iter() {
        let mut fusion_score = 0.0;
        for ranked_list in ranked_lists.iter() {
            if let Some(rank) = get_rank(doc, ranked_list) {
                fusion_score += 1.0 / rank as f64;
            }
        }
        fusion_score /= k;
        fusion_scores.insert(doc.clone(), fusion_score);
    }

    let mut sorted_docs: Vec<(T, f64)> = fusion_scores.into_iter().collect();
    sorted_docs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    sorted_docs.into_iter().map(|(doc, _)| doc).collect()
}


fn get_rank<T: Eq + Hash>(doc: &T, ranked_list: &[T]) -> Option<usize> {
    ranked_list.iter().position(|x| x == doc).map(|pos| pos + 1)
}



