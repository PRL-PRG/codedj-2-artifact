# Test report for javascript / file:///tmp/top-repos-quality-repos-oytmtc04/necsus.git HEAD d662dfec5b90476bb56c224d69430339344529ba

### Classification report

PPCR: 0.765

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.898| 0.914| 0.910| 0.906| 0.904| 222| 223| 0.996 |
| `␣` | 0.811| 0.876| 0.647| 0.843| 0.720| 113| 153| 0.739 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 16| 0.625 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 20| 0.100 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 20| 0.050 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 23| 0.000 |
| `micro avg` | 0.868| 0.868| 0.664| 0.868| 0.752| 348| 455| 0.765 |
| `weighted avg` | 0.837| 0.868| 0.664| 0.852| 0.685| 348| 455| 0.765 |
| `macro avg` | 0.285| 0.298| 0.260| 0.291| 0.271| 348| 455| 0.765 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| ⏎␣⁺␣⁺| '| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|1 |203 |19 |0 |0 |0 |0 |
|40 |14 |99 |0 |0 |0 |0 |
|23 |0 |0 |0 |0 |0 |0 |
|18 |0 |2 |0 |0 |0 |0 |
|6 |8 |2 |0 |0 |0 |0 |
|19 |1 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "macro avg": {"f1-score": 0.2914671985815603, "precision": 0.2849509163886068, "recall": 0.29842010151744663, "support": 348}, "micro avg": {"f1-score": 0.867816091954023, "precision": 0.867816091954023, "recall": 0.867816091954023, "support": 348}, "weighted avg": {"f1-score": 0.8517126742479824, "precision": 0.8365051751652101, "recall": 0.867816091954023, "support": 348}, "\u2205": {"f1-score": 0.90625, "precision": 0.8982300884955752, "recall": 0.9144144144144144, "support": 222}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.8425531914893618, "precision": 0.8114754098360656, "recall": 0.8761061946902655, "support": 113}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "macro avg": {"f1-score": 0.27070527097253155, "precision": 0.2849509163886068, "recall": 0.25956212081245056, "support": 455}, "micro avg": {"f1-score": 0.7521793275217933, "precision": 0.867816091954023, "recall": 0.6637362637362637, "support": 455}, "weighted avg": {"f1-score": 0.6852827528818621, "precision": 0.7131012031635853, "recall": 0.6637362637362637, "support": 455}, "\u2205": {"f1-score": 0.9042316258351893, "precision": 0.8982300884955752, "recall": 0.9103139013452914, "support": 223}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 23}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 20}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 20}, "\u2423": {"f1-score": 0.7200000000000001, "precision": 0.8114754098360656, "recall": 0.6470588235294118, "support": 153}},
  "ppcr": 0.7648351648351648
}
```
</details>
