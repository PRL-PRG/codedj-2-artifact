# Test report for javascript / file:///tmp/top-repos-quality-repos-k84oyq4k/sibi-activity-search-v3.git HEAD e1190faedef65d911c0c22cebe1cab1f542757dc

### Classification report

PPCR: 0.255

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.960| 0.993| 0.386| 0.976| 0.550| 580| 1493| 0.388 |
| `␣` | 0.807| 0.958| 0.117| 0.876| 0.205| 118| 965| 0.122 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 23| 241| 0.095 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 11| 12| 0.917 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 94| 0.064 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 95| 0.021 |
| `macro avg` | 0.295| 0.325| 0.084| 0.309| 0.126| 740| 2900| 0.255 |
| `weighted avg` | 0.881| 0.931| 0.238| 0.905| 0.351| 740| 2900| 0.255 |
| `micro avg` | 0.931| 0.931| 0.238| 0.931| 0.379| 740| 2900| 0.255 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|913 |576 |4 |0 |0 |0 |0 |
|847 |5 |113 |0 |0 |0 |0 |
|1 |10 |1 |0 |0 |0 |0 |
|218 |2 |21 |0 |0 |0 |0 |
|88 |6 |0 |0 |0 |0 |0 |
|93 |1 |1 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "macro avg": {"f1-score": 0.3087066964481233, "precision": 0.2945238095238095, "recall": 0.32512176115332164, "support": 740}, "micro avg": {"f1-score": 0.9310810810810811, "precision": 0.9310810810810811, "recall": 0.9310810810810811, "support": 740}, "weighted avg": {"f1-score": 0.9048670665146819, "precision": 0.881138996138996, "recall": 0.9310810810810811, "support": 740}, "\u2205": {"f1-score": 0.976271186440678, "precision": 0.96, "recall": 0.993103448275862, "support": 580}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 23}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u2423": {"f1-score": 0.875968992248062, "precision": 0.8071428571428572, "recall": 0.9576271186440678, "support": 118}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "macro avg": {"f1-score": 0.1258218337502225, "precision": 0.2945238095238095, "recall": 0.08381647457854559, "support": 2900}, "micro avg": {"f1-score": 0.3785714285714286, "precision": 0.9310810810810811, "recall": 0.23758620689655172, "support": 2900}, "weighted avg": {"f1-score": 0.3514216711941379, "precision": 0.7628182266009852, "recall": 0.23758620689655172, "support": 2900}, "\u2205": {"f1-score": 0.5504061156235069, "precision": 0.96, "recall": 0.38580040187541864, "support": 1493}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 241}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 95}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 94}, "\u2423": {"f1-score": 0.20452488687782805, "precision": 0.8071428571428572, "recall": 0.11709844559585492, "support": 965}},
  "ppcr": 0.25517241379310346
}
```
</details>
