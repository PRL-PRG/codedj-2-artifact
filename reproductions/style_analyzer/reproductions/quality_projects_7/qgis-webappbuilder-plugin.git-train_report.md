# Train report for javascript / file:///tmp/top-repos-quality-repos-niet5311/qgis-webappbuilder-plugin.git HEAD ceed00caa091ca875feef624c1cf4a12bb23cda4

### Classification report

PPCR: 0.689

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.959| 0.995| 0.840| 0.977| 0.895| 12514| 14833| 0.844 |
| `␣` | 0.988| 0.916| 0.553| 0.951| 0.710| 4026| 6665| 0.604 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.906| 0.998| 0.956| 0.950| 0.930| 454| 474| 0.958 |
| `⏎⏎` | 0.997| 0.992| 0.844| 0.994| 0.914| 359| 422| 0.851 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 226| 813| 0.278 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 49| 0.020 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1185| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 609| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 469| 0.000 |
| `micro avg` | 0.964| 0.964| 0.664| 0.964| 0.787| 17580| 25519| 0.689 |
| `weighted avg` | 0.953| 0.964| 0.664| 0.958| 0.738| 17580| 25519| 0.689 |
| `macro avg` | 0.428| 0.433| 0.355| 0.430| 0.383| 17580| 25519| 0.689 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁻␣⁻␣⁻␣⁻| '| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎⏎| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2319 |12456 |11 |0 |0 |0 |0 |47 |0 |0 |
|2639 |337 |3689 |0 |0 |0 |0 |0 |0 |0 |
|1185 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|587 |196 |29 |0 |0 |0 |0 |0 |1 |0 |
|609 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|469 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|20 |1 |0 |0 |0 |0 |0 |453 |0 |0 |
|63 |0 |3 |0 |0 |0 |0 |0 |356 |0 |
|48 |1 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| webappbuilder/tests/expected/symbologycluster-2.14.js | 113 |
| webappbuilder/tests/expected/symbologycluster.js | 99 |
| webappbuilder/tests/expected/symbologypoints.js | 88 |
| webappbuilder/tests/expected/symbologypoints-2.14.js | 86 |
| webappbuilder/js/qgis2web_expressions.js | 66 |
| webappbuilder/tests/expected/symbologypolygons.js | 44 |
| webappbuilder/tests/expected/layers_locallines.js | 33 |
| webappbuilder/tests/expected/symbologylines.js | 32 |
| webappbuilder/tests/expected/nochartwidget.js | 23 |
| webappbuilder/tests/expected/symbologysimplelabels.js | 16 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.4302074336698839, "precision": 0.42783272782250226, "recall": 0.43345556558650256, "support": 17580}, "micro avg": {"f1-score": 0.964391353811149, "precision": 0.964391353811149, "recall": 0.964391353811149, "support": 17580}, "weighted avg": {"f1-score": 0.9579065155617937, "precision": 0.9526493129834387, "recall": 0.964391353811149, "support": 17580}, "\u2205": {"f1-score": 0.9767496569300137, "precision": 0.9588176429836041, "recall": 0.9953651909860955, "support": 12514}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 226}, "\u23ce\u23ce": {"f1-score": 0.994413407821229, "precision": 0.9971988795518207, "recall": 0.9916434540389972, "support": 359}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.949685534591195, "precision": 0.906, "recall": 0.9977973568281938, "support": 454}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9510183036865172, "precision": 0.9884780278670954, "recall": 0.916294088425236, "support": 4026}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1185}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 469}, "macro avg": {"f1-score": 0.38323855896117576, "precision": 0.42783272782250226, "recall": 0.3547261864674037, "support": 25519}, "micro avg": {"f1-score": 0.786746792268962, "precision": 0.964391353811149, "recall": 0.6643677260080724, "support": 25519}, "weighted avg": {"f1-score": 0.7381516361652716, "precision": 0.8488032478263591, "recall": 0.6643677260080724, "support": 25519}, "\u2205": {"f1-score": 0.8953421506612995, "precision": 0.9588176429836041, "recall": 0.8397492078473674, "support": 14833}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 813}, "\u23ce\u23ce": {"f1-score": 0.9139922978177151, "precision": 0.9971988795518207, "recall": 0.8436018957345972, "support": 422}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9301848049281315, "precision": 0.906, "recall": 0.9556962025316456, "support": 474}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 49}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 609}, "\u2423": {"f1-score": 0.7096277772434356, "precision": 0.9884780278670954, "recall": 0.5534883720930233, "support": 6665}},
  "ppcr": 0.6888984678082997
}
```
</details>
