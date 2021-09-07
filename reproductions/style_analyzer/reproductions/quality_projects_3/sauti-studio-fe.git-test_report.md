# Test report for javascript / file:///tmp/top-repos-quality-repos-g4pj7g7b/sauti-studio-fe.git HEAD e3905ede28b7fa2e41488547d2ddad6d115337b6

### Classification report

PPCR: 0.512

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.953| 0.991| 0.756| 0.972| 0.843| 576| 755| 0.763 |
| `␣` | 0.739| 0.298| 0.060| 0.425| 0.111| 57| 283| 0.201 |
| `"` | 0.227| 0.909| 0.128| 0.364| 0.164| 11| 78| 0.141 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 59| 0.153 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 14| 0.500 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 13| 0.462 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 74| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 25| 0.000 |
| `macro avg` | 0.240| 0.275| 0.118| 0.220| 0.140| 666| 1301| 0.512 |
| `weighted avg` | 0.891| 0.898| 0.460| 0.883| 0.523| 666| 1301| 0.512 |
| `micro avg` | 0.898| 0.898| 0.460| 0.898| 0.608| 666| 1301| 0.512 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| '| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|179 |571 |4 |0 |1 |0 |0 |0 |0 |
|226 |8 |17 |0 |32 |0 |0 |0 |0 |
|50 |8 |0 |0 |1 |0 |0 |0 |0 |
|67 |1 |0 |0 |10 |0 |0 |0 |0 |
|7 |4 |2 |0 |0 |0 |0 |0 |0 |
|7 |7 |0 |0 |0 |0 |0 |0 |0 |
|25 |0 |0 |0 |0 |0 |0 |0 |0 |
|74 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.36363636363636365, "precision": 0.22727272727272727, "recall": 0.9090909090909091, "support": 11}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.2200689071566731, "precision": 0.23995732347060647, "recall": 0.27483199594630514, "support": 666}, "micro avg": {"f1-score": 0.8978978978978979, "precision": 0.8978978978978979, "recall": 0.8978978978978979, "support": 666}, "weighted avg": {"f1-score": 0.8829549230081145, "precision": 0.8914497897767115, "recall": 0.8978978978978979, "support": 666}, "\u2205": {"f1-score": 0.9719148936170212, "precision": 0.9532554257095158, "recall": 0.9913194444444444, "support": 576}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u2423": {"f1-score": 0.425, "precision": 0.7391304347826086, "recall": 0.2982456140350877, "support": 57}},
  "cl_report_full": {"\"": {"f1-score": 0.16393442622950816, "precision": 0.22727272727272727, "recall": 0.1282051282051282, "support": 78}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 74}, "macro avg": {"f1-score": 0.13980905258116677, "precision": 0.23995732347060647, "recall": 0.11807089878896211, "support": 1301}, "micro avg": {"f1-score": 0.6080325368581596, "precision": 0.8978978978978979, "recall": 0.45964642582628745, "support": 1301}, "weighted avg": {"f1-score": 0.5234578221279215, "precision": 0.727601100831234, "recall": 0.45964642582628745, "support": 1301}, "\u2205": {"f1-score": 0.8434268833087148, "precision": 0.9532554257095158, "recall": 0.7562913907284768, "support": 755}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 59}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 25}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 13}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u2423": {"f1-score": 0.1111111111111111, "precision": 0.7391304347826086, "recall": 0.06007067137809187, "support": 283}},
  "ppcr": 0.5119139123750961
}
```
</details>
