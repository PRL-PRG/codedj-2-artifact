# Train report for javascript / file:///tmp/top-repos-quality-repos-q1dj35ii/mechanical-squirrel.git HEAD 6dd0ba3bb0fa5636a5edfa9ed0ef80885e4f10c6

### Classification report

PPCR: 0.179

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.300| 1.000| 0.461| 143| 477| 0.300 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 207| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 115| 0.000 |
| `macro avg` | 0.333| 0.333| 0.100| 0.333| 0.154| 143| 799| 0.179 |
| `weighted avg` | 1.000| 1.000| 0.179| 1.000| 0.275| 143| 799| 0.179 |
| `micro avg` | 1.000| 1.000| 0.179| 1.000| 0.304| 143| 799| 0.179 |

### Confusion matrix

|refusal|  ∅| ␣| '| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|334 |143 |0 |0 |
|207 |0 |0 |0 |
|115 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.3333333333333333, "precision": 0.3333333333333333, "recall": 0.3333333333333333, "support": 143}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 143}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 143}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 143}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 115}, "macro avg": {"f1-score": 0.15376344086021507, "precision": 0.3333333333333333, "recall": 0.09993011879804332, "support": 799}, "micro avg": {"f1-score": 0.3036093418259023, "precision": 1.0, "recall": 0.17897371714643304, "support": 799}, "weighted avg": {"f1-score": 0.2753885905769308, "precision": 0.5969962453066333, "recall": 0.17897371714643304, "support": 799}, "\u2205": {"f1-score": 0.4612903225806452, "precision": 1.0, "recall": 0.29979035639413, "support": 477}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 207}},
  "ppcr": 0.17897371714643304
}
```
</details>
