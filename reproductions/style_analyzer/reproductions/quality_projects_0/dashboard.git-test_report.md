# Test report for javascript / file:///tmp/top-repos-quality-repos-mwgl8e7c/dashboard.git HEAD 265e8beaa7cf94381ac208074d1405a3b046b475

### Classification report

PPCR: 0.804

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.657| 0.979| 0.908| 0.786| 0.763| 284| 306| 0.928 |
| `␣` | 0.802| 0.485| 0.384| 0.604| 0.519| 167| 211| 0.791 |
| `"` | 1.000| 0.672| 0.402| 0.804| 0.573| 61| 102| 0.598 |
| `⏎` | 0.733| 0.478| 0.379| 0.579| 0.500| 23| 29| 0.793 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 20| 29| 0.690 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 13| 20| 0.650 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 12| 24| 0.500 |
| `macro avg` | 0.456| 0.373| 0.296| 0.396| 0.336| 580| 721| 0.804 |
| `micro avg` | 0.709| 0.709| 0.570| 0.709| 0.632| 580| 721| 0.804 |
| `weighted avg` | 0.687| 0.709| 0.570| 0.667| 0.577| 580| 721| 0.804 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|22 |278 |6 |0 |0 |0 |0 |0 |
|44 |83 |81 |0 |3 |0 |0 |0 |
|41 |20 |0 |41 |0 |0 |0 |0 |
|6 |12 |0 |0 |11 |0 |0 |0 |
|9 |10 |10 |0 |0 |0 |0 |0 |
|12 |11 |0 |0 |1 |0 |0 |0 |
|7 |9 |4 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.8039215686274509, "precision": 1.0, "recall": 0.6721311475409836, "support": 61}, "macro avg": {"f1-score": 0.39625257832594124, "precision": 0.4560748476063412, "recall": 0.37347074238036876, "support": 580}, "micro avg": {"f1-score": 0.7086206896551724, "precision": 0.7086206896551724, "recall": 0.7086206896551724, "support": 580}, "weighted avg": {"f1-score": 0.6666318485398963, "precision": 0.6869743342639473, "recall": 0.7086206896551724, "support": 580}, "\u2205": {"f1-score": 0.7864214992927863, "precision": 0.6572104018912529, "recall": 0.9788732394366197, "support": 284}, "\u23ce": {"f1-score": 0.5789473684210527, "precision": 0.7333333333333333, "recall": 0.4782608695652174, "support": 23}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 13}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 20}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u2423": {"f1-score": 0.6044776119402986, "precision": 0.801980198019802, "recall": 0.48502994011976047, "support": 167}},
  "cl_report_full": {"\"": {"f1-score": 0.5734265734265734, "precision": 1.0, "recall": 0.4019607843137255, "support": 102}, "macro avg": {"f1-score": 0.3364779938854013, "precision": 0.4560748476063412, "recall": 0.2962363024416609, "support": 721}, "micro avg": {"f1-score": 0.6318216756341275, "precision": 0.7086206896551724, "recall": 0.5700416088765603, "support": 721}, "weighted avg": {"f1-score": 0.5768792217012282, "precision": 0.6845920546845606, "recall": 0.5700416088765603, "support": 721}, "\u2205": {"f1-score": 0.7626886145404663, "precision": 0.6572104018912529, "recall": 0.9084967320261438, "support": 306}, "\u23ce": {"f1-score": 0.5, "precision": 0.7333333333333333, "recall": 0.3793103448275862, "support": 29}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 20}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 29}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 24}, "\u2423": {"f1-score": 0.5192307692307692, "precision": 0.801980198019802, "recall": 0.38388625592417064, "support": 211}},
  "ppcr": 0.8044382801664355
}
```
</details>
