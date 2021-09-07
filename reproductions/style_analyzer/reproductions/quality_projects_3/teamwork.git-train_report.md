# Train report for javascript / file:///tmp/top-repos-quality-repos-rnv6mo0r/teamwork.git HEAD deba9917cadbe438797e15502a82ffa5a671edbb

### Classification report

PPCR: 0.302

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.934| 1.000| 0.455| 0.966| 0.612| 2395| 5263| 0.455 |
| `␣` | 1.000| 1.000| 0.166| 1.000| 0.285| 305| 1833| 0.166 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 126| 570| 0.221 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 32| 245| 0.131 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 11| 222| 0.050 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 835| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 530| 0.000 |
| `macro avg` | 0.276| 0.286| 0.089| 0.281| 0.128| 2869| 9498| 0.302 |
| `weighted avg` | 0.886| 0.941| 0.284| 0.913| 0.394| 2869| 9498| 0.302 |
| `micro avg` | 0.941| 0.941| 0.284| 0.941| 0.437| 2869| 9498| 0.302 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| '| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|2868 |2395 |0 |0 |0 |0 |0 |0 |
|1528 |0 |305 |0 |0 |0 |0 |0 |
|835 |0 |0 |0 |0 |0 |0 |0 |
|444 |126 |0 |0 |0 |0 |0 |0 |
|530 |0 |0 |0 |0 |0 |0 |0 |
|213 |32 |0 |0 |0 |0 |0 |0 |
|211 |11 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| backend/test/article-route.test.js | 56 |
| backend/test/auth-route.test.js | 47 |
| backend/test/gif-route.test.js | 36 |
| backend/test/admin-route.test.js | 27 |
| backend/database/db.js | 3 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.2808457926425259, "precision": 0.2762981947849342, "recall": 0.2857142857142857, "support": 2869}, "micro avg": {"f1-score": 0.9410944579993029, "precision": 0.9410944579993029, "recall": 0.9410944579993029, "support": 2869}, "weighted avg": {"f1-score": 0.9126454212798697, "precision": 0.8860715355766549, "recall": 0.9410944579993029, "support": 2869}, "\u2205": {"f1-score": 0.965920548497681, "precision": 0.9340873634945398, "recall": 1.0, "support": 2395}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 126}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 32}, "\u2423": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 305}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 835}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 530}, "macro avg": {"f1-score": 0.1281853620559568, "precision": 0.2762981947849342, "recall": 0.08877964881538605, "support": 9498}, "micro avg": {"f1-score": 0.4366459125090968, "precision": 0.9410944579993029, "recall": 0.28427037271004424, "support": 9498}, "weighted avg": {"f1-score": 0.3941726721873253, "precision": 0.7105813638736327, "recall": 0.28427037271004424, "support": 9498}, "\u2205": {"f1-score": 0.6119841574038584, "precision": 0.9340873634945398, "recall": 0.4550636519095573, "support": 5263}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 570}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 222}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 245}, "\u2423": {"f1-score": 0.2853133769878391, "precision": 1.0, "recall": 0.16639388979814512, "support": 1833}},
  "ppcr": 0.30206359233522845
}
```
</details>
