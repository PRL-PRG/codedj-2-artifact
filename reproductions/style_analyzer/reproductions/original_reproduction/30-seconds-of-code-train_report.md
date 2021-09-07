# Train report for javascript / file:///tmp/top-repos-quality-repos-2279v_7n/30-seconds-of-code HEAD 3a122c9cfcbdc091227879a06a32bc67ccd0d35d

### Classification report

PPCR: 1.000

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.938| 0.973| 0.973| 0.955| 0.955| 37858| 37858| 1.000 |
| `␣` | 0.915| 0.901| 0.901| 0.908| 0.908| 17329| 17329| 1.000 |
| `'` | 1.000| 1.000| 1.000| 1.000| 1.000| 6248| 6248| 1.000 |
| `⏎` | 0.911| 0.810| 0.810| 0.858| 0.858| 3154| 3154| 1.000 |
| `⏎␣⁺␣⁺` | 0.926| 0.746| 0.746| 0.826| 0.826| 1888| 1888| 1.000 |
| `⏎␣⁻␣⁻` | 0.999| 0.777| 0.777| 0.874| 0.874| 1778| 1778| 1.000 |
| `⏎⏎` | 0.853| 0.748| 0.748| 0.797| 0.797| 457| 457| 1.000 |
| `weighted avg` | 0.937| 0.937| 0.937| 0.936| 0.936| 68712| 68712| 1.000 |
| `macro avg` | 0.935| 0.851| 0.851| 0.888| 0.888| 68712| 68712| 1.000 |
| `micro avg` | 0.937| 0.937| 0.937| 0.937| 0.937| 68712| 68712| 1.000 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|
|36852 |916 |0 |0 |90 |0 |0 |
|1677 |15612 |0 |17 |23 |0 |0 |
|0 |0 |6248 |0 |0 |0 |0 |
|185 |359 |0 |2556 |0 |0 |54 |
|380 |99 |0 |0 |1408 |1 |0 |
|187 |75 |0 |129 |0 |1382 |5 |
|11 |1 |0 |103 |0 |0 |342 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| scripts/web.js | 281 |
| scripts/util.js | 139 |
| scripts/tdd.js | 66 |
| test/hexToRGB/hexToRGB.js | 62 |
| scripts/extract.js | 62 |
| scripts/localize.js | 56 |
| scripts/tag.js | 52 |
| scripts/analyze.js | 45 |
| scripts/rollup.js | 44 |
| test/levenshteinDistance/levenshteinDistance.js | 43 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 6248}, "macro avg": {"f1-score": 0.8883951573726222, "precision": 0.934571164657843, "recall": 0.8508776372585477, "support": 68712}, "micro avg": {"f1-score": 0.9372453137734311, "precision": 0.9372453137734311, "recall": 0.9372453137734311, "support": 68712}, "weighted avg": {"f1-score": 0.9362633285075166, "precision": 0.9372393182797057, "recall": 0.9372453137734311, "support": 68712}, "\u2205": {"f1-score": 0.9553337653920934, "precision": 0.9379008449557161, "recall": 0.9734270167467907, "support": 37858}, "\u23ce": {"f1-score": 0.85786205739218, "precision": 0.9112299465240642, "recall": 0.8103994927076728, "support": 3154}, "\u23ce\u23ce": {"f1-score": 0.7972027972027972, "precision": 0.8528678304239401, "recall": 0.7483588621444202, "support": 457}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8260486946318568, "precision": 0.925706771860618, "recall": 0.7457627118644068, "support": 1888}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8744068332806075, "precision": 0.9992769342010123, "recall": 0.7772778402699663, "support": 1778}, "\u2423": {"f1-score": 0.9079119537088193, "precision": 0.9150158246395499, "recall": 0.9009175370765768, "support": 17329}},
  "cl_report_full": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 6248}, "macro avg": {"f1-score": 0.8883951573726222, "precision": 0.934571164657843, "recall": 0.8508776372585477, "support": 68712}, "micro avg": {"f1-score": 0.9372453137734311, "precision": 0.9372453137734311, "recall": 0.9372453137734311, "support": 68712}, "weighted avg": {"f1-score": 0.9362633285075166, "precision": 0.9372393182797057, "recall": 0.9372453137734311, "support": 68712}, "\u2205": {"f1-score": 0.9553337653920934, "precision": 0.9379008449557161, "recall": 0.9734270167467907, "support": 37858}, "\u23ce": {"f1-score": 0.85786205739218, "precision": 0.9112299465240642, "recall": 0.8103994927076728, "support": 3154}, "\u23ce\u23ce": {"f1-score": 0.7972027972027972, "precision": 0.8528678304239401, "recall": 0.7483588621444202, "support": 457}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8260486946318568, "precision": 0.925706771860618, "recall": 0.7457627118644068, "support": 1888}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8744068332806075, "precision": 0.9992769342010123, "recall": 0.7772778402699663, "support": 1778}, "\u2423": {"f1-score": 0.9079119537088193, "precision": 0.9150158246395499, "recall": 0.9009175370765768, "support": 17329}},
  "ppcr": 1.0
}
```
</details>
