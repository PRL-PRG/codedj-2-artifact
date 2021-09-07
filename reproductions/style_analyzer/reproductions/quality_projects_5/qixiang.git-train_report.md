# Train report for javascript / file:///tmp/top-repos-quality-repos-mw8neqtd/qixiang.git HEAD d888c3b4f626ebe7fd86b81b2055358ee458cf56

### Classification report

PPCR: 0.139

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `␣` | 0.989| 1.000| 0.477| 0.994| 0.644| 606| 1270| 0.477 |
| `∅` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 2038| 0.002 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 126| 0.016 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 382| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 327| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 140| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 138| 0.000 |
| `micro avg` | 0.989| 0.989| 0.137| 0.989| 0.241| 613| 4421| 0.139 |
| `macro avg` | 0.141| 0.143| 0.068| 0.142| 0.092| 613| 4421| 0.139 |
| `weighted avg` | 0.977| 0.989| 0.137| 0.983| 0.185| 613| 4421| 0.139 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|2033 |0 |5 |0 |0 |0 |0 |0 |
|664 |0 |606 |0 |0 |0 |0 |0 |
|382 |0 |0 |0 |0 |0 |0 |0 |
|327 |0 |0 |0 |0 |0 |0 |0 |
|140 |0 |0 |0 |0 |0 |0 |0 |
|124 |0 |2 |0 |0 |0 |0 |0 |
|138 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/store/sagas.js | 4 |
| src/components/border.js | 1 |
| src/pages/category/views/context/MaskContext.js | 1 |
| src/pages/orders/views/context/MaskContext.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.14203679831243407, "precision": 0.1412258214868329, "recall": 0.14285714285714285, "support": 613}, "micro avg": {"f1-score": 0.9885807504078303, "precision": 0.9885807504078303, "recall": 0.9885807504078303, "support": 613}, "weighted avg": {"f1-score": 0.9829039126286221, "precision": 0.977291900076909, "recall": 0.9885807504078303, "support": 613}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u2423": {"f1-score": 0.9942575881870386, "precision": 0.9885807504078303, "recall": 1.0, "support": 606}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 382}, "macro avg": {"f1-score": 0.09195053486078446, "precision": 0.1412258214868329, "recall": 0.06816647919010124, "support": 4421}, "micro avg": {"f1-score": 0.2407628128724672, "precision": 0.9885807504078303, "recall": 0.1370730603935761, "support": 4421}, "weighted avg": {"f1-score": 0.18489940169924765, "precision": 0.2839849701465606, "recall": 0.1370730603935761, "support": 4421}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2038}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 327}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 138}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 140}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 126}, "\u2423": {"f1-score": 0.6436537440254912, "precision": 0.9885807504078303, "recall": 0.47716535433070867, "support": 1270}},
  "ppcr": 0.1386564125763402
}
```
</details>
