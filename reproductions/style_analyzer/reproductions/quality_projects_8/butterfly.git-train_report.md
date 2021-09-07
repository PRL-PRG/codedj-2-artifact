# Train report for javascript / file:///tmp/top-repos-quality-repos-j7qjpcaa/butterfly.git HEAD 5fa9f561ce4cae64997e2a90be2284b239c72fe4

### Classification report

PPCR: 0.045

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `"` | 1.000| 1.000| 0.500| 1.000| 0.667| 448| 896| 0.500 |
| `∅` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 6252| 0.000 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1736| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 560| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 248| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 201| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 108| 0.000 |
| `weighted avg` | 1.000| 1.000| 0.045| 1.000| 0.060| 448| 10001| 0.045 |
| `micro avg` | 1.000| 1.000| 0.045| 1.000| 0.086| 448| 10001| 0.045 |
| `macro avg` | 0.143| 0.143| 0.071| 0.143| 0.095| 448| 10001| 0.045 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|6252 |0 |0 |0 |0 |0 |0 |0 |
|1736 |0 |0 |0 |0 |0 |0 |0 |
|448 |0 |0 |448 |0 |0 |0 |0 |
|560 |0 |0 |0 |0 |0 |0 |0 |
|248 |0 |0 |0 |0 |0 |0 |0 |
|201 |0 |0 |0 |0 |0 |0 |0 |
|108 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 448}, "macro avg": {"f1-score": 0.14285714285714285, "precision": 0.14285714285714285, "recall": 0.14285714285714285, "support": 448}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 448}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 448}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\"": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 896}, "macro avg": {"f1-score": 0.09523809523809523, "precision": 0.14285714285714285, "recall": 0.07142857142857142, "support": 10001}, "micro avg": {"f1-score": 0.08574983251985838, "precision": 1.0, "recall": 0.044795520447955206, "support": 10001}, "weighted avg": {"f1-score": 0.0597273605972736, "precision": 0.08959104089591041, "recall": 0.044795520447955206, "support": 10001}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6252}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 560}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 108}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 248}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 201}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1736}},
  "ppcr": 0.044795520447955206
}
```
</details>
