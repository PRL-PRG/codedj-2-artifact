# Train report for javascript / file:///tmp/top-repos-quality-repos-4boubbal/recipe-app.git HEAD ee8134081c1554b85305271fdfafc8c7dd542381

### Classification report

PPCR: 0.052

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `␣` | 0.997| 1.000| 0.200| 0.998| 0.334| 297| 1482| 0.200 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 182| 0.005 |
| `∅` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 3153| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 326| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 317| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 175| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 112| 0.000 |
| `macro avg` | 0.142| 0.143| 0.029| 0.143| 0.048| 298| 5747| 0.052 |
| `micro avg` | 0.997| 0.997| 0.052| 0.997| 0.098| 298| 5747| 0.052 |
| `weighted avg` | 0.993| 0.997| 0.052| 0.995| 0.086| 298| 5747| 0.052 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|3153 |0 |0 |0 |0 |0 |0 |0 |
|1185 |0 |297 |0 |0 |0 |0 |0 |
|326 |0 |0 |0 |0 |0 |0 |0 |
|317 |0 |0 |0 |0 |0 |0 |0 |
|181 |0 |1 |0 |0 |0 |0 |0 |
|175 |0 |0 |0 |0 |0 |0 |0 |
|112 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/redux/reducers/RecipesReducer.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.1426170468187275, "precision": 0.1423777564717162, "recall": 0.14285714285714285, "support": 298}, "micro avg": {"f1-score": 0.9966442953020134, "precision": 0.9966442953020134, "recall": 0.9966442953020134, "support": 298}, "weighted avg": {"f1-score": 0.9949692628729344, "precision": 0.9932998513580469, "recall": 0.9966442953020134, "support": 298}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9983193277310924, "precision": 0.9966442953020134, "recall": 1.0, "support": 297}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 112}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 326}, "macro avg": {"f1-score": 0.04767255216693418, "precision": 0.1423777564717162, "recall": 0.028629265471370735, "support": 5747}, "micro avg": {"f1-score": 0.09826302729528535, "precision": 0.9966442953020134, "recall": 0.0516791369410127, "support": 5747}, "weighted avg": {"f1-score": 0.08605447297368632, "precision": 0.25700832532409673, "recall": 0.0516791369410127, "support": 5747}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3153}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 317}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 182}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 175}, "\u2423": {"f1-score": 0.3337078651685393, "precision": 0.9966442953020134, "recall": 0.20040485829959515, "support": 1482}},
  "ppcr": 0.05185314076909692
}
```
</details>
