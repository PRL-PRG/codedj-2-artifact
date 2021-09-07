# Train report for javascript / file:///tmp/top-repos-quality-repos-n68j2a10/yumi-s-house.git HEAD ed27ccb20ee1d10d35ed0264d74445b43188343c

### Classification report

PPCR: 0.243

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.987| 1.000| 0.285| 0.993| 0.443| 673| 2359| 0.285 |
| `␣` | 1.000| 1.000| 0.202| 1.000| 0.337| 235| 1161| 0.202 |
| `'` | 1.000| 1.000| 0.495| 1.000| 0.663| 215| 434| 0.495 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 270| 0.019 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 161| 0.019 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 153| 0.007 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 124| 0.000 |
| `macro avg` | 0.427| 0.429| 0.140| 0.428| 0.206| 1132| 4662| 0.243 |
| `micro avg` | 0.992| 0.992| 0.241| 0.992| 0.388| 1132| 4662| 0.243 |
| `weighted avg` | 0.984| 0.992| 0.241| 0.988| 0.369| 1132| 4662| 0.243 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1686 |673 |0 |0 |0 |0 |0 |0 |
|926 |0 |235 |0 |0 |0 |0 |0 |
|219 |0 |0 |215 |0 |0 |0 |0 |
|265 |5 |0 |0 |0 |0 |0 |0 |
|158 |3 |0 |0 |0 |0 |0 |0 |
|152 |1 |0 |0 |0 |0 |0 |0 |
|124 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| server/db/models/user.js | 4 |
| server/auth/google.js | 3 |
| server/api/users.spec.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 215}, "macro avg": {"f1-score": 0.4276225619399051, "precision": 0.42668621700879766, "recall": 0.42857142857142855, "support": 1132}, "micro avg": {"f1-score": 0.9920494699646644, "precision": 0.9920494699646644, "recall": 0.9920494699646644, "support": 1132}, "weighted avg": {"f1-score": 0.9881006089212836, "precision": 0.9842038589473902, "recall": 0.9920494699646644, "support": 1132}, "\u2205": {"f1-score": 0.9933579335793358, "precision": 0.9868035190615836, "recall": 1.0, "support": 673}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 235}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6625577812018489, "precision": 1.0, "recall": 0.49539170506912444, "support": 434}, "macro avg": {"f1-score": 0.2059787941400065, "precision": 0.42668621700879766, "recall": 0.14044197091246477, "support": 4662}, "micro avg": {"f1-score": 0.38764238867794265, "precision": 0.9920494699646644, "recall": 0.24088374088374087, "support": 4662}, "weighted avg": {"f1-score": 0.3694907743324425, "precision": 0.8414563495208657, "recall": 0.24088374088374087, "support": 4662}, "\u2205": {"f1-score": 0.44261756001315367, "precision": 0.9868035190615836, "recall": 0.28529037727850787, "support": 2359}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 270}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 124}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 161}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 153}, "\u2423": {"f1-score": 0.336676217765043, "precision": 1.0, "recall": 0.20241171403962102, "support": 1161}},
  "ppcr": 0.24281424281424283
}
```
</details>
