# Train report for javascript / file:///tmp/top-repos-quality-repos-8jjadn08/protractor-cucumber-slices.git HEAD bac7c77acc117b766e8d4aea80d5c57ed18e07a4

### Classification report

PPCR: 0.195

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.947| 1.000| 0.276| 0.973| 0.428| 681| 2466| 0.276 |
| `'` | 1.000| 1.000| 0.456| 1.000| 0.626| 161| 353| 0.456 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 36| 978| 0.037 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 212| 0.005 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 117| 0.009 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 240| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 146| 0.000 |
| `weighted avg` | 0.916| 0.957| 0.187| 0.936| 0.283| 880| 4512| 0.195 |
| `micro avg` | 0.957| 0.957| 0.187| 0.957| 0.312| 880| 4512| 0.195 |
| `macro avg` | 0.278| 0.286| 0.105| 0.282| 0.151| 880| 4512| 0.195 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1785 |681 |0 |0 |0 |0 |0 |0 |
|942 |36 |0 |0 |0 |0 |0 |0 |
|192 |0 |0 |161 |0 |0 |0 |0 |
|240 |0 |0 |0 |0 |0 |0 |0 |
|211 |1 |0 |0 |0 |0 |0 |0 |
|146 |0 |0 |0 |0 |0 |0 |0 |
|116 |1 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/utils/by.js | 8 |
| src/step-definitions/form.js | 8 |
| src/utils/screenshot.js | 5 |
| src/utils/url.js | 4 |
| src/protractor-mink.js | 3 |
| src/step-definitions/navigation.js | 3 |
| src/utils/element.js | 2 |
| src/step-definitions/utility.js | 1 |
| src/step-definitions/assert-url.js | 1 |
| gulpfile.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 161}, "macro avg": {"f1-score": 0.28183673469387754, "precision": 0.27816411682892905, "recall": 0.2857142857142857, "support": 880}, "micro avg": {"f1-score": 0.9568181818181817, "precision": 0.9568181818181818, "recall": 0.9568181818181818, "support": 880}, "weighted avg": {"f1-score": 0.9358133116883116, "precision": 0.91591857377671, "recall": 0.9568181818181818, "support": 880}, "\u2205": {"f1-score": 0.9728571428571429, "precision": 0.9471488178025035, "recall": 1.0, "support": 681}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 36}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6264591439688716, "precision": 1.0, "recall": 0.45609065155807366, "support": 353}, "macro avg": {"f1-score": 0.1505840939018101, "precision": 0.27816411682892905, "recall": 0.1046066241885187, "support": 4512}, "micro avg": {"f1-score": 0.31231454005934717, "precision": 0.9568181818181818, "recall": 0.18661347517730498, "support": 4512}, "weighted avg": {"f1-score": 0.2827292681132137, "precision": 0.5958929487369179, "recall": 0.18661347517730498, "support": 4512}, "\u2205": {"f1-score": 0.4276295133437991, "precision": 0.9471488178025035, "recall": 0.2761557177615572, "support": 2466}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 240}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 117}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 212}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 146}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 978}},
  "ppcr": 0.1950354609929078
}
```
</details>
