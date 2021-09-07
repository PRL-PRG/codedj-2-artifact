# Train report for javascript / file:///tmp/top-repos-quality-repos-bg_8qta_/food-pickup-app.git HEAD 808fbde3c98308cae23ff425f423b69e6b533df9

### Classification report

PPCR: 0.034

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `␣` | 0.988| 1.000| 0.203| 0.994| 0.336| 243| 1200| 0.203 |
| `∅` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 3746| 0.001 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 818| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 605| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 324| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 238| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 217| 0.000 |
| `weighted avg` | 0.976| 0.988| 0.034| 0.982| 0.056| 246| 7148| 0.034 |
| `macro avg` | 0.141| 0.143| 0.029| 0.142| 0.048| 246| 7148| 0.034 |
| `micro avg` | 0.988| 0.988| 0.034| 0.988| 0.066| 246| 7148| 0.034 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|3743 |0 |3 |0 |0 |0 |0 |0 |
|957 |0 |243 |0 |0 |0 |0 |0 |
|818 |0 |0 |0 |0 |0 |0 |0 |
|605 |0 |0 |0 |0 |0 |0 |0 |
|324 |0 |0 |0 |0 |0 |0 |0 |
|238 |0 |0 |0 |0 |0 |0 |0 |
|217 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| public/scripts/formControl.js | 2 |
| routes/admin.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.14198071866783524, "precision": 0.14111498257839722, "recall": 0.14285714285714285, "support": 246}, "micro avg": {"f1-score": 0.9878048780487805, "precision": 0.9878048780487805, "recall": 0.9878048780487805, "support": 246}, "weighted avg": {"f1-score": 0.9817447254227144, "precision": 0.9757584770969661, "recall": 0.9878048780487805, "support": 246}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9938650306748467, "precision": 0.9878048780487805, "recall": 1.0, "support": 243}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 818}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 324}, "macro avg": {"f1-score": 0.04801422643746295, "precision": 0.14111498257839722, "recall": 0.028928571428571432, "support": 7148}, "micro avg": {"f1-score": 0.06572896943467678, "precision": 0.9878048780487805, "recall": 0.03399552322327924, "support": 7148}, "weighted avg": {"f1-score": 0.056424104934903296, "precision": 0.16583182060136212, "recall": 0.03399552322327924, "support": 7148}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3746}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 605}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 238}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 217}, "\u2423": {"f1-score": 0.33609958506224064, "precision": 0.9878048780487805, "recall": 0.2025, "support": 1200}},
  "ppcr": 0.03441522104085059
}
```
</details>
