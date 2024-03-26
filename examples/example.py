import fusion

print(fusion.normalization.min_max_normalization([1.0, 2.0, 3.0]))




ranked_lists = [["item1", "item2", "item3"], ["item2", "item1", "item3"]]
result = fusion.normalization.max_norm_rank_fusion(ranked_lists)
print(result)



scores = [1.0, 2.0, 3.0]  # Example scores (unused in the current function logic, but passed for consistency)
ranks = [1, 2, 3]
num_candidates = 3

normalized_scores = fusion.normalization.borda_norm(scores, ranks, num_candidates)
print(normalized_scores)
