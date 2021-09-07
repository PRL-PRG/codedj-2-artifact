# Train report for javascript / file:///tmp/top-repos-quality-repos-27gup3in/techpoint.git HEAD 911184c8b87b58843db9b96507f59d6a236da953

### Classification report

PPCR: 0.202

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.995| 1.000| 0.267| 0.998| 0.422| 1676| 6266| 0.267 |
| `'` | 1.000| 1.000| 0.663| 1.000| 0.797| 750| 1132| 0.663 |
| `␣` | 1.000| 1.000| 0.039| 1.000| 0.076| 149| 3779| 0.039 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 421| 0.017 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 512| 0.002 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 355| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 301| 0.000 |
| `macro avg` | 0.428| 0.429| 0.138| 0.428| 0.185| 2583| 12766| 0.202 |
| `weighted avg` | 0.994| 0.997| 0.202| 0.995| 0.300| 2583| 12766| 0.202 |
| `micro avg` | 0.997| 0.997| 0.202| 0.997| 0.336| 2583| 12766| 0.202 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|4590 |1676 |0 |0 |0 |0 |0 |0 |
|3630 |0 |149 |0 |0 |0 |0 |0 |
|382 |0 |0 |750 |0 |0 |0 |0 |
|511 |1 |0 |0 |0 |0 |0 |0 |
|414 |7 |0 |0 |0 |0 |0 |0 |
|355 |0 |0 |0 |0 |0 |0 |0 |
|301 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| client/src/serviceWorker.js | 3 |
| client/src/components/checkout-item.component.js | 1 |
| client/src/components/product-display-item.component.js | 1 |
| client/src/components/product-shop.component.js | 1 |
| client/src/components/signup.component.js | 1 |
| client/src/components/header.component.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 750}, "macro avg": {"f1-score": 0.4282312925170068, "precision": 0.42789277231082457, "recall": 0.42857142857142855, "support": 2583}, "micro avg": {"f1-score": 0.9969028261711188, "precision": 0.9969028261711188, "recall": 0.9969028261711188, "support": 2583}, "weighted avg": {"f1-score": 0.9953579263683793, "precision": 0.9938203657571018, "recall": 0.9969028261711188, "support": 2583}, "\u2205": {"f1-score": 0.9976190476190476, "precision": 0.995249406175772, "recall": 1.0, "support": 1676}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 149}},
  "cl_report_full": {"\u0027": {"f1-score": 0.7970244420828906, "precision": 1.0, "recall": 0.6625441696113075, "support": 1132}, "macro avg": {"f1-score": 0.18493217752239172, "precision": 0.42789277231082457, "recall": 0.1384925504505974, "support": 12766}, "micro avg": {"f1-score": 0.33552674441331676, "precision": 0.9969028261711188, "recall": 0.2017076609744634, "support": 12766}, "weighted avg": {"f1-score": 0.3000856953046566, "precision": 0.8731969903726607, "recall": 0.2017076609744634, "support": 12766}, "\u2205": {"f1-score": 0.42163522012578614, "precision": 0.995249406175772, "recall": 0.2674752633258857, "support": 6266}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 512}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 301}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 421}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 355}, "\u2423": {"f1-score": 0.07586558044806517, "precision": 1.0, "recall": 0.03942842021698862, "support": 3779}},
  "ppcr": 0.20233432555224815
}
```
</details>
