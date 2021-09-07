# Test report for javascript / file:///tmp/top-repos-quality-repos-gil9c1pi/2019-2-atom-frontend-n-tarasov.git HEAD 8fa9ccf47a3d6aea525cda63e7dfde81b5c2d673

### Classification report

PPCR: 0.665

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.957| 0.989| 0.813| 0.973| 0.879| 1030| 1254| 0.821 |
| `␣` | 0.957| 0.890| 0.544| 0.922| 0.694| 327| 535| 0.611 |
| `'` | 0.928| 0.947| 0.500| 0.937| 0.650| 95| 180| 0.528 |
| `⏎⇥⁺` | 0.821| 0.852| 0.605| 0.836| 0.697| 27| 38| 0.711 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 14| 0.643 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 156| 0.026 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 32| 0.062 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 36| 0.000 |
| `macro avg` | 0.458| 0.460| 0.308| 0.459| 0.365| 1494| 2245| 0.665 |
| `micro avg` | 0.952| 0.952| 0.634| 0.952| 0.761| 1494| 2245| 0.665 |
| `weighted avg` | 0.943| 0.952| 0.634| 0.947| 0.720| 1494| 2245| 0.665 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⇥⁺| ⏎⇥⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|224 |1019 |11 |0 |0 |0 |0 |0 |0 |
|208 |31 |291 |0 |0 |5 |0 |0 |0 |
|85 |3 |2 |90 |0 |0 |0 |0 |0 |
|152 |4 |0 |0 |0 |0 |0 |0 |0 |
|11 |4 |0 |0 |0 |23 |0 |0 |0 |
|30 |2 |0 |0 |0 |0 |0 |0 |0 |
|36 |0 |0 |0 |0 |0 |0 |0 |0 |
|5 |2 |0 |7 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u0027": {"f1-score": 0.9374999999999999, "precision": 0.9278350515463918, "recall": 0.9473684210526315, "support": 95}, "macro avg": {"f1-score": 0.45862518531148494, "precision": 0.45791349710216445, "recall": 0.4598061147668415, "support": 1494}, "micro avg": {"f1-score": 0.9524765729585006, "precision": 0.9524765729585006, "recall": 0.9524765729585006, "support": 1494}, "weighted avg": {"f1-score": 0.9472740461124542, "precision": 0.9430060815147936, "recall": 0.9524765729585006, "support": 1494}, "\u2205": {"f1-score": 0.9727923627684965, "precision": 0.9568075117370892, "recall": 0.9893203883495145, "support": 1030}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u21e5\u207a": {"f1-score": 0.8363636363636364, "precision": 0.8214285714285714, "recall": 0.8518518518518519, "support": 27}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9223454833597465, "precision": 0.9572368421052632, "recall": 0.8899082568807339, "support": 327}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u0027": {"f1-score": 0.6498194945848376, "precision": 0.9278350515463918, "recall": 0.5, "support": 180}, "macro avg": {"f1-score": 0.36491240351157767, "precision": 0.45791349710216445, "recall": 0.30772350907004126, "support": 2245}, "micro avg": {"f1-score": 0.7611660871890878, "precision": 0.9524765729585006, "recall": 0.6338530066815145, "support": 2245}, "weighted avg": {"f1-score": 0.720098840996377, "precision": 0.8508609911969988, "recall": 0.6338530066815145, "support": 2245}, "\u2205": {"f1-score": 0.8788270806382061, "precision": 0.9568075117370892, "recall": 0.8125996810207337, "support": 1254}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 156}, "\u23ce\u21e5\u207a": {"f1-score": 0.6969696969696969, "precision": 0.8214285714285714, "recall": 0.6052631578947368, "support": 38}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 32}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 36}, "\u2423": {"f1-score": 0.6936829558998807, "precision": 0.9572368421052632, "recall": 0.5439252336448598, "support": 535}},
  "ppcr": 0.6654788418708241
}
```
</details>
