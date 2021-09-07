# Train report for javascript / file:///tmp/top-repos-quality-repos-supm1nf0/map-dispatch-to-props-lab-onl01-seng-pt-100619.git HEAD ed79239a806f78c127e77abae3f491583dfeece5

### Classification report

PPCR: 0.199

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.333| 1.000| 0.499| 216| 649| 0.333 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 319| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 115| 0.000 |
| `weighted avg` | 1.000| 1.000| 0.199| 1.000| 0.299| 216| 1083| 0.199 |
| `micro avg` | 1.000| 1.000| 0.199| 1.000| 0.333| 216| 1083| 0.199 |
| `macro avg` | 0.333| 0.333| 0.111| 0.333| 0.166| 216| 1083| 0.199 |

### Confusion matrix

|refusal|  ∅| ␣| '| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|433 |216 |0 |0 |
|319 |0 |0 |0 |
|115 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.3333333333333333, "precision": 0.3333333333333333, "recall": 0.3333333333333333, "support": 216}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 216}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 216}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 216}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 115}, "macro avg": {"f1-score": 0.16647398843930636, "precision": 0.3333333333333333, "recall": 0.11093990755007704, "support": 1083}, "micro avg": {"f1-score": 0.3325635103926097, "precision": 1.0, "recall": 0.1994459833795014, "support": 1083}, "weighted avg": {"f1-score": 0.2992842617648472, "precision": 0.5992613111726686, "recall": 0.1994459833795014, "support": 1083}, "\u2205": {"f1-score": 0.49942196531791905, "precision": 1.0, "recall": 0.33281972265023113, "support": 649}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 319}},
  "ppcr": 0.1994459833795014
}
```
</details>
