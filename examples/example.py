from fusion import normalization as norm
from tabulate import tabulate


scores = [1.0, 2.0, 3.0]  

# Min Max Normalization
min_max_norm = norm.min_max_normalization([1.0, 2.0, 3.0])

# Max Norm Rank Fusion
max_norm_rank_fusion = norm.max_norm(scores)

# Borda norm
ranks = [1, 2, 3]
num_candidates = 3
borda_norm = norm.borda_norm(scores,ranks, num_candidates)

# Rank Norm
rank_norm = norm.rank_norm(ranks)

# Sum Norm
sum_norm = norm.sum_norm(ranks)

# ZMUV Norm
zmuv_norm = norm.zmuv_norm(ranks)

# Prepare data for tabulation
data = [
    ["Min Max Normalization", min_max_norm],
    ["Max Norm Rank Fusion", max_norm_rank_fusion],
    ["Borda Norm", borda_norm],
    ["Rank Norm", rank_norm],
    ["Sum Norm", sum_norm],
    ["ZMUV Norm", zmuv_norm]
]

# Print table
print(tabulate(data, headers=["Normalization Method", "Result"], tablefmt="github"))
