# Train report for javascript / file:///tmp/top-repos-quality-repos-idkzfw1t/guardioes-web.git HEAD 468d4f518ee2788021a0ef3d6af845b4b4eeb665

### Classification report

PPCR: 0.766

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.977| 0.990| 0.955| 0.983| 0.966| 34104| 35340| 0.965 |
| `␣` | 0.946| 0.975| 0.698| 0.960| 0.803| 10972| 15334| 0.716 |
| `"` | 0.957| 0.996| 0.702| 0.976| 0.810| 2789| 3956| 0.705 |
| `⏎⏎` | 0.966| 1.000| 0.509| 0.982| 0.667| 448| 880| 0.509 |
| `'` | 0.933| 0.572| 0.098| 0.709| 0.177| 290| 1696| 0.171 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 284| 995| 0.285 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 146| 965| 0.151 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 115| 385| 0.299 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 110| 3903| 0.028 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 74| 412| 0.180 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 56| 319| 0.176 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 37| 368| 0.101 |
| `weighted avg` | 0.952| 0.968| 0.741| 0.960| 0.783| 49425| 64553| 0.766 |
| `micro avg` | 0.968| 0.968| 0.741| 0.968| 0.840| 49425| 64553| 0.766 |
| `macro avg` | 0.398| 0.378| 0.247| 0.384| 0.285| 49425| 64553| 0.766 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| '| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| ⏎⏎| ⏎⇥⁺| ⏎⇥⁻| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1236 |33763 |341 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|4362 |272 |10699 |0 |0 |0 |0 |0 |1 |0 |0 |0 |0 |
|1167 |0 |0 |2777 |0 |12 |0 |0 |0 |0 |0 |0 |0 |
|3793 |42 |59 |0 |0 |0 |0 |0 |9 |0 |0 |0 |0 |
|1406 |0 |0 |124 |0 |166 |0 |0 |0 |0 |0 |0 |0 |
|711 |268 |16 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|819 |95 |51 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|432 |0 |0 |0 |0 |0 |0 |0 |448 |0 |0 |0 |0 |
|331 |1 |36 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|263 |2 |54 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|338 |31 |43 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|270 |101 |8 |0 |0 |0 |0 |0 |6 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/pages/Home/components/Groups/index.js | 159 |
| canvasjs-dashboard-samples/assets/economy-dashboard/economy-dashboard.js | 113 |
| src/pages/Home/components/GoData/index.js | 94 |
| src/pages/Home/components/GroupManagers/index.js | 70 |
| src/pages/Home/components/Syndromes/index.js | 67 |
| src/pages/Home/components/Contents/index.js | 51 |
| canvasjs-dashboard-samples/assets/web-analytics/real-time.js | 49 |
| src/pages/Home/components/Vigilance/index.js | 44 |
| src/pages/Home/components/Managers/index.js | 43 |
| src/pages/Home/components/Users/index.js | 43 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9760984182776802, "precision": 0.9572561185798001, "recall": 0.9956973825743994, "support": 2789}, "\u0027": {"f1-score": 0.7094017094017094, "precision": 0.9325842696629213, "recall": 0.5724137931034483, "support": 290}, "macro avg": {"f1-score": 0.3843020046741719, "precision": 0.3981750377353895, "recall": 0.37776923599775963, "support": 49425}, "micro avg": {"f1-score": 0.9681942336874052, "precision": 0.9681942336874052, "recall": 0.9681942336874052, "support": 49425}, "weighted avg": {"f1-score": 0.9597932742834712, "precision": 0.9521064970233466, "recall": 0.9681942336874052, "support": 49425}, "\u2205": {"f1-score": 0.983211753228789, "precision": 0.9765148228488792, "recall": 0.9900011728829463, "support": 34104}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 110}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 37}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 56}, "\u23ce\u23ce": {"f1-score": 0.9824561403508771, "precision": 0.9655172413793104, "recall": 1.0, "support": 448}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 146}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 74}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 284}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 115}, "\u2423": {"f1-score": 0.9604560348310068, "precision": 0.9462280003537632, "recall": 0.9751184834123223, "support": 10972}},
  "cl_report_full": {"\"": {"f1-score": 0.8099752078168295, "precision": 0.9572561185798001, "recall": 0.7019716885743175, "support": 3956}, "\u0027": {"f1-score": 0.17716115261472787, "precision": 0.9325842696629213, "recall": 0.09787735849056604, "support": 1696}, "macro avg": {"f1-score": 0.2852359201333106, "precision": 0.3981750377353895, "recall": 0.24683723614140674, "support": 64553}, "micro avg": {"f1-score": 0.8396883609117549, "precision": 0.9681942336874052, "recall": 0.7412978482797081, "support": 64553}, "weighted avg": {"f1-score": 0.7829235385595471, "precision": 0.8556955880558269, "recall": 0.7412978482797081, "support": 64553}, "\u2205": {"f1-score": 0.9658299363512836, "precision": 0.9765148228488792, "recall": 0.9553763440860215, "support": 35340}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3903}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 368}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 319}, "\u23ce\u23ce": {"f1-score": 0.6666666666666667, "precision": 0.9655172413793104, "recall": 0.509090909090909, "support": 880}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 965}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 412}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 995}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 385}, "\u2423": {"f1-score": 0.8031980781502195, "precision": 0.9462280003537632, "recall": 0.6977305334550672, "support": 15334}},
  "ppcr": 0.7656499310643967
}
```
</details>
