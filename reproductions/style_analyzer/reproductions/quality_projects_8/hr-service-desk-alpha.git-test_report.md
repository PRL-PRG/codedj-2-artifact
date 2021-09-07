# Test report for javascript / file:///tmp/top-repos-quality-repos-ar1ujtb6/hr-service-desk-alpha.git HEAD 784cd966b08dfff7c4f97bec6bdb43338255af43

### Classification report

PPCR: 0.926

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.912| 0.995| 0.976| 0.952| 0.943| 367| 374| 0.981 |
| `␣` | 0.901| 0.840| 0.794| 0.870| 0.844| 119| 126| 0.944 |
| `'` | 1.000| 0.783| 0.783| 0.878| 0.878| 46| 46| 1.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 25| 0.200 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 7| 0.571 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 6| 0.500 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 7| 0.429 |
| `weighted avg` | 0.892| 0.916| 0.848| 0.902| 0.845| 547| 591| 0.926 |
| `micro avg` | 0.916| 0.916| 0.848| 0.916| 0.880| 547| 591| 0.926 |
| `macro avg` | 0.402| 0.374| 0.365| 0.386| 0.381| 547| 591| 0.926 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|7 |365 |2 |0 |0 |0 |0 |0 |
|7 |19 |100 |0 |0 |0 |0 |0 |
|0 |10 |0 |36 |0 |0 |0 |0 |
|20 |2 |3 |0 |0 |0 |0 |0 |
|3 |1 |3 |0 |0 |0 |0 |0 |
|4 |3 |0 |0 |0 |0 |0 |0 |
|3 |0 |3 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.878048780487805, "precision": 1.0, "recall": 0.782608695652174, "support": 46}, "macro avg": {"f1-score": 0.38562487174022664, "precision": 0.40191441441441444, "recall": 0.3739278912607574, "support": 547}, "micro avg": {"f1-score": 0.9159049360146252, "precision": 0.9159049360146252, "recall": 0.9159049360146252, "support": 547}, "weighted avg": {"f1-score": 0.901580371208434, "precision": 0.8923120789894099, "recall": 0.9159049360146252, "support": 547}, "\u2205": {"f1-score": 0.9517601043024772, "precision": 0.9125, "recall": 0.9945504087193461, "support": 367}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u2423": {"f1-score": 0.8695652173913043, "precision": 0.9009009009009009, "recall": 0.8403361344537815, "support": 119}},
  "cl_report_full": {"\u0027": {"f1-score": 0.878048780487805, "precision": 1.0, "recall": 0.782608695652174, "support": 46}, "macro avg": {"f1-score": 0.3807261559726073, "precision": 0.40191441441441444, "recall": 0.3645993311685675, "support": 591}, "micro avg": {"f1-score": 0.8804920913884008, "precision": 0.9159049360146252, "recall": 0.8477157360406091, "support": 591}, "weighted avg": {"f1-score": 0.845107235049653, "precision": 0.8473578908858097, "recall": 0.8477157360406091, "support": 591}, "\u2205": {"f1-score": 0.9431524547803617, "precision": 0.9125, "recall": 0.9759358288770054, "support": 374}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 25}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u2423": {"f1-score": 0.8438818565400843, "precision": 0.9009009009009009, "recall": 0.7936507936507936, "support": 126}},
  "ppcr": 0.9255499153976311
}
```
</details>
