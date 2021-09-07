# Test report for javascript / file:///tmp/top-repos-quality-repos-z2f2rfmj/github-actions-continuous-delivery.git HEAD a5998abde5671cc419a86132cf45a16ce07cafa0

### Classification report

PPCR: 0.123

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.281| 1.000| 0.439| 18| 64| 0.281 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 34| 0.000 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 48| 0.000 |
| `weighted avg` | 1.000| 1.000| 0.123| 1.000| 0.192| 18| 146| 0.123 |
| `micro avg` | 1.000| 1.000| 0.123| 1.000| 0.220| 18| 146| 0.123 |
| `macro avg` | 0.333| 0.333| 0.094| 0.333| 0.146| 18| 146| 0.123 |

### Confusion matrix

|refusal|  ∅| ␣| '| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|46 |18 |0 |0 |
|48 |0 |0 |0 |
|34 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.3333333333333333, "precision": 0.3333333333333333, "recall": 0.3333333333333333, "support": 18}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 18}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 18}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 18}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 34}, "macro avg": {"f1-score": 0.14634146341463414, "precision": 0.3333333333333333, "recall": 0.09375, "support": 146}, "micro avg": {"f1-score": 0.2195121951219512, "precision": 1.0, "recall": 0.1232876712328767, "support": 146}, "weighted avg": {"f1-score": 0.192449047778149, "precision": 0.4383561643835616, "recall": 0.1232876712328767, "support": 146}, "\u2205": {"f1-score": 0.43902439024390244, "precision": 1.0, "recall": 0.28125, "support": 64}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 48}},
  "ppcr": 0.1232876712328767
}
```
</details>
