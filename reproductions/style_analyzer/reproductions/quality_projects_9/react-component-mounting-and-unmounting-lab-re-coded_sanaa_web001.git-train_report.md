# Train report for javascript / file:///tmp/top-repos-quality-repos-6fqd68g3/react-component-mounting-and-unmounting-lab-re-coded_sanaa_web001.git HEAD bc5eab0809dad6eb00090b5ba0e8e84896af225f

### Classification report

PPCR: 0.144

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.244| 1.000| 0.392| 190| 779| 0.244 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 420| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 118| 0.000 |
| `weighted avg` | 1.000| 1.000| 0.144| 1.000| 0.232| 190| 1317| 0.144 |
| `macro avg` | 0.333| 0.333| 0.081| 0.333| 0.131| 190| 1317| 0.144 |
| `micro avg` | 1.000| 1.000| 0.144| 1.000| 0.252| 190| 1317| 0.144 |

### Confusion matrix

|refusal|  ∅| ␣| '| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|589 |190 |0 |0 |
|420 |0 |0 |0 |
|118 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.3333333333333333, "precision": 0.3333333333333333, "recall": 0.3333333333333333, "support": 190}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 190}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 190}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 190}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 118}, "macro avg": {"f1-score": 0.13071895424836602, "precision": 0.3333333333333333, "recall": 0.08130081300813008, "support": 1317}, "micro avg": {"f1-score": 0.252156602521566, "precision": 1.0, "recall": 0.1442672741078208, "support": 1317}, "weighted avg": {"f1-score": 0.2319591466047315, "precision": 0.5914958238420653, "recall": 0.1442672741078208, "support": 1317}, "\u2205": {"f1-score": 0.39215686274509803, "precision": 1.0, "recall": 0.24390243902439024, "support": 779}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 420}},
  "ppcr": 0.1442672741078208
}
```
</details>
