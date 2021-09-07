# Test report for javascript / file:///tmp/top-repos-quality-repos-i8r3erp2/map-state-to-props-lab-nyc-web-082619.git HEAD ead61ff840ae7ec0dc18405f0d2263f9edc4b2b2

### Classification report

PPCR: 0.220

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.868| 1.000| 0.284| 0.930| 0.429| 33| 116| 0.284 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 37| 0.081 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 10| 0.100 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 10| 0.100 |
| `micro avg` | 0.868| 0.868| 0.191| 0.868| 0.313| 38| 173| 0.220 |
| `weighted avg` | 0.754| 0.868| 0.191| 0.807| 0.287| 38| 173| 0.220 |
| `macro avg` | 0.217| 0.250| 0.071| 0.232| 0.107| 38| 173| 0.220 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|83 |33 |0 |0 |0 |
|34 |3 |0 |0 |0 |
|9 |1 |0 |0 |0 |
|9 |1 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "macro avg": {"f1-score": 0.2323943661971831, "precision": 0.21710526315789475, "recall": 0.25, "support": 38}, "micro avg": {"f1-score": 0.868421052631579, "precision": 0.868421052631579, "recall": 0.868421052631579, "support": 38}, "weighted avg": {"f1-score": 0.8072646404744255, "precision": 0.7541551246537397, "recall": 0.868421052631579, "support": 38}, "\u2205": {"f1-score": 0.9295774647887324, "precision": 0.868421052631579, "recall": 1.0, "support": 33}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "macro avg": {"f1-score": 0.10714285714285715, "precision": 0.21710526315789475, "recall": 0.07112068965517242, "support": 173}, "micro avg": {"f1-score": 0.3127962085308057, "precision": 0.868421052631579, "recall": 0.1907514450867052, "support": 173}, "weighted avg": {"f1-score": 0.2873658133773741, "precision": 0.5822938850015211, "recall": 0.1907514450867052, "support": 173}, "\u2205": {"f1-score": 0.4285714285714286, "precision": 0.868421052631579, "recall": 0.28448275862068967, "support": 116}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 37}},
  "ppcr": 0.21965317919075145
}
```
</details>
