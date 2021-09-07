# Train report for javascript / file:///tmp/top-repos-quality-repos-mhy86_54/portfolio-jerga.git HEAD 5d32328f6668d1062a1824b54105a1f8c1c85864

### Classification report

PPCR: 0.643

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.949| 0.998| 0.835| 0.973| 0.888| 8362| 10000| 0.836 |
| `␣` | 0.979| 0.818| 0.261| 0.891| 0.412| 1477| 4624| 0.319 |
| `'` | 0.988| 0.998| 0.983| 0.993| 0.985| 1128| 1146| 0.984 |
| `"` | 0.995| 0.966| 0.961| 0.980| 0.978| 410| 412| 0.995 |
| `⏎␣⁻␣⁻` | 0.989| 0.760| 0.347| 0.859| 0.514| 233| 510| 0.457 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 68| 550| 0.124 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 52| 526| 0.099 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 15| 488| 0.031 |
| `macro avg` | 0.612| 0.567| 0.423| 0.587| 0.472| 11745| 18256| 0.643 |
| `weighted avg` | 0.948| 0.958| 0.617| 0.952| 0.689| 11745| 18256| 0.643 |
| `micro avg` | 0.958| 0.958| 0.617| 0.958| 0.750| 11745| 18256| 0.643 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1638 |8348 |14 |0 |0 |0 |0 |0 |0 |
|3147 |268 |1208 |0 |0 |0 |1 |0 |0 |
|18 |0 |0 |1126 |0 |0 |0 |0 |2 |
|482 |59 |8 |0 |0 |0 |1 |0 |0 |
|474 |48 |4 |0 |0 |0 |0 |0 |0 |
|277 |56 |0 |0 |0 |0 |177 |0 |0 |
|473 |15 |0 |0 |0 |0 |0 |0 |0 |
|2 |0 |0 |14 |0 |0 |0 |0 |396 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| resources/3_Blogs/slate-editor_1/renderers/index.js | 48 |
| components/slate-editor/renderers/index.js | 48 |
| components/slate-editor/Editor.js | 32 |
| resources/3_Blogs/slate-editor/Editor.js | 30 |
| resources/3_Blogs/slate-editor_1/Editor.js | 26 |
| components/form/PortDate.js | 25 |
| pages/userBlogs.js | 24 |
| components/shared/Header.js | 24 |
| server/index.js | 15 |
| pages/portfolios.js | 13 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9801980198019801, "precision": 0.9949748743718593, "recall": 0.9658536585365853, "support": 410}, "\u0027": {"f1-score": 0.9929453262786596, "precision": 0.987719298245614, "recall": 0.99822695035461, "support": 1128}, "macro avg": {"f1-score": 0.5870922418876428, "precision": 0.6124668623322256, "recall": 0.56749213621229, "support": 11745}, "micro avg": {"f1-score": 0.9582801191996594, "precision": 0.9582801191996594, "recall": 0.9582801191996594, "support": 11745}, "weighted avg": {"f1-score": 0.9515702825865787, "precision": 0.9481714145191096, "recall": 0.9582801191996594, "support": 11745}, "\u2205": {"f1-score": 0.9731872231289345, "precision": 0.9492836024562201, "recall": 0.9983257593877063, "support": 8362}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 68}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 15}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 52}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8592233009708738, "precision": 0.9888268156424581, "recall": 0.759656652360515, "support": 233}, "\u2423": {"f1-score": 0.8911840649206934, "precision": 0.9789303079416531, "recall": 0.8178740690589031, "support": 1477}},
  "cl_report_full": {"\"": {"f1-score": 0.9777777777777776, "precision": 0.9949748743718593, "recall": 0.9611650485436893, "support": 412}, "\u0027": {"f1-score": 0.9851268591426071, "precision": 0.987719298245614, "recall": 0.9825479930191972, "support": 1146}, "macro avg": {"f1-score": 0.47218610169266034, "precision": 0.6124668623322256, "recall": 0.4233521924790979, "support": 18256}, "micro avg": {"f1-score": 0.7503083230558981, "precision": 0.9582801191996594, "recall": 0.6165096406660824, "support": 18256}, "weighted avg": {"f1-score": 0.689339535788469, "precision": 0.880015743234703, "recall": 0.6165096406660824, "support": 18256}, "\u2205": {"f1-score": 0.888368628285623, "precision": 0.9492836024562201, "recall": 0.8348, "support": 10000}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 550}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 488}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 526}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.5137880986937591, "precision": 0.9888268156424581, "recall": 0.34705882352941175, "support": 510}, "\u2423": {"f1-score": 0.41242744964151584, "precision": 0.9789303079416531, "recall": 0.26124567474048443, "support": 4624}},
  "ppcr": 0.6433501314636284
}
```
</details>
