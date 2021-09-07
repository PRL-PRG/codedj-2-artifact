# Train report for javascript / file:///tmp/top-repos-quality-repos-i8r3erp2/map-state-to-props-lab-nyc-web-082619.git HEAD ead61ff840ae7ec0dc18405f0d2263f9edc4b2b2

### Classification report

PPCR: 0.150

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.269| 1.000| 0.423| 243| 905| 0.269 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 389| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 218| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 109| 0.000 |
| `micro avg` | 1.000| 1.000| 0.150| 1.000| 0.261| 243| 1621| 0.150 |
| `weighted avg` | 1.000| 1.000| 0.150| 1.000| 0.236| 243| 1621| 0.150 |
| `macro avg` | 0.250| 0.250| 0.067| 0.250| 0.106| 243| 1621| 0.150 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|662 |243 |0 |0 |0 |
|389 |0 |0 |0 |0 |
|218 |0 |0 |0 |0 |
|109 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.25, "precision": 0.25, "recall": 0.25, "support": 243}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 243}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 243}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 243}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 218}, "macro avg": {"f1-score": 0.1058362369337979, "precision": 0.25, "recall": 0.06712707182320442, "support": 1621}, "micro avg": {"f1-score": 0.2607296137339056, "precision": 1.0, "recall": 0.1499074645280691, "support": 1621}, "weighted avg": {"f1-score": 0.23635236132038767, "precision": 0.5582973473164713, "recall": 0.1499074645280691, "support": 1621}, "\u2205": {"f1-score": 0.4233449477351916, "precision": 1.0, "recall": 0.26850828729281767, "support": 905}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 109}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 389}},
  "ppcr": 0.1499074645280691
}
```
</details>
