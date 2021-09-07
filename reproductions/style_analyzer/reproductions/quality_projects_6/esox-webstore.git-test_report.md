# Test report for javascript / file:///tmp/top-repos-quality-repos-yh3x8nlw/esox-webstore.git HEAD a939a58e6ef0b90efba760b9dfb27f1dff0dfd8b

### Classification report

PPCR: 0.063

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.938| 1.000| 0.500| 0.968| 0.652| 61| 122| 0.500 |
| `␣` | 1.000| 0.917| 0.053| 0.957| 0.100| 12| 208| 0.058 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 8| 0.250 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 209| 0.005 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 640| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 6| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 8| 0.000 |
| `micro avg` | 0.947| 0.947| 0.060| 0.947| 0.113| 76| 1201| 0.063 |
| `weighted avg` | 0.911| 0.947| 0.060| 0.928| 0.084| 76| 1201| 0.063 |
| `macro avg` | 0.277| 0.274| 0.079| 0.275| 0.108| 76| 1201| 0.063 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|61 |61 |0 |0 |0 |0 |0 |0 |
|196 |1 |11 |0 |0 |0 |0 |0 |
|208 |1 |0 |0 |0 |0 |0 |0 |
|6 |0 |0 |0 |0 |0 |0 |0 |
|640 |0 |0 |0 |0 |0 |0 |0 |
|8 |0 |0 |0 |0 |0 |0 |0 |
|6 |2 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.27496795819777187, "precision": 0.27692307692307694, "recall": 0.2738095238095238, "support": 76}, "micro avg": {"f1-score": 0.9473684210526315, "precision": 0.9473684210526315, "recall": 0.9473684210526315, "support": 76}, "weighted avg": {"f1-score": 0.9281809596454905, "precision": 0.9111336032388664, "recall": 0.9473684210526315, "support": 76}, "\u2205": {"f1-score": 0.9682539682539683, "precision": 0.9384615384615385, "recall": 1.0, "support": 61}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u2423": {"f1-score": 0.9565217391304348, "precision": 1.0, "recall": 0.9166666666666666, "support": 12}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 640}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "macro avg": {"f1-score": 0.10755186258812367, "precision": 0.27692307692307694, "recall": 0.0789835164835165, "support": 1201}, "micro avg": {"f1-score": 0.11276429130775256, "precision": 0.9473684210526315, "recall": 0.05995004163197336, "support": 1201}, "weighted avg": {"f1-score": 0.083670741096295, "precision": 0.26851982322423623, "recall": 0.05995004163197336, "support": 1201}, "\u2205": {"f1-score": 0.6524064171122995, "precision": 0.9384615384615385, "recall": 0.5, "support": 122}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 209}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u2423": {"f1-score": 0.1004566210045662, "precision": 1.0, "recall": 0.052884615384615384, "support": 208}},
  "ppcr": 0.06328059950041633
}
```
</details>
