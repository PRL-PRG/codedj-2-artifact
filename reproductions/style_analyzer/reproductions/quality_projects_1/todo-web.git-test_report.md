# Test report for javascript / file:///tmp/top-repos-quality-repos-5n14cu3_/todo-web.git HEAD 2c05c6919a24b3e8ca27a0604c85a084fbfc86b4

### Classification report

PPCR: 0.832

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.847| 0.859| 0.807| 0.853| 0.826| 540| 575| 0.939 |
| `␣` | 0.668| 0.823| 0.635| 0.737| 0.651| 271| 351| 0.772 |
| `'` | 1.000| 1.000| 1.000| 1.000| 1.000| 62| 62| 1.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 30| 33| 0.909 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 23| 47| 0.489 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 11| 34| 0.324 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 33| 0.212 |
| `macro avg` | 0.359| 0.383| 0.349| 0.370| 0.354| 944| 1135| 0.832 |
| `micro avg` | 0.793| 0.793| 0.660| 0.793| 0.721| 944| 1135| 0.832 |
| `weighted avg` | 0.742| 0.793| 0.660| 0.765| 0.675| 944| 1135| 0.832 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|35 |464 |76 |0 |0 |0 |0 |0 |
|80 |48 |223 |0 |0 |0 |0 |0 |
|0 |0 |0 |62 |0 |0 |0 |0 |
|24 |21 |2 |0 |0 |0 |0 |0 |
|3 |6 |24 |0 |0 |0 |0 |0 |
|23 |8 |3 |0 |0 |0 |0 |0 |
|26 |1 |6 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 62}, "macro avg": {"f1-score": 0.37001875130217377, "precision": 0.3591971427322623, "recall": 0.3831624982916496, "support": 944}, "micro avg": {"f1-score": 0.7934322033898306, "precision": 0.7934322033898306, "recall": 0.7934322033898306, "support": 944}, "weighted avg": {"f1-score": 0.7652190123843345, "precision": 0.7416985202550483, "recall": 0.7934322033898306, "support": 944}, "\u2205": {"f1-score": 0.8529411764705882, "precision": 0.8467153284671532, "recall": 0.8592592592592593, "support": 540}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 23}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 30}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u2423": {"f1-score": 0.737190082644628, "precision": 0.6676646706586826, "recall": 0.8228782287822878, "support": 271}},
  "cl_report_full": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 62}, "macro avg": {"f1-score": 0.3539218371764146, "precision": 0.3591971427322623, "recall": 0.3488977367238237, "support": 1135}, "micro avg": {"f1-score": 0.7205387205387206, "precision": 0.7934322033898306, "recall": 0.6599118942731278, "support": 1135}, "weighted avg": {"f1-score": 0.6746168626965784, "precision": 0.6900542848192165, "recall": 0.6599118942731278, "support": 1135}, "\u2205": {"f1-score": 0.8263579697239537, "precision": 0.8467153284671532, "recall": 0.8069565217391305, "support": 575}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 47}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 33}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 33}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 34}, "\u2423": {"f1-score": 0.6510948905109489, "precision": 0.6676646706586826, "recall": 0.6353276353276354, "support": 351}},
  "ppcr": 0.8317180616740089
}
```
</details>
