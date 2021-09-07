# Train report for javascript / file:///tmp/top-repos-quality-repos-g9gjbqzi/divergent-prototype.git HEAD 04bad10fa5da4d1906d6be023940d6fb9a0c27ed

### Classification report

PPCR: 0.034

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `"` | 1.000| 1.000| 0.430| 1.000| 0.602| 197| 458| 0.430 |
| `∅` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 3387| 0.000 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1281| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 376| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 349| 0.000 |
| `weighted avg` | 1.000| 1.000| 0.034| 1.000| 0.047| 197| 5851| 0.034 |
| `micro avg` | 1.000| 1.000| 0.034| 1.000| 0.065| 197| 5851| 0.034 |
| `macro avg` | 0.200| 0.200| 0.086| 0.200| 0.120| 197| 5851| 0.034 |

### Confusion matrix

|refusal|  ∅| ␣| "| '| ⏎| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|3387 |0 |0 |0 |0 |0 |
|1281 |0 |0 |0 |0 |0 |
|261 |0 |0 |197 |0 |0 |
|376 |0 |0 |0 |0 |0 |
|349 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 197}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.2, "precision": 0.2, "recall": 0.2, "support": 197}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 197}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 197}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\"": {"f1-score": 0.6015267175572518, "precision": 1.0, "recall": 0.43013100436681223, "support": 458}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 376}, "macro avg": {"f1-score": 0.12030534351145036, "precision": 0.2, "recall": 0.08602620087336245, "support": 5851}, "micro avg": {"f1-score": 0.06514550264550265, "precision": 1.0, "recall": 0.03366945821227141, "support": 5851}, "weighted avg": {"f1-score": 0.04708583774418413, "precision": 0.07827721756964622, "recall": 0.03366945821227141, "support": 5851}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3387}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 349}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1281}},
  "ppcr": 0.03366945821227141
}
```
</details>
