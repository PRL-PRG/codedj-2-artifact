# Test report for javascript / file:///tmp/top-repos-quality-repos-87h5j5he/angular-syca.git HEAD c01051c2e780dd1823d7bf1e8ccbf15947a73c18

### Classification report

PPCR: 0.778

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.837| 1.000| 0.871| 0.911| 0.853| 195| 224| 0.871 |
| `␣` | 0.880| 0.647| 0.423| 0.746| 0.571| 34| 52| 0.654 |
| `'` | 0.923| 0.800| 0.444| 0.857| 0.600| 30| 54| 0.556 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 17| 20| 0.850 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 11| 0.545 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 4| 0.500 |
| `micro avg` | 0.849| 0.849| 0.660| 0.849| 0.743| 284| 365| 0.778 |
| `weighted avg` | 0.777| 0.849| 0.660| 0.805| 0.694| 284| 365| 0.778 |
| `macro avg` | 0.440| 0.408| 0.290| 0.419| 0.337| 284| 365| 0.778 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| ⏎␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|29 |195 |0 |0 |0 |0 |0 |
|18 |12 |22 |0 |0 |0 |0 |
|24 |6 |0 |24 |0 |0 |0 |
|3 |14 |3 |0 |0 |0 |0 |
|2 |0 |0 |2 |0 |0 |0 |
|5 |6 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u0027": {"f1-score": 0.8571428571428571, "precision": 0.9230769230769231, "recall": 0.8, "support": 30}, "macro avg": {"f1-score": 0.419020087046382, "precision": 0.43999779905359304, "recall": 0.407843137254902, "support": 284}, "micro avg": {"f1-score": 0.8485915492957746, "precision": 0.8485915492957746, "recall": 0.8485915492957746, "support": 284}, "weighted avg": {"f1-score": 0.8054828655124157, "precision": 0.777499058397928, "recall": 0.8485915492957746, "support": 284}, "\u2205": {"f1-score": 0.9112149532710281, "precision": 0.8369098712446352, "recall": 1.0, "support": 195}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 17}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u2423": {"f1-score": 0.7457627118644068, "precision": 0.88, "recall": 0.6470588235294118, "support": 34}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u0027": {"f1-score": 0.6, "precision": 0.9230769230769231, "recall": 0.4444444444444444, "support": 54}, "macro avg": {"f1-score": 0.3374700427216839, "precision": 0.43999779905359304, "recall": 0.2896761803011803, "support": 365}, "micro avg": {"f1-score": 0.7426810477657936, "precision": 0.8485915492957746, "recall": 0.6602739726027397, "support": 365}, "weighted avg": {"f1-score": 0.6939014332389831, "precision": 0.7755451096026086, "recall": 0.6602739726027397, "support": 365}, "\u2205": {"f1-score": 0.8533916849015318, "precision": 0.8369098712446352, "recall": 0.8705357142857143, "support": 224}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 20}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u2423": {"f1-score": 0.5714285714285714, "precision": 0.88, "recall": 0.4230769230769231, "support": 52}},
  "ppcr": 0.7780821917808219
}
```
</details>
