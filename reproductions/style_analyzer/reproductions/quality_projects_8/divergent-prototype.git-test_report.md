# Test report for javascript / file:///tmp/top-repos-quality-repos-g9gjbqzi/divergent-prototype.git HEAD 04bad10fa5da4d1906d6be023940d6fb9a0c27ed

### Classification report

PPCR: 0.629

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.786| 1.000| 0.923| 0.880| 0.849| 346| 375| 0.923 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 61| 194| 0.314 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 33| 49| 0.673 |
| `"` | 0.870| 1.000| 0.385| 0.930| 0.533| 20| 52| 0.385 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 66| 0.045 |
| `weighted avg` | 0.625| 0.790| 0.497| 0.698| 0.470| 463| 736| 0.629 |
| `micro avg` | 0.790| 0.790| 0.497| 0.790| 0.611| 463| 736| 0.629 |
| `macro avg` | 0.331| 0.400| 0.261| 0.362| 0.276| 463| 736| 0.629 |

### Confusion matrix

|refusal|  ∅| ␣| "| '| ⏎| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|29 |346 |0 |0 |0 |0 |
|133 |61 |0 |0 |0 |0 |
|32 |0 |0 |20 |0 |0 |
|63 |0 |0 |3 |0 |0 |
|16 |33 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9302325581395349, "precision": 0.8695652173913043, "recall": 1.0, "support": 20}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "macro avg": {"f1-score": 0.36212793656429376, "precision": 0.33118577075098815, "recall": 0.4, "support": 463}, "micro avg": {"f1-score": 0.7904967602591793, "precision": 0.7904967602591793, "recall": 0.7904967602591793, "support": 463}, "weighted avg": {"f1-score": 0.6981112663126129, "precision": 0.6252119277098147, "recall": 0.7904967602591793, "support": 463}, "\u2205": {"f1-score": 0.8804071246819338, "precision": 0.7863636363636364, "recall": 1.0, "support": 346}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 33}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 61}},
  "cl_report_full": {"\"": {"f1-score": 0.5333333333333333, "precision": 0.8695652173913043, "recall": 0.38461538461538464, "support": 52}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 66}, "macro avg": {"f1-score": 0.27648261758691206, "precision": 0.33118577075098815, "recall": 0.26145641025641025, "support": 736}, "micro avg": {"f1-score": 0.6105087572977481, "precision": 0.7904967602591793, "recall": 0.49728260869565216, "support": 736}, "weighted avg": {"f1-score": 0.4702965235173825, "precision": 0.46209749312596665, "recall": 0.49728260869565216, "support": 736}, "\u2205": {"f1-score": 0.8490797546012271, "precision": 0.7863636363636364, "recall": 0.9226666666666666, "support": 375}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 49}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 194}},
  "ppcr": 0.6290760869565217
}
```
</details>
