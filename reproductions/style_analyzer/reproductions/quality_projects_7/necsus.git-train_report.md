# Train report for javascript / file:///tmp/top-repos-quality-repos-oytmtc04/necsus.git HEAD d662dfec5b90476bb56c224d69430339344529ba

### Classification report

PPCR: 0.434

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.981| 1.000| 0.762| 0.990| 0.858| 1288| 1690| 0.762 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 13| 116| 0.112 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 12| 805| 0.015 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 180| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 128| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 108| 0.000 |
| `micro avg` | 0.981| 0.981| 0.426| 0.981| 0.594| 1313| 3027| 0.434 |
| `weighted avg` | 0.962| 0.981| 0.426| 0.972| 0.479| 1313| 3027| 0.434 |
| `macro avg` | 0.163| 0.167| 0.127| 0.165| 0.143| 1313| 3027| 0.434 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| ⏎␣⁺␣⁺| '| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|402 |1288 |0 |0 |0 |0 |0 |
|793 |12 |0 |0 |0 |0 |0 |
|180 |0 |0 |0 |0 |0 |0 |
|128 |0 |0 |0 |0 |0 |0 |
|108 |0 |0 |0 |0 |0 |0 |
|103 |13 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| client/necsus.js | 14 |
| client/js/marked-plaintext.js | 11 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.16506471869793668, "precision": 0.16349327240416348, "recall": 0.16666666666666666, "support": 1313}, "micro avg": {"f1-score": 0.9809596344249809, "precision": 0.9809596344249809, "recall": 0.9809596344249809, "support": 1313}, "weighted avg": {"f1-score": 0.9715309566623417, "precision": 0.9622818043711923, "recall": 0.9809596344249809, "support": 1313}, "\u2205": {"f1-score": 0.9903883121876201, "precision": 0.9809596344249809, "recall": 1.0, "support": 1288}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 13}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 108}, "macro avg": {"f1-score": 0.14296814296814295, "precision": 0.16349327240416348, "recall": 0.12702169625246548, "support": 3027}, "micro avg": {"f1-score": 0.5935483870967742, "precision": 0.9809596344249809, "recall": 0.42550379914106373, "support": 3027}, "weighted avg": {"f1-score": 0.47892202500725783, "precision": 0.54767815731028, "recall": 0.42550379914106373, "support": 3027}, "\u2205": {"f1-score": 0.8578088578088577, "precision": 0.9809596344249809, "recall": 0.762130177514793, "support": 1690}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 180}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 128}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 116}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 805}},
  "ppcr": 0.4337628014535844
}
```
</details>
