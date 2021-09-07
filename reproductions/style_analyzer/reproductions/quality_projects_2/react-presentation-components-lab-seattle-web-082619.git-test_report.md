# Test report for javascript / file:///tmp/top-repos-quality-repos-sq7v4158/react-presentation-components-lab-seattle-web-082619.git HEAD c3f7e0e8ac4c2d493fcf58796f5b4f7a802691ec

### Classification report

PPCR: 0.074

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.138| 1.000| 0.242| 4| 29| 0.138 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 10| 0.000 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 15| 0.000 |
| `weighted avg` | 1.000| 1.000| 0.074| 1.000| 0.130| 4| 54| 0.074 |
| `micro avg` | 1.000| 1.000| 0.074| 1.000| 0.138| 4| 54| 0.074 |
| `macro avg` | 0.333| 0.333| 0.046| 0.333| 0.081| 4| 54| 0.074 |

### Confusion matrix

|refusal|  ∅| ␣| '| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|25 |4 |0 |0 |
|15 |0 |0 |0 |
|10 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.3333333333333333, "precision": 0.3333333333333333, "recall": 0.3333333333333333, "support": 4}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 4}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 4}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 4}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "macro avg": {"f1-score": 0.0808080808080808, "precision": 0.3333333333333333, "recall": 0.04597701149425287, "support": 54}, "micro avg": {"f1-score": 0.13793103448275862, "precision": 1.0, "recall": 0.07407407407407407, "support": 54}, "weighted avg": {"f1-score": 0.13019079685746351, "precision": 0.5370370370370371, "recall": 0.07407407407407407, "support": 54}, "\u2205": {"f1-score": 0.2424242424242424, "precision": 1.0, "recall": 0.13793103448275862, "support": 29}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 15}},
  "ppcr": 0.07407407407407407
}
```
</details>
