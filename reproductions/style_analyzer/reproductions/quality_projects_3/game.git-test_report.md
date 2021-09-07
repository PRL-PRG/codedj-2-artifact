# Test report for javascript / file:///tmp/top-repos-quality-repos-rpiesur9/game.git HEAD e5691501199e1d7fa8679275bb79b9176899e3ed

### Classification report

PPCR: 0.374

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.999| 1.000| 0.526| 0.999| 0.689| 3375| 6419| 0.526 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 246| 0.008 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 648| 0.003 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1716| 0.000 |
| `macro avg` | 0.250| 0.250| 0.131| 0.250| 0.172| 3379| 9029| 0.374 |
| `weighted avg` | 0.998| 0.999| 0.374| 0.998| 0.490| 3379| 9029| 0.374 |
| `micro avg` | 0.999| 0.999| 0.374| 0.999| 0.544| 3379| 9029| 0.374 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|3044 |3375 |0 |0 |0 |
|1716 |0 |0 |0 |0 |
|244 |2 |0 |0 |0 |
|646 |2 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "macro avg": {"f1-score": 0.24985193959135327, "precision": 0.24970405445398047, "recall": 0.25, "support": 3379}, "micro avg": {"f1-score": 0.9988162178159219, "precision": 0.9988162178159219, "recall": 0.9988162178159219, "support": 3379}, "weighted avg": {"f1-score": 0.9982246772664307, "precision": 0.9976338369721031, "recall": 0.9988162178159219, "support": 3379}, "\u2205": {"f1-score": 0.9994077583654131, "precision": 0.9988162178159219, "recall": 1.0, "support": 3375}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 246}, "macro avg": {"f1-score": 0.17222902633190446, "precision": 0.24970405445398047, "recall": 0.13144570805421404, "support": 9029}, "micro avg": {"f1-score": 0.5440038684719535, "precision": 0.9988162178159219, "recall": 0.37379554767969875, "support": 9029}, "weighted avg": {"f1-score": 0.48977212095447764, "precision": 0.7100898551512241, "recall": 0.37379554767969875, "support": 9029}, "\u2205": {"f1-score": 0.6889161053276178, "precision": 0.9988162178159219, "recall": 0.5257828322168562, "support": 6419}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 648}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1716}},
  "ppcr": 0.3742385646250969
}
```
</details>
