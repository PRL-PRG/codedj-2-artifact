# Train report for javascript / file:///tmp/top-repos-quality-repos-bjp_v9ti/book-store.git HEAD f6ce8d39b94f718bba5f4ab150634689dd032add

### Classification report

PPCR: 0.050

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `␣` | 0.975| 1.000| 0.198| 0.987| 0.329| 468| 2367| 0.198 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 307| 0.033 |
| `∅` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 5180| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 862| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 632| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 293| 0.000 |
| `micro avg` | 0.975| 0.975| 0.049| 0.975| 0.092| 480| 9641| 0.050 |
| `weighted avg` | 0.951| 0.975| 0.049| 0.963| 0.081| 480| 9641| 0.050 |
| `macro avg` | 0.163| 0.167| 0.033| 0.165| 0.055| 480| 9641| 0.050 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|5178 |0 |2 |0 |0 |0 |0 |
|1899 |0 |468 |0 |0 |0 |0 |
|862 |0 |0 |0 |0 |0 |0 |
|632 |0 |0 |0 |0 |0 |0 |
|297 |0 |10 |0 |0 |0 |0 |
|293 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/utils/fuelSavings.js | 4 |
| src/reducers/fuelSavingsReducer.js | 3 |
| src/utils/numberFormat.js | 3 |
| src/utils/dates.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.16455696202531644, "precision": 0.1625, "recall": 0.16666666666666666, "support": 480}, "micro avg": {"f1-score": 0.975, "precision": 0.975, "recall": 0.975, "support": 480}, "weighted avg": {"f1-score": 0.9626582278481012, "precision": 0.950625, "recall": 0.975, "support": 480}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9873417721518987, "precision": 0.975, "recall": 1.0, "support": 468}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 862}, "macro avg": {"f1-score": 0.0547945205479452, "precision": 0.1625, "recall": 0.032953105196451206, "support": 9641}, "micro avg": {"f1-score": 0.09248098014030233, "precision": 0.975, "recall": 0.048542682294367805, "support": 9641}, "weighted avg": {"f1-score": 0.08071691534300568, "precision": 0.2393761020641012, "recall": 0.048542682294367805, "support": 9641}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5180}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 632}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 307}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 293}, "\u2423": {"f1-score": 0.3287671232876712, "precision": 0.975, "recall": 0.19771863117870722, "support": 2367}},
  "ppcr": 0.04978736645576185
}
```
</details>
