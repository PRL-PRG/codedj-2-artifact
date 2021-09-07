# Test report for javascript / file:///tmp/top-repos-quality-repos-i5dg_o8x/serverless-wsgi.git HEAD e596820dee9b1c3cbff9a2981c2630da145d1d73

### Classification report

PPCR: 1.000

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.880| 0.957| 0.957| 0.917| 0.917| 23| 23| 1.000 |
| `␣` | 0.792| 0.905| 0.905| 0.844| 0.844| 21| 21| 1.000 |
| `"` | 1.000| 1.000| 1.000| 1.000| 1.000| 20| 20| 1.000 |
| `⏎` | 1.000| 0.750| 0.750| 0.857| 0.857| 8| 8| 1.000 |
| `⏎␣⁺␣⁺` | 1.000| 0.750| 0.750| 0.857| 0.857| 4| 4| 1.000 |
| `⏎␣⁻␣⁻` | 1.000| 0.500| 0.500| 0.667| 0.667| 4| 4| 1.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `micro avg` | 0.900| 0.900| 0.900| 0.900| 0.900| 80| 80| 1.000 |
| `weighted avg` | 0.911| 0.900| 0.900| 0.897| 0.897| 80| 80| 1.000 |
| `macro avg` | 0.810| 0.694| 0.694| 0.735| 0.735| 80| 80| 1.000 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|
|22 |1 |0 |0 |0 |0 |
|2 |19 |0 |0 |0 |0 |
|0 |0 |20 |0 |0 |0 |
|1 |1 |0 |6 |0 |0 |
|0 |1 |0 |0 |3 |0 |
|0 |2 |0 |0 |0 |2 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 20}, "macro avg": {"f1-score": 0.7345804988662132, "precision": 0.8102380952380953, "recall": 0.6944690919846199, "support": 80}, "micro avg": {"f1-score": 0.9, "precision": 0.9, "recall": 0.9, "support": 80}, "weighted avg": {"f1-score": 0.8971130952380953, "precision": 0.9108124999999999, "recall": 0.9, "support": 80}, "\u2205": {"f1-score": 0.9166666666666666, "precision": 0.88, "recall": 0.9565217391304348, "support": 23}, "\u23ce": {"f1-score": 0.8571428571428571, "precision": 1.0, "recall": 0.75, "support": 8}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8571428571428571, "precision": 1.0, "recall": 0.75, "support": 4}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 4}, "\u2423": {"f1-score": 0.8444444444444444, "precision": 0.7916666666666666, "recall": 0.9047619047619048, "support": 21}},
  "cl_report_full": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 20}, "macro avg": {"f1-score": 0.7345804988662132, "precision": 0.8102380952380953, "recall": 0.6944690919846199, "support": 80}, "micro avg": {"f1-score": 0.9, "precision": 0.9, "recall": 0.9, "support": 80}, "weighted avg": {"f1-score": 0.8971130952380953, "precision": 0.9108124999999999, "recall": 0.9, "support": 80}, "\u2205": {"f1-score": 0.9166666666666666, "precision": 0.88, "recall": 0.9565217391304348, "support": 23}, "\u23ce": {"f1-score": 0.8571428571428571, "precision": 1.0, "recall": 0.75, "support": 8}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8571428571428571, "precision": 1.0, "recall": 0.75, "support": 4}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 4}, "\u2423": {"f1-score": 0.8444444444444444, "precision": 0.7916666666666666, "recall": 0.9047619047619048, "support": 21}},
  "ppcr": 1.0
}
```
</details>
