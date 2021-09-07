# Train report for javascript / file:///tmp/top-repos-quality-repos-nxt_vcuv/forum-app-project2.git HEAD e4a79e5ed9de3fff6485ce855892e5007678e3a0

### Classification report

PPCR: 0.028

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `'` | 1.000| 1.000| 0.500| 1.000| 0.667| 194| 388| 0.500 |
| `∅` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 4380| 0.000 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1448| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 346| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 202| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 171| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 99| 0.000 |
| `micro avg` | 1.000| 1.000| 0.028| 1.000| 0.054| 194| 7034| 0.028 |
| `weighted avg` | 1.000| 1.000| 0.028| 1.000| 0.037| 194| 7034| 0.028 |
| `macro avg` | 0.143| 0.143| 0.071| 0.143| 0.095| 194| 7034| 0.028 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|4380 |0 |0 |0 |0 |0 |0 |0 |
|1448 |0 |0 |0 |0 |0 |0 |0 |
|194 |0 |0 |194 |0 |0 |0 |0 |
|346 |0 |0 |0 |0 |0 |0 |0 |
|202 |0 |0 |0 |0 |0 |0 |0 |
|171 |0 |0 |0 |0 |0 |0 |0 |
|99 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 194}, "macro avg": {"f1-score": 0.14285714285714285, "precision": 0.14285714285714285, "recall": 0.14285714285714285, "support": 194}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 194}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 194}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 388}, "macro avg": {"f1-score": 0.09523809523809523, "precision": 0.14285714285714285, "recall": 0.07142857142857142, "support": 7034}, "micro avg": {"f1-score": 0.05368013281682346, "precision": 1.0, "recall": 0.027580324139891952, "support": 7034}, "weighted avg": {"f1-score": 0.03677376551985593, "precision": 0.055160648279783904, "recall": 0.027580324139891952, "support": 7034}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4380}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 346}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 99}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 202}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 171}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1448}},
  "ppcr": 0.027580324139891952
}
```
</details>
