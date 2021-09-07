# Test report for javascript / file:///tmp/top-repos-quality-repos-0vofpx0m/contact-list-app.git HEAD d2d9c26d20f6e8242f8ada0803628a01ea89e263

### Classification report

PPCR: 0.305

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.896| 1.000| 0.491| 0.945| 0.635| 112| 228| 0.491 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 48| 0.104 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 105| 0.038 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 14| 0.143 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 15| 0.133 |
| `micro avg` | 0.896| 0.896| 0.273| 0.896| 0.419| 125| 410| 0.305 |
| `weighted avg` | 0.803| 0.896| 0.273| 0.847| 0.353| 125| 410| 0.305 |
| `macro avg` | 0.179| 0.200| 0.098| 0.189| 0.127| 125| 410| 0.305 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|116 |112 |0 |0 |0 |0 |
|101 |4 |0 |0 |0 |0 |
|43 |5 |0 |0 |0 |0 |
|12 |2 |0 |0 |0 |0 |
|13 |2 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "macro avg": {"f1-score": 0.18902953586497892, "precision": 0.1792, "recall": 0.2, "support": 125}, "micro avg": {"f1-score": 0.8960000000000001, "precision": 0.896, "recall": 0.896, "support": 125}, "weighted avg": {"f1-score": 0.8468523206751056, "precision": 0.8028160000000001, "recall": 0.896, "support": 125}, "\u2205": {"f1-score": 0.9451476793248946, "precision": 0.896, "recall": 1.0, "support": 112}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 48}, "macro avg": {"f1-score": 0.12691218130311613, "precision": 0.1792, "recall": 0.09824561403508772, "support": 410}, "micro avg": {"f1-score": 0.4186915887850467, "precision": 0.896, "recall": 0.2731707317073171, "support": 410}, "weighted avg": {"f1-score": 0.3528777724037863, "precision": 0.49826341463414636, "recall": 0.2731707317073171, "support": 410}, "\u2205": {"f1-score": 0.6345609065155806, "precision": 0.896, "recall": 0.49122807017543857, "support": 228}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 15}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 105}},
  "ppcr": 0.3048780487804878
}
```
</details>
