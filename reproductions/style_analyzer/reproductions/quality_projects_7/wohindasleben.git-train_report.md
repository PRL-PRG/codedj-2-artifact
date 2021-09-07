# Train report for javascript / file:///tmp/top-repos-quality-repos-jexg9eg6/wohindasleben.git HEAD 4cb6da362de937d9974803d8ff32cff4292ec3a3

### Classification report

PPCR: 0.624

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.961| 1.000| 0.837| 0.980| 0.895| 4456| 5323| 0.837 |
| `"` | 1.000| 0.996| 0.994| 0.998| 0.997| 522| 523| 0.998 |
| `'` | 0.996| 1.000| 0.877| 0.998| 0.933| 505| 576| 0.877 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 97| 1687| 0.057 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 65| 224| 0.290 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 15| 410| 0.037 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 219| 0.023 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 121| 0.000 |
| `micro avg` | 0.968| 0.968| 0.603| 0.968| 0.743| 5665| 9083| 0.624 |
| `weighted avg` | 0.937| 0.968| 0.603| 0.952| 0.641| 5665| 9083| 0.624 |
| `macro avg` | 0.370| 0.375| 0.339| 0.372| 0.353| 5665| 9083| 0.624 |

### Confusion matrix

|refusal|  ∅| ␣| '| "| ⏎| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|867 |4456 |0 |0 |0 |0 |0 |0 |0 |
|1590 |97 |0 |0 |0 |0 |0 |0 |0 |
|71 |0 |0 |505 |0 |0 |0 |0 |0 |
|1 |0 |0 |2 |520 |0 |0 |0 |0 |
|395 |15 |0 |0 |0 |0 |0 |0 |0 |
|159 |65 |0 |0 |0 |0 |0 |0 |0 |
|214 |5 |0 |0 |0 |0 |0 |0 |0 |
|121 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/templates/product-page.js | 21 |
| src/templates/index-page.js | 17 |
| src/components/Navbar.js | 15 |
| src/components/Layout.js | 14 |
| src/components/Footer.js | 12 |
| src/pages/contact/file-upload.js | 11 |
| src/templates/blog-post.js | 10 |
| src/pages/contact/index.js | 10 |
| src/components/BlogRoll.js | 10 |
| src/cms/preview-templates/ProductPagePreview.js | 8 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9980806142034548, "precision": 1.0, "recall": 0.9961685823754789, "support": 522}, "\u0027": {"f1-score": 0.9980237154150198, "precision": 0.9960552268244576, "recall": 1.0, "support": 505}, "macro avg": {"f1-score": 0.37201139176311865, "precision": 0.36960177183084936, "recall": 0.37452107279693486, "support": 5665}, "micro avg": {"f1-score": 0.9675198587819948, "precision": 0.9675198587819948, "recall": 0.9675198587819948, "support": 5665}, "weighted avg": {"f1-score": 0.9517778036523422, "precision": 0.9366548563182147, "recall": 0.9675198587819948, "support": 5665}, "\u2205": {"f1-score": 0.9799868044864746, "precision": 0.9607589478223372, "recall": 1.0, "support": 4456}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 15}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 65}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 97}},
  "cl_report_full": {"\"": {"f1-score": 0.9971236816874401, "precision": 1.0, "recall": 0.994263862332696, "support": 523}, "\u0027": {"f1-score": 0.9325946445060018, "precision": 0.9960552268244576, "recall": 0.8767361111111112, "support": 576}, "macro avg": {"f1-score": 0.3530509518021895, "precision": 0.36960177183084936, "recall": 0.3385152371463786, "support": 9083}, "micro avg": {"f1-score": 0.7432872253864931, "precision": 0.9675198587819948, "recall": 0.6034349884399427, "support": 9083}, "weighted avg": {"f1-score": 0.6408787054909979, "precision": 0.6837881415731794, "recall": 0.6034349884399427, "support": 9083}, "\u2205": {"f1-score": 0.8946892882240739, "precision": 0.9607589478223372, "recall": 0.8371219237272215, "support": 5323}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 410}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 121}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 219}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 224}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1687}},
  "ppcr": 0.6236926125729385
}
```
</details>
