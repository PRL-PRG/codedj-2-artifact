# Train report for javascript / file:///tmp/top-repos-quality-repos-tirec064/nodeciudemy.git HEAD 20606fce8a2305e2424235578ed09985e6642270

### Classification report

PPCR: 0.140

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.196| 1.000| 0.328| 354| 1802| 0.196 |
| `'` | 1.000| 1.000| 0.500| 1.000| 0.667| 142| 284| 0.500 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1001| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 136| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 123| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 117| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 91| 0.000 |
| `weighted avg` | 1.000| 1.000| 0.140| 1.000| 0.220| 496| 3554| 0.140 |
| `micro avg` | 1.000| 1.000| 0.140| 1.000| 0.245| 496| 3554| 0.140 |
| `macro avg` | 0.286| 0.286| 0.099| 0.286| 0.142| 496| 3554| 0.140 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1448 |354 |0 |0 |0 |0 |0 |0 |
|1001 |0 |0 |0 |0 |0 |0 |0 |
|142 |0 |0 |142 |0 |0 |0 |0 |
|136 |0 |0 |0 |0 |0 |0 |0 |
|123 |0 |0 |0 |0 |0 |0 |0 |
|117 |0 |0 |0 |0 |0 |0 |0 |
|91 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 142}, "macro avg": {"f1-score": 0.2857142857142857, "precision": 0.2857142857142857, "recall": 0.2857142857142857, "support": 496}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 496}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 496}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 354}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 284}, "macro avg": {"f1-score": 0.14215036664016253, "precision": 0.2857142857142857, "recall": 0.09949262723957508, "support": 3554}, "micro avg": {"f1-score": 0.24493827160493828, "precision": 1.0, "recall": 0.13956105796285875, "support": 3554}, "weighted avg": {"f1-score": 0.21977623095076265, "precision": 0.5869442881260551, "recall": 0.13956105796285875, "support": 3554}, "\u2205": {"f1-score": 0.3283858998144712, "precision": 1.0, "recall": 0.19644839067702552, "support": 1802}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 136}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 91}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 123}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 117}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1001}},
  "ppcr": 0.13956105796285875
}
```
</details>
