# Train report for javascript / file:///tmp/top-repos-quality-repos-g4pj7g7b/sauti-studio-fe.git HEAD e3905ede28b7fa2e41488547d2ddad6d115337b6

### Classification report

PPCR: 0.134

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.996| 1.000| 0.200| 0.998| 0.333| 1067| 5332| 0.200 |
| `"` | 1.000| 1.000| 0.342| 1.000| 0.509| 179| 524| 0.342 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 1760| 0.002 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 861| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 261| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 246| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 154| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 166| 0.000 |
| `macro avg` | 0.250| 0.250| 0.068| 0.250| 0.105| 1250| 9304| 0.134 |
| `weighted avg` | 0.994| 0.997| 0.134| 0.995| 0.220| 1250| 9304| 0.134 |
| `micro avg` | 0.997| 0.997| 0.134| 0.997| 0.236| 1250| 9304| 0.134 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| '| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|4265 |1067 |0 |0 |0 |0 |0 |0 |0 |
|1756 |4 |0 |0 |0 |0 |0 |0 |0 |
|861 |0 |0 |0 |0 |0 |0 |0 |0 |
|345 |0 |0 |0 |179 |0 |0 |0 |0 |
|261 |0 |0 |0 |0 |0 |0 |0 |0 |
|246 |0 |0 |0 |0 |0 |0 |0 |0 |
|154 |0 |0 |0 |0 |0 |0 |0 |0 |
|166 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/pages/Profile.js | 2 |
| src/components/Project.js | 1 |
| src/components/Folder.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 179}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.24976613657623947, "precision": 0.24953314659197012, "recall": 0.25, "support": 1250}, "micro avg": {"f1-score": 0.9968, "precision": 0.9968, "recall": 0.9968, "support": 1250}, "weighted avg": {"f1-score": 0.9952029934518241, "precision": 0.9936119514472456, "recall": 0.9968, "support": 1250}, "\u2205": {"f1-score": 0.9981290926099158, "precision": 0.996265172735761, "recall": 1.0, "support": 1067}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}},
  "cl_report_full": {"\"": {"f1-score": 0.5092460881934566, "precision": 1.0, "recall": 0.3416030534351145, "support": 524}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 166}, "macro avg": {"f1-score": 0.10531592032451004, "precision": 0.24953314659197012, "recall": 0.06771444769589344, "support": 9304}, "micro avg": {"f1-score": 0.2361190070115596, "precision": 0.9968, "recall": 0.13392089423903697, "support": 9304}, "weighted avg": {"f1-score": 0.21967978346175418, "precision": 0.6272663264216549, "recall": 0.13392089423903697, "support": 9304}, "\u2205": {"f1-score": 0.33328127440262373, "precision": 0.996265172735761, "recall": 0.200112528132033, "support": 5332}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 861}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 154}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 261}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 246}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1760}},
  "ppcr": 0.13435081685296646
}
```
</details>
