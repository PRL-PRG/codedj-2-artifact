# Test report for javascript / file:///tmp/top-repos-quality-repos-smrxrt81/react-component-mounting-and-unmounting-lab-austin-web-012720.git HEAD 5492c7b085da532ec5e1332f62d88e32975530e3

### Classification report

PPCR: 0.099

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.152| 1.000| 0.265| 34| 223| 0.152 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 12| 0.000 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 107| 0.000 |
| `weighted avg` | 1.000| 1.000| 0.099| 1.000| 0.173| 34| 342| 0.099 |
| `micro avg` | 1.000| 1.000| 0.099| 1.000| 0.181| 34| 342| 0.099 |
| `macro avg` | 0.333| 0.333| 0.051| 0.333| 0.088| 34| 342| 0.099 |

### Confusion matrix

|refusal|  ∅| ␣| '| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|189 |34 |0 |0 |
|107 |0 |0 |0 |
|12 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.3333333333333333, "precision": 0.3333333333333333, "recall": 0.3333333333333333, "support": 34}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 34}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 34}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 34}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "macro avg": {"f1-score": 0.08819714656290532, "precision": 0.3333333333333333, "recall": 0.05082212257100149, "support": 342}, "micro avg": {"f1-score": 0.18085106382978725, "precision": 1.0, "recall": 0.09941520467836257, "support": 342}, "weighted avg": {"f1-score": 0.17252599722392883, "precision": 0.652046783625731, "recall": 0.09941520467836257, "support": 342}, "\u2205": {"f1-score": 0.26459143968871596, "precision": 1.0, "recall": 0.15246636771300448, "support": 223}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 107}},
  "ppcr": 0.09941520467836257
}
```
</details>
