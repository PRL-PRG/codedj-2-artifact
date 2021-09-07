# Train report for javascript / file:///tmp/top-repos-quality-repos-z2f2rfmj/github-actions-continuous-delivery.git HEAD a5998abde5671cc419a86132cf45a16ce07cafa0

### Classification report

PPCR: 0.297

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.517| 1.000| 0.682| 496| 959| 0.517 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 552| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 158| 0.000 |
| `weighted avg` | 1.000| 1.000| 0.297| 1.000| 0.392| 496| 1669| 0.297 |
| `micro avg` | 1.000| 1.000| 0.297| 1.000| 0.458| 496| 1669| 0.297 |
| `macro avg` | 0.333| 0.333| 0.172| 0.333| 0.227| 496| 1669| 0.297 |

### Confusion matrix

|refusal|  ∅| ␣| '| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|463 |496 |0 |0 |
|552 |0 |0 |0 |
|158 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.3333333333333333, "precision": 0.3333333333333333, "recall": 0.3333333333333333, "support": 496}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 496}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 496}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 496}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 158}, "macro avg": {"f1-score": 0.227262313860252, "precision": 0.3333333333333333, "recall": 0.1724018074383038, "support": 1669}, "micro avg": {"f1-score": 0.4581986143187067, "precision": 1.0, "recall": 0.29718394248052726, "support": 1669}, "weighted avg": {"f1-score": 0.39175175373034454, "precision": 0.5745955662073098, "recall": 0.2971839424805273, "support": 1669}, "\u2205": {"f1-score": 0.681786941580756, "precision": 1.0, "recall": 0.5172054223149114, "support": 959}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 552}},
  "ppcr": 0.29718394248052726
}
```
</details>
