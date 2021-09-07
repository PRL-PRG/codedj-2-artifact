# Test report for javascript / file:///tmp/top-repos-quality-repos-8jjadn08/protractor-cucumber-slices.git HEAD bac7c77acc117b766e8d4aea80d5c57ed18e07a4

### Classification report

PPCR: 0.453

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.880| 1.000| 0.733| 0.936| 0.800| 344| 469| 0.733 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 41| 237| 0.173 |
| `'` | 1.000| 1.000| 0.432| 1.000| 0.603| 38| 88| 0.432 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 38| 0.132 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 33| 0.030 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 55| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 28| 0.000 |
| `weighted avg` | 0.794| 0.890| 0.403| 0.839| 0.452| 429| 948| 0.453 |
| `micro avg` | 0.890| 0.890| 0.403| 0.890| 0.555| 429| 948| 0.453 |
| `macro avg` | 0.269| 0.286| 0.166| 0.277| 0.200| 429| 948| 0.453 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|125 |344 |0 |0 |0 |0 |0 |0 |
|196 |41 |0 |0 |0 |0 |0 |0 |
|50 |0 |0 |38 |0 |0 |0 |0 |
|55 |0 |0 |0 |0 |0 |0 |0 |
|33 |5 |0 |0 |0 |0 |0 |0 |
|32 |1 |0 |0 |0 |0 |0 |0 |
|28 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 38}, "macro avg": {"f1-score": 0.2765792031098154, "precision": 0.26854219948849106, "recall": 0.2857142857142857, "support": 429}, "micro avg": {"f1-score": 0.8904428904428905, "precision": 0.8904428904428905, "recall": 0.8904428904428905, "support": 429}, "weighted avg": {"f1-score": 0.8391671820243248, "precision": 0.7940550498095255, "recall": 0.8904428904428905, "support": 429}, "\u2205": {"f1-score": 0.9360544217687075, "precision": 0.8797953964194374, "recall": 1.0, "support": 344}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 41}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6031746031746031, "precision": 1.0, "recall": 0.4318181818181818, "support": 88}, "macro avg": {"f1-score": 0.200453514739229, "precision": 0.26854219948849106, "recall": 0.1664705230803312, "support": 948}, "micro avg": {"f1-score": 0.5548293391430646, "precision": 0.8904428904428905, "recall": 0.4029535864978903, "support": 948}, "weighted avg": {"f1-score": 0.45177148215122903, "precision": 0.5280844313509664, "recall": 0.4029535864978903, "support": 948}, "\u2205": {"f1-score": 0.8, "precision": 0.8797953964194374, "recall": 0.7334754797441365, "support": 469}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 55}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 28}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 38}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 33}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 237}},
  "ppcr": 0.4525316455696203
}
```
</details>
