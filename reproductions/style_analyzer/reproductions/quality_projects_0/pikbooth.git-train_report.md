# Train report for javascript / file:///tmp/top-repos-quality-repos-8fat365u/pikbooth.git HEAD 1136cc8a9cc9b3dac6f509f3bd00b514bc6db3a2

### Classification report

PPCR: 0.276

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.970| 1.000| 0.468| 0.985| 0.631| 772| 1651| 0.468 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 24| 589| 0.041 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 294| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 191| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 154| 0.000 |
| `macro avg` | 0.194| 0.200| 0.094| 0.197| 0.126| 796| 2879| 0.276 |
| `micro avg` | 0.970| 0.970| 0.268| 0.970| 0.420| 796| 2879| 0.276 |
| `weighted avg` | 0.941| 0.970| 0.268| 0.955| 0.362| 796| 2879| 0.276 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|879 |772 |0 |0 |0 |0 |
|565 |24 |0 |0 |0 |0 |
|294 |0 |0 |0 |0 |0 |
|191 |0 |0 |0 |0 |0 |
|154 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| frontend/assets/js/booth.js | 7 |
| frontend/server.js | 6 |
| frontend/assets/js/cmd.js | 5 |
| frontend/assets/js/client.js | 5 |
| frontend/assets/js/cmdset.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.19693877551020406, "precision": 0.19396984924623115, "recall": 0.2, "support": 796}, "micro avg": {"f1-score": 0.9698492462311558, "precision": 0.9698492462311558, "recall": 0.9698492462311558, "support": 796}, "weighted avg": {"f1-score": 0.9550046149112911, "precision": 0.9406075604151409, "recall": 0.9698492462311558, "support": 796}, "\u2205": {"f1-score": 0.9846938775510203, "precision": 0.9698492462311558, "recall": 1.0, "support": 772}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 24}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 154}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 294}, "macro avg": {"f1-score": 0.12619534123416426, "precision": 0.19396984924623115, "recall": 0.09351907934585099, "support": 2879}, "micro avg": {"f1-score": 0.42013605442176877, "precision": 0.9698492462311558, "recall": 0.2681486627301146, "support": 2879}, "weighted avg": {"f1-score": 0.3618417998916381, "precision": 0.5561726660394715, "recall": 0.2681486627301146, "support": 2879}, "\u2205": {"f1-score": 0.6309767061708214, "precision": 0.9698492462311558, "recall": 0.467595396729255, "support": 1651}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 191}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 589}},
  "ppcr": 0.27648489058700937
}
```
</details>
