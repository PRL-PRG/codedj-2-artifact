# Test report for javascript / file:///tmp/top-repos-quality-repos-bjp_v9ti/book-store.git HEAD f6ce8d39b94f718bba5f4ab150634689dd032add

### Classification report

PPCR: 0.296

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.721| 1.000| 0.251| 0.838| 0.373| 44| 175| 0.251 |
| `␣` | 1.000| 0.042| 0.022| 0.080| 0.043| 24| 45| 0.533 |
| `'` | 0.400| 1.000| 0.500| 0.571| 0.444| 4| 8| 0.500 |
| `⏎` | 1.000| 1.000| 0.077| 1.000| 0.143| 1| 13| 0.077 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 3| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 3| 0.000 |
| `micro avg` | 0.685| 0.685| 0.202| 0.685| 0.312| 73| 247| 0.296 |
| `weighted avg` | 0.799| 0.685| 0.202| 0.576| 0.294| 73| 247| 0.296 |
| `macro avg` | 0.520| 0.507| 0.142| 0.415| 0.167| 73| 247| 0.296 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|131 |44 |0 |0 |0 |0 |0 |
|21 |17 |1 |6 |0 |0 |0 |
|4 |0 |0 |4 |0 |0 |0 |
|12 |0 |0 |0 |1 |0 |0 |
|3 |0 |0 |0 |0 |0 |0 |
|3 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.5714285714285715, "precision": 0.4, "recall": 1.0, "support": 4}, "macro avg": {"f1-score": 0.4149206349206349, "precision": 0.5202185792349726, "recall": 0.5069444444444445, "support": 73}, "micro avg": {"f1-score": 0.684931506849315, "precision": 0.684931506849315, "recall": 0.684931506849315, "support": 73}, "weighted avg": {"f1-score": 0.576464448793216, "precision": 0.799146642712778, "recall": 0.684931506849315, "support": 73}, "\u2205": {"f1-score": 0.8380952380952381, "precision": 0.7213114754098361, "recall": 1.0, "support": 44}, "\u23ce": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.07999999999999999, "precision": 1.0, "recall": 0.041666666666666664, "support": 24}},
  "cl_report_full": {"\u0027": {"f1-score": 0.4444444444444445, "precision": 0.4, "recall": 0.5, "support": 8}, "macro avg": {"f1-score": 0.16727686735055933, "precision": 0.5202185792349726, "recall": 0.14176231176231177, "support": 247}, "micro avg": {"f1-score": 0.31249999999999994, "precision": 0.684931506849315, "recall": 0.20242914979757085, "support": 247}, "weighted avg": {"f1-score": 0.2940220948986415, "precision": 0.7588239198247827, "recall": 0.20242914979757085, "support": 247}, "\u2205": {"f1-score": 0.3728813559322034, "precision": 0.7213114754098361, "recall": 0.25142857142857145, "support": 175}, "\u23ce": {"f1-score": 0.14285714285714288, "precision": 1.0, "recall": 0.07692307692307693, "support": 13}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u2423": {"f1-score": 0.04347826086956522, "precision": 1.0, "recall": 0.022222222222222223, "support": 45}},
  "ppcr": 0.29554655870445345
}
```
</details>
