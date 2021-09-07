# Test report for javascript / file:///tmp/top-repos-quality-repos-q1dj35ii/mechanical-squirrel.git HEAD 6dd0ba3bb0fa5636a5edfa9ed0ef80885e4f10c6

### Classification report

PPCR: 0.190

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.571| 1.000| 0.727| 8| 14| 0.571 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 8| 0.000 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 20| 0.000 |
| `macro avg` | 0.333| 0.333| 0.190| 0.333| 0.242| 8| 42| 0.190 |
| `weighted avg` | 1.000| 1.000| 0.190| 1.000| 0.242| 8| 42| 0.190 |
| `micro avg` | 1.000| 1.000| 0.190| 1.000| 0.320| 8| 42| 0.190 |

### Confusion matrix

|refusal|  ∅| ␣| '| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|6 |8 |0 |0 |
|20 |0 |0 |0 |
|8 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.3333333333333333, "precision": 0.3333333333333333, "recall": 0.3333333333333333, "support": 8}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 8}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 8}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 8}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "macro avg": {"f1-score": 0.24242424242424243, "precision": 0.3333333333333333, "recall": 0.19047619047619047, "support": 42}, "micro avg": {"f1-score": 0.32, "precision": 1.0, "recall": 0.19047619047619047, "support": 42}, "weighted avg": {"f1-score": 0.24242424242424243, "precision": 0.3333333333333333, "recall": 0.19047619047619047, "support": 42}, "\u2205": {"f1-score": 0.7272727272727273, "precision": 1.0, "recall": 0.5714285714285714, "support": 14}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 20}},
  "ppcr": 0.19047619047619047
}
```
</details>
