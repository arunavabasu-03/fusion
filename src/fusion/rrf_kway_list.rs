use pyo3::prelude::*;
use pyo3::types::PyList;
use std::collections::{HashMap, HashSet};

/// Performs Reciprocal Rank Fusion (RRF) on k-way ranked lists.
#[pyfunction]
pub fn rrf_k_way(py: Python, ranked_lists: &PyList) -> PyResult<Vec<String>> {
    let mut all_docs: HashSet<String> = HashSet::new();
    let mut ranked_list_vecs: Vec<Vec<String>> = Vec::new();
    
    // Extract the ranked lists and collect all documents.
    for py_ranked_list in ranked_lists.iter() {
        let ranked_list: Vec<String> = py_ranked_list?.extract()?;
        for doc in &ranked_list {
            all_docs.insert(doc.clone());
        }
        ranked_list_vecs.push(ranked_list);
    }

    let mut fusion_scores: HashMap<String, f64> = HashMap::new();
    let k = ranked_lists.len() as f64;

    // Calculate the fusion score for each document.
    for doc in all_docs.iter() {
        let mut fusion_score = 0.0;
        for ranked_list in &ranked_list_vecs {
            if let Some(rank) = get_rank(doc, ranked_list) {
                fusion_score += 1.0 / rank as f64;
            }
        }
        fusion_score /= k;
        fusion_scores.insert(doc.clone(), fusion_score);
    }

    // Sort the documents by their fusion score.
    let mut sorted_docs: Vec<(String, f64)> = fusion_scores.into_iter().collect();
    sorted_docs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    Ok(sorted_docs.into_iter().map(|(doc, _)| doc).collect())
}

fn get_rank(doc: &str, ranked_list: &[String]) -> Option<usize> {
    ranked_list.iter().position(|x| x == doc).map(|pos| pos + 1)
}
