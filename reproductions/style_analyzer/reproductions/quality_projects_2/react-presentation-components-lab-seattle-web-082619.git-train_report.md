# Train report for javascript / file:///tmp/top-repos-quality-repos-sq7v4158/react-presentation-components-lab-seattle-web-082619.git HEAD c3f7e0e8ac4c2d493fcf58796f5b4f7a802691ec

### Classification report

PPCR: 0.189

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.336| 1.000| 0.503| 172| 512| 0.336 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 241| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 155| 0.000 |
| `weighted avg` | 1.000| 1.000| 0.189| 1.000| 0.284| 172| 908| 0.189 |
| `micro avg` | 1.000| 1.000| 0.189| 1.000| 0.319| 172| 908| 0.189 |
| `macro avg` | 0.333| 0.333| 0.112| 0.333| 0.168| 172| 908| 0.189 |

### Confusion matrix

|refusal|  ∅| ␣| '| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|340 |172 |0 |0 |
|241 |0 |0 |0 |
|155 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.3333333333333333, "precision": 0.3333333333333333, "recall": 0.3333333333333333, "support": 172}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 172}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 172}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 172}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 155}, "macro avg": {"f1-score": 0.16764132553606237, "precision": 0.3333333333333333, "recall": 0.11197916666666667, "support": 908}, "micro avg": {"f1-score": 0.31851851851851853, "precision": 1.0, "recall": 0.1894273127753304, "support": 908}, "weighted avg": {"f1-score": 0.28358708813148875, "precision": 0.5638766519823789, "recall": 0.1894273127753304, "support": 908}, "\u2205": {"f1-score": 0.5029239766081871, "precision": 1.0, "recall": 0.3359375, "support": 512}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 241}},
  "ppcr": 0.1894273127753304
}
```
</details>
