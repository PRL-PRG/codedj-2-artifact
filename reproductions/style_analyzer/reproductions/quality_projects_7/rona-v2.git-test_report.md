# Test report for javascript / file:///tmp/top-repos-quality-repos-8bmdcf1m/rona-v2.git HEAD a56dca26de054977783196add322b6b8c2f622a8

### Classification report

PPCR: 0.095

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.957| 1.000| 0.141| 0.978| 0.245| 44| 313| 0.141 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 12| 0.167 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 8| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 50| 0.000 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 101| 0.000 |
| `micro avg` | 0.957| 0.957| 0.091| 0.957| 0.166| 46| 484| 0.095 |
| `weighted avg` | 0.915| 0.957| 0.091| 0.935| 0.159| 46| 484| 0.095 |
| `macro avg` | 0.191| 0.200| 0.028| 0.196| 0.049| 46| 484| 0.095 |

### Confusion matrix

|refusal|  ∅| ␣| '| "| ⏎| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|269 |44 |0 |0 |0 |0 |
|101 |0 |0 |0 |0 |0 |
|50 |0 |0 |0 |0 |0 |
|8 |0 |0 |0 |0 |0 |
|10 |2 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.19555555555555554, "precision": 0.19130434782608696, "recall": 0.2, "support": 46}, "micro avg": {"f1-score": 0.9565217391304348, "precision": 0.9565217391304348, "recall": 0.9565217391304348, "support": 46}, "weighted avg": {"f1-score": 0.9352657004830917, "precision": 0.9149338374291116, "recall": 0.9565217391304348, "support": 46}, "\u2205": {"f1-score": 0.9777777777777777, "precision": 0.9565217391304348, "recall": 1.0, "support": 44}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 50}, "macro avg": {"f1-score": 0.04902506963788301, "precision": 0.19130434782608696, "recall": 0.028115015974440893, "support": 484}, "micro avg": {"f1-score": 0.16603773584905662, "precision": 0.9565217391304348, "recall": 0.09090909090909091, "support": 484}, "weighted avg": {"f1-score": 0.158521144593568, "precision": 0.6185770750988143, "recall": 0.09090909090909091, "support": 484}, "\u2205": {"f1-score": 0.24512534818941503, "precision": 0.9565217391304348, "recall": 0.14057507987220447, "support": 313}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 101}},
  "ppcr": 0.09504132231404959
}
```
</details>
