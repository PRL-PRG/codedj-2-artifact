# Test report for javascript / file:///tmp/top-repos-quality-repos-8fat365u/pikbooth.git HEAD 1136cc8a9cc9b3dac6f509f3bd00b514bc6db3a2

### Classification report

PPCR: 0.693

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.806| 1.000| 0.933| 0.892| 0.865| 224| 240| 0.933 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 41| 68| 0.603 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 13| 17| 0.765 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 20| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 56| 0.000 |
| `macro avg` | 0.161| 0.200| 0.187| 0.178| 0.173| 278| 401| 0.693 |
| `micro avg` | 0.806| 0.806| 0.559| 0.806| 0.660| 278| 401| 0.693 |
| `weighted avg` | 0.649| 0.806| 0.559| 0.719| 0.518| 278| 401| 0.693 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|16 |224 |0 |0 |0 |0 |
|27 |41 |0 |0 |0 |0 |
|56 |0 |0 |0 |0 |0 |
|4 |13 |0 |0 |0 |0 |
|20 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.17848605577689244, "precision": 0.16115107913669063, "recall": 0.2, "support": 278}, "micro avg": {"f1-score": 0.8057553956834532, "precision": 0.8057553956834532, "recall": 0.8057553956834532, "support": 278}, "weighted avg": {"f1-score": 0.7190805124824444, "precision": 0.6492417576729982, "recall": 0.8057553956834532, "support": 278}, "\u2205": {"f1-score": 0.8924302788844622, "precision": 0.8057553956834532, "recall": 1.0, "support": 224}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 13}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 41}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 20}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 56}, "macro avg": {"f1-score": 0.17297297297297295, "precision": 0.16115107913669063, "recall": 0.18666666666666668, "support": 401}, "micro avg": {"f1-score": 0.6597938144329897, "precision": 0.8057553956834532, "recall": 0.5586034912718204, "support": 401}, "weighted avg": {"f1-score": 0.5176248567769764, "precision": 0.4822476183641615, "recall": 0.5586034912718204, "support": 401}, "\u2205": {"f1-score": 0.8648648648648648, "precision": 0.8057553956834532, "recall": 0.9333333333333333, "support": 240}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 17}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 68}},
  "ppcr": 0.6932668329177057
}
```
</details>
