# Test report for javascript / file:///tmp/top-repos-quality-repos-rcqjsyj3/whats-new.git HEAD 199be4289d55e664cba2be26bf98297b70ce3df9

### Classification report

PPCR: 0.639

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.796| 1.000| 0.667| 0.886| 0.726| 82| 123| 0.667 |
| `␣` | 1.000| 0.080| 0.045| 0.148| 0.087| 25| 44| 0.568 |
| `'` | 0.875| 1.000| 1.000| 0.933| 0.933| 14| 14| 1.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 10| 0.100 |
| `weighted avg` | 0.840| 0.803| 0.513| 0.733| 0.556| 122| 191| 0.639 |
| `macro avg` | 0.668| 0.520| 0.428| 0.492| 0.436| 122| 191| 0.639 |
| `micro avg` | 0.803| 0.803| 0.513| 0.803| 0.626| 122| 191| 0.639 |

### Confusion matrix

|refusal|  ∅| '| ␣| ⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|41 |82 |0 |0 |0 |
|0 |0 |14 |0 |0 |
|19 |20 |2 |2 |1 |
|9 |1 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.9333333333333333, "precision": 0.875, "recall": 1.0, "support": 14}, "macro avg": {"f1-score": 0.491991991991992, "precision": 0.6677791262135923, "recall": 0.52, "support": 122}, "micro avg": {"f1-score": 0.8032786885245902, "precision": 0.8032786885245902, "recall": 0.8032786885245902, "support": 122}, "weighted avg": {"f1-score": 0.7332972316578874, "precision": 0.840422568836543, "recall": 0.8032786885245902, "support": 122}, "\u2205": {"f1-score": 0.8864864864864864, "precision": 0.7961165048543689, "recall": 1.0, "support": 82}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.14814814814814814, "precision": 1.0, "recall": 0.08, "support": 25}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9333333333333333, "precision": 0.875, "recall": 1.0, "support": 14}, "macro avg": {"f1-score": 0.43648839297165576, "precision": 0.6677791262135923, "recall": 0.428030303030303, "support": 191}, "micro avg": {"f1-score": 0.6261980830670926, "precision": 0.8032786885245902, "recall": 0.5130890052356021, "support": 191}, "weighted avg": {"f1-score": 0.5557559727294763, "precision": 0.8071849743303005, "recall": 0.5130890052356021, "support": 191}, "\u2205": {"f1-score": 0.7256637168141592, "precision": 0.7961165048543689, "recall": 0.6666666666666666, "support": 123}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u2423": {"f1-score": 0.08695652173913045, "precision": 1.0, "recall": 0.045454545454545456, "support": 44}},
  "ppcr": 0.6387434554973822
}
```
</details>
