use pyo3::prelude::*;
use pyo3::types::PyList;
use std::collections::{HashMap, HashSet};

#[pyfunction]
pub fn max_norm_rank_fusion(py: Python, ranked_lists: &PyList) -> PyResult<Vec<String>> {
    let num_lists = ranked_lists.len();
    let mut item_ranks: HashMap<String, Vec<usize>> = HashMap::new();
    let mut all_items: HashSet<String> = HashSet::new();

    // Populate item_ranks and all_items
    for py_ranked_list in ranked_lists.iter() {
        let ranked_list: &PyList = py_ranked_list.extract()?; // Handle PyAny correctly
        for (rank, py_item) in ranked_list.iter().enumerate() {
            let item: String = py_item.extract()?; // Extract item as String
            item_ranks.entry(item.clone()).or_default().push(rank + 1);
            all_items.insert(item);
        }
    }

    let mut fused_ranks: Vec<(String, f64)> = all_items
        .into_iter()
        .map(|item| {
            let binding = Vec::new();
            let ranks = item_ranks.get(&item).unwrap_or(&binding);
            let max_rank = *ranks.iter().max().unwrap_or(&0) as f64;
            let normalized_ranks: Vec<f64> = ranks.iter().map(|rank| (*rank as f64) / max_rank).collect();
            let fused_rank = normalized_ranks.iter().sum::<f64>() / num_lists as f64;
            (item, fused_rank)
        })
        .collect();

    // Sort fused ranks based on the fused rank value, and in case of a tie, maintain the original ordering
    fused_ranks.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

    Ok(fused_ranks.into_iter().map(|(item, _)| item).collect())
}

