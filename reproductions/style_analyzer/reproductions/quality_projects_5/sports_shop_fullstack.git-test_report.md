# Test report for javascript / file:///tmp/top-repos-quality-repos-53piskwu/sports_shop_fullstack.git HEAD aaaf7d51e2998a0522951db75fa1e01d14e9e063

### Classification report

PPCR: 0.412

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.702| 1.000| 0.825| 40| 57| 0.702 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 4| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 3| 0.000 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 33| 0.000 |
| `micro avg` | 1.000| 1.000| 0.412| 1.000| 0.584| 40| 97| 0.412 |
| `macro avg` | 0.250| 0.250| 0.175| 0.250| 0.206| 40| 97| 0.412 |
| `weighted avg` | 1.000| 1.000| 0.412| 1.000| 0.485| 40| 97| 0.412 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|17 |40 |0 |0 |0 |
|33 |0 |0 |0 |0 |
|4 |0 |0 |0 |0 |
|3 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.25, "precision": 0.25, "recall": 0.25, "support": 40}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 40}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 40}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 40}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "macro avg": {"f1-score": 0.20618556701030927, "precision": 0.25, "recall": 0.17543859649122806, "support": 97}, "micro avg": {"f1-score": 0.5839416058394161, "precision": 1.0, "recall": 0.41237113402061853, "support": 97}, "weighted avg": {"f1-score": 0.48464236369433517, "precision": 0.5876288659793815, "recall": 0.41237113402061853, "support": 97}, "\u2205": {"f1-score": 0.8247422680412371, "precision": 1.0, "recall": 0.7017543859649122, "support": 57}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 33}},
  "ppcr": 0.41237113402061853
}
```
</details>
