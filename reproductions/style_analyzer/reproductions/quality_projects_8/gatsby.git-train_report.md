# Train report for javascript / file:///tmp/top-repos-quality-repos-y3yu8cwh/gatsby.git HEAD fd5869bd9bada740b6ed01f855e389cf953130cd

### Classification report

PPCR: 0.302

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.953| 1.000| 0.418| 0.976| 0.581| 1381| 3304| 0.418 |
| `"` | 1.000| 1.000| 0.500| 1.000| 0.667| 217| 434| 0.500 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 37| 1288| 0.029 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 30| 115| 0.261 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 260| 0.004 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 114| 0.000 |
| `weighted avg` | 0.920| 0.959| 0.290| 0.939| 0.401| 1666| 5515| 0.302 |
| `micro avg` | 0.959| 0.959| 0.290| 0.959| 0.445| 1666| 5515| 0.302 |
| `macro avg` | 0.326| 0.333| 0.153| 0.329| 0.208| 1666| 5515| 0.302 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|1923 |1381 |0 |0 |0 |0 |0 |
|1251 |37 |0 |0 |0 |0 |0 |
|217 |0 |0 |217 |0 |0 |0 |
|259 |1 |0 |0 |0 |0 |0 |
|114 |0 |0 |0 |0 |0 |0 |
|85 |30 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/pages/index.js | 11 |
| src/components/navbar.js | 10 |
| src/pages/imagetest.js | 9 |
| src/pages/page-2.js | 8 |
| src/pages/markdownpage.js | 6 |
| src/pages/formpage.js | 5 |
| src/templates/blog-list-template.js | 5 |
| src/templates/blog-post.js | 4 |
| src/pages/datapage.js | 4 |
| src/components/layout.js | 3 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 217}, "macro avg": {"f1-score": 0.32932862190812723, "precision": 0.3255118472509777, "recall": 0.3333333333333333, "support": 1666}, "micro avg": {"f1-score": 0.9591836734693877, "precision": 0.9591836734693877, "recall": 0.9591836734693877, "support": 1666}, "weighted avg": {"f1-score": 0.9392658830316578, "precision": 0.9202828129181279, "recall": 0.9591836734693877, "support": 1666}, "\u2205": {"f1-score": 0.9759717314487631, "precision": 0.9530710835058661, "recall": 1.0, "support": 1381}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 30}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 37}},
  "cl_report_full": {"\"": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 434}, "macro avg": {"f1-score": 0.2079622226897632, "precision": 0.3255118472509777, "recall": 0.15299636803874092, "support": 5515}, "micro avg": {"f1-score": 0.4450633616487955, "precision": 0.9591836734693877, "recall": 0.2897552130553037, "support": 5515}, "weighted avg": {"f1-score": 0.4006001394865879, "precision": 0.6496730480332514, "recall": 0.2897552130553037, "support": 5515}, "\u2205": {"f1-score": 0.5811066694719125, "precision": 0.9530710835058661, "recall": 0.4179782082324455, "support": 3304}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 260}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 114}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 115}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1288}},
  "ppcr": 0.30208522212148686
}
```
</details>
