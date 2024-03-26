
```
| Normalization Method   | Result                                         |
|------------------------|------------------------------------------------|
| Min Max Normalization  | [0.0, 0.5, 1.0]                                |
| Max Norm Rank Fusion   | [0.3333333333333333, 0.6666666666666666, 1.0]  |
| Borda Norm             | [1.0, 0.5, 0.0]                                |
| Rank Norm              | [1.0, 0.5, 0.0]                                |
| Sum Norm               | [0.0, 0.3333333333333333, 0.6666666666666666]  |
| ZMUV Norm              | [-0.8660254037844385, 0.0, 0.8660254037844385] |
python3 examples/example.py  0.02s user 0.01s system 42% cpu 0.083 total
```


####  Checklists -

|  Fusion Algorithms | Status |
|------------|------------|
| RRF with K way list  | ✅ |
| JINA   |  |
| BGE   |  |
|  Weighted sum | ✅ |

|  Normalisation | Status |
|------------|------------|
| Min-Max Normalisation  | ✅ |
| Max Norm | ✅ |
| Sum Norm | ✅ |
|  ZUMV Norm | ✅ |
|  RANK Norm | ✅ |
|  Borda Norm | ✅ |
