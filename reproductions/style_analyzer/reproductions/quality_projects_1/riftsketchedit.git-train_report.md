# Train report for javascript / file:///tmp/top-repos-quality-repos-a7lhypiv/riftsketchedit.git HEAD e3e45f52f5b044dc8d2ebd3ce5c6bc6f88b6b0cf

### Classification report

PPCR: 0.355

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.982| 0.991| 0.497| 0.986| 0.660| 2307| 4598| 0.502 |
| `␣` | 0.933| 0.933| 0.120| 0.933| 0.213| 240| 1865| 0.129 |
| `⏎␣⁺␣⁺` | 0.910| 0.995| 0.815| 0.951| 0.860| 213| 260| 0.819 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 39| 259| 0.151 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 449| 0.007 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 290| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 167| 0.000 |
| `macro avg` | 0.404| 0.417| 0.205| 0.410| 0.248| 2802| 7888| 0.355 |
| `micro avg` | 0.971| 0.971| 0.345| 0.971| 0.509| 2802| 7888| 0.355 |
| `weighted avg` | 0.957| 0.971| 0.345| 0.964| 0.463| 2802| 7888| 0.355 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|2291 |2286 |16 |0 |0 |5 |0 |0 |
|1625 |1 |224 |0 |0 |15 |0 |0 |
|446 |2 |0 |0 |0 |1 |0 |0 |
|290 |0 |0 |0 |0 |0 |0 |0 |
|47 |1 |0 |0 |0 |212 |0 |0 |
|220 |39 |0 |0 |0 |0 |0 |0 |
|167 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| js/KeyboardHandler.js | 30 |
| js/SketchController.js | 12 |
| test/runner/html.js | 8 |
| js/TextArea.js | 6 |
| js/RiftSandbox.js | 6 |
| webpack.config.js | 5 |
| js/Files/World.js | 4 |
| js/Files/Boid.js | 3 |
| js/Files/Behaviors.js | 3 |
| js/Sketch.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.4100287106798829, "precision": 0.40353453119601895, "recall": 0.4170765381190481, "support": 2802}, "micro avg": {"f1-score": 0.9714489650249821, "precision": 0.9714489650249821, "recall": 0.9714489650249821, "support": 2802}, "weighted avg": {"f1-score": 0.9641845569557658, "precision": 0.9572479507560373, "recall": 0.9714489650249821, "support": 2802}, "\u2205": {"f1-score": 0.9861949956859363, "precision": 0.9815371404036067, "recall": 0.9908972691807543, "support": 2307}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9506726457399104, "precision": 0.9098712446351931, "recall": 0.9953051643192489, "support": 213}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 39}, "\u2423": {"f1-score": 0.9333333333333333, "precision": 0.9333333333333333, "recall": 0.9333333333333333, "support": 240}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 290}, "macro avg": {"f1-score": 0.24755616522167787, "precision": 0.40353453119601895, "recall": 0.2046663625380097, "support": 7888}, "micro avg": {"f1-score": 0.5092609915809168, "precision": 0.9714489650249821, "recall": 0.3450811359026369, "support": 7888}, "weighted avg": {"f1-score": 0.46340411297714795, "precision": 0.8228119880638439, "recall": 0.3450811359026369, "support": 7888}, "\u2205": {"f1-score": 0.6600259852750109, "precision": 0.9815371404036067, "recall": 0.49717268377555457, "support": 4598}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 449}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 167}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8600405679513184, "precision": 0.9098712446351931, "recall": 0.8153846153846154, "support": 260}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 259}, "\u2423": {"f1-score": 0.21282660332541567, "precision": 0.9333333333333333, "recall": 0.12010723860589813, "support": 1865}},
  "ppcr": 0.3552231237322515
}
```
</details>
