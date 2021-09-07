# Train report for javascript / file:///tmp/top-repos-quality-repos-rcqjsyj3/whats-new.git HEAD 199be4289d55e664cba2be26bf98297b70ce3df9

### Classification report

PPCR: 0.211

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `'` | 1.000| 1.000| 0.500| 1.000| 0.667| 249| 498| 0.500 |
| `␣` | 1.000| 1.000| 0.427| 1.000| 0.598| 224| 525| 0.427 |
| `∅` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 950| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 271| 0.000 |
| `weighted avg` | 1.000| 1.000| 0.211| 1.000| 0.288| 473| 2244| 0.211 |
| `macro avg` | 0.500| 0.500| 0.232| 0.500| 0.316| 473| 2244| 0.211 |
| `micro avg` | 1.000| 1.000| 0.211| 1.000| 0.348| 473| 2244| 0.211 |

### Confusion matrix

|refusal|  ∅| '| ␣| ⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|950 |0 |0 |0 |0 |
|249 |0 |249 |0 |0 |
|301 |0 |0 |224 |0 |
|271 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 249}, "macro avg": {"f1-score": 0.5, "precision": 0.5, "recall": 0.5, "support": 473}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 473}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 473}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 224}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 498}, "macro avg": {"f1-score": 0.3161993769470405, "precision": 0.5, "recall": 0.2316666666666667, "support": 2244}, "micro avg": {"f1-score": 0.3481781376518219, "precision": 1.0, "recall": 0.2107843137254902, "support": 2244}, "weighted avg": {"f1-score": 0.28788711746380796, "precision": 0.45588235294117646, "recall": 0.2107843137254902, "support": 2244}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 950}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 271}, "\u2423": {"f1-score": 0.5981308411214953, "precision": 1.0, "recall": 0.4266666666666667, "support": 525}},
  "ppcr": 0.2107843137254902
}
```
</details>
