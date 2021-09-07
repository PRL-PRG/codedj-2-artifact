# Train report for javascript / file:///tmp/top-repos-quality-repos-lnub4lqn/react-video-recorder.git HEAD dc441024546cfe9862e5e4a8bc0ea3a72840833e

### Classification report

PPCR: 0.200

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.982| 1.000| 0.357| 0.991| 0.523| 986| 2763| 0.357 |
| `⏎` | 0.949| 0.949| 0.309| 0.949| 0.467| 136| 417| 0.326 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 231| 0.035 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 250| 0.016 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 186| 0.022 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 1517| 0.001 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 336| 0.000 |
| `micro avg` | 0.978| 0.978| 0.196| 0.978| 0.326| 1140| 5700| 0.200 |
| `weighted avg` | 0.963| 0.978| 0.196| 0.970| 0.288| 1140| 5700| 0.200 |
| `macro avg` | 0.276| 0.278| 0.095| 0.277| 0.141| 1140| 5700| 0.200 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1777 |986 |0 |0 |0 |0 |0 |0 |
|1515 |1 |0 |1 |0 |0 |0 |0 |
|281 |7 |0 |129 |0 |0 |0 |0 |
|336 |0 |0 |0 |0 |0 |0 |0 |
|246 |4 |0 |0 |0 |0 |0 |0 |
|223 |6 |0 |2 |0 |0 |0 |0 |
|182 |0 |0 |4 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/video-recorder.js | 18 |
| src/get-video-info.js | 4 |
| test/browser/setup.js | 2 |
| src/defaults/render-actions.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.2770691693762932, "precision": 0.2758001607017309, "recall": 0.27836134453781514, "support": 1140}, "micro avg": {"f1-score": 0.9780701754385965, "precision": 0.9780701754385965, "recall": 0.9780701754385965, "support": 1140}, "weighted avg": {"f1-score": 0.9702468482764699, "precision": 0.9625637799678479, "recall": 0.9780701754385965, "support": 1140}, "\u2205": {"f1-score": 0.9909547738693467, "precision": 0.9820717131474104, "recall": 1.0, "support": 986}, "\u23ce": {"f1-score": 0.9485294117647058, "precision": 0.9485294117647058, "recall": 0.9485294117647058, "support": 136}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 336}, "macro avg": {"f1-score": 0.14143422975235936, "precision": 0.2758001607017309, "recall": 0.09517300073389404, "support": 5700}, "micro avg": {"f1-score": 0.32602339181286544, "precision": 0.9780701754385965, "recall": 0.1956140350877193, "support": 5700}, "weighted avg": {"f1-score": 0.28788811554682003, "precision": 0.5454387558126627, "recall": 0.1956140350877193, "support": 5700}, "\u2205": {"f1-score": 0.5234934961507831, "precision": 0.9820717131474104, "recall": 0.35685848715164675, "support": 2763}, "\u23ce": {"f1-score": 0.4665461121157323, "precision": 0.9485294117647058, "recall": 0.30935251798561153, "support": 417}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 186}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 250}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 231}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1517}},
  "ppcr": 0.2
}
```
</details>
