# Test report for javascript / file:///tmp/top-repos-quality-repos-supm1nf0/map-dispatch-to-props-lab-onl01-seng-pt-100619.git HEAD ed79239a806f78c127e77abae3f491583dfeece5

### Classification report

PPCR: 0.109

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.165| 1.000| 0.284| 20| 121| 0.165 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 10| 0.000 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 53| 0.000 |
| `weighted avg` | 1.000| 1.000| 0.109| 1.000| 0.187| 20| 184| 0.109 |
| `micro avg` | 1.000| 1.000| 0.109| 1.000| 0.196| 20| 184| 0.109 |
| `macro avg` | 0.333| 0.333| 0.055| 0.333| 0.095| 20| 184| 0.109 |

### Confusion matrix

|refusal|  ∅| ␣| '| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|101 |20 |0 |0 |
|53 |0 |0 |0 |
|10 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.3333333333333333, "precision": 0.3333333333333333, "recall": 0.3333333333333333, "support": 20}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 20}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 20}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 20}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "macro avg": {"f1-score": 0.09456264775413713, "precision": 0.3333333333333333, "recall": 0.05509641873278237, "support": 184}, "micro avg": {"f1-score": 0.19607843137254902, "precision": 1.0, "recall": 0.10869565217391304, "support": 184}, "weighted avg": {"f1-score": 0.18655565834104226, "precision": 0.657608695652174, "recall": 0.10869565217391304, "support": 184}, "\u2205": {"f1-score": 0.28368794326241137, "precision": 1.0, "recall": 0.1652892561983471, "support": 121}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 53}},
  "ppcr": 0.10869565217391304
}
```
</details>
