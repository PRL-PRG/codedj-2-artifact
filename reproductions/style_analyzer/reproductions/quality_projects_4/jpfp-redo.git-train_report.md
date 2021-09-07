# Train report for javascript / file:///tmp/top-repos-quality-repos-3u2jbfdv/jpfp-redo.git HEAD 18c087278b7c1ee5ad557de45ed88f5a71fd2f19

### Classification report

PPCR: 0.391

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.923| 1.000| 0.587| 0.960| 0.718| 239| 407| 0.587 |
| `'` | 1.000| 1.000| 0.470| 1.000| 0.639| 47| 100| 0.470 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 14| 195| 0.072 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 79| 0.063 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 1| 1.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `macro avg` | 0.320| 0.333| 0.176| 0.327| 0.226| 306| 782| 0.391 |
| `weighted avg` | 0.874| 0.935| 0.366| 0.903| 0.455| 306| 782| 0.391 |
| `micro avg` | 0.935| 0.935| 0.366| 0.935| 0.526| 306| 782| 0.391 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|168 |239 |0 |0 |0 |0 |
|181 |14 |0 |0 |0 |0 |
|53 |0 |0 |47 |0 |0 |
|74 |5 |0 |0 |0 |0 |
|0 |1 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| server/index.js | 5 |
| app/reducers/index.js | 4 |
| server/api/index.js | 3 |
| server/db/database.js | 3 |
| seed.js | 1 |
| app/components/root.js | 1 |
| server/db/index.js | 1 |
| webpack.config.js | 1 |
| app/main.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 47}, "macro avg": {"f1-score": 0.32663989290495316, "precision": 0.3204633204633205, "recall": 0.3333333333333333, "support": 306}, "micro avg": {"f1-score": 0.934640522875817, "precision": 0.934640522875817, "recall": 0.934640522875817, "support": 306}, "weighted avg": {"f1-score": 0.9032732236134078, "precision": 0.8743281096222274, "recall": 0.934640522875817, "support": 306}, "\u2205": {"f1-score": 0.9598393574297188, "precision": 0.9227799227799228, "recall": 1.0, "support": 239}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6394557823129251, "precision": 1.0, "recall": 0.47, "support": 100}, "macro avg": {"f1-score": 0.22619558333844048, "precision": 0.3204633204633205, "recall": 0.17620393120393119, "support": 782}, "micro avg": {"f1-score": 0.5257352941176471, "precision": 0.934640522875817, "recall": 0.3657289002557545, "support": 782}, "weighted avg": {"f1-score": 0.45531545951714025, "precision": 0.6081476068688345, "recall": 0.3657289002557545, "support": 782}, "\u2205": {"f1-score": 0.7177177177177178, "precision": 0.9227799227799228, "recall": 0.5872235872235873, "support": 407}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 79}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 195}},
  "ppcr": 0.391304347826087
}
```
</details>
