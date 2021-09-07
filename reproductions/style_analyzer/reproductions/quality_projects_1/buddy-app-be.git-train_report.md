# Train report for javascript / file:///tmp/top-repos-quality-repos-v8ugd2xp/buddy-app-be.git HEAD 66d503f27d1442029f021bba3b15a4d7573daed2

### Classification report

PPCR: 0.615

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.924| 0.997| 0.858| 0.959| 0.890| 3543| 4117| 0.861 |
| `␣` | 0.960| 0.656| 0.265| 0.780| 0.416| 617| 1527| 0.404 |
| `"` | 1.000| 1.000| 0.500| 1.000| 0.667| 395| 790| 0.500 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 56| 199| 0.281 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 24| 342| 0.070 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 411| 0.015 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 158| 0.000 |
| `macro avg` | 0.412| 0.379| 0.232| 0.391| 0.282| 4641| 7544| 0.615 |
| `micro avg` | 0.934| 0.934| 0.574| 0.934| 0.711| 4641| 7544| 0.615 |
| `weighted avg` | 0.918| 0.934| 0.574| 0.921| 0.640| 4641| 7544| 0.615 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|574 |3533 |10 |0 |0 |0 |0 |0 |
|910 |212 |405 |0 |0 |0 |0 |0 |
|395 |0 |0 |395 |0 |0 |0 |0 |
|405 |6 |0 |0 |0 |0 |0 |0 |
|318 |20 |4 |0 |0 |0 |0 |0 |
|143 |53 |3 |0 |0 |0 |0 |0 |
|158 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| api/auth/authRouter.js | 34 |
| knexfile.js | 28 |
| api/users/usersRouter.spec.js | 23 |
| api/interests/interestsRouter.spec.js | 22 |
| api/user_activities/user_activitiesRouter.js | 21 |
| api/activities/activitiesRouter.js | 16 |
| api/user_activities/user_activities.js | 16 |
| api/interests/interestsRouter.js | 15 |
| api/auth/authMiddleware.js | 14 |
| data/seeds/02-interests.js | 14 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 395}, "macro avg": {"f1-score": 0.3912482693464083, "precision": 0.411945330492942, "recall": 0.37908278257980527, "support": 4641}, "micro avg": {"f1-score": 0.9336349924585219, "precision": 0.9336349924585219, "recall": 0.9336349924585219, "support": 4641}, "weighted avg": {"f1-score": 0.9209763238969195, "precision": 0.9180194310429102, "recall": 0.9336349924585219, "support": 4641}, "\u2205": {"f1-score": 0.9591421202660514, "precision": 0.9239016736401674, "recall": 0.9971775331639853, "support": 3543}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 24}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 56}, "\u2423": {"f1-score": 0.7795957651588065, "precision": 0.9597156398104265, "recall": 0.6564019448946515, "support": 617}},
  "cl_report_full": {"\"": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 790}, "macro avg": {"f1-score": 0.2817252536142019, "precision": 0.411945330492942, "recall": 0.23191072441771418, "support": 7544}, "micro avg": {"f1-score": 0.7112022979072631, "precision": 0.9336349924585219, "recall": 0.5743637327677624, "support": 7544}, "weighted avg": {"f1-score": 0.6395336599958593, "precision": 0.8031798743858816, "recall": 0.5743637327677624, "support": 7544}, "\u2205": {"f1-score": 0.8898123662007305, "precision": 0.9239016736401674, "recall": 0.8581491377216419, "support": 4117}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 411}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 158}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 342}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 199}, "\u2423": {"f1-score": 0.41559774243201636, "precision": 0.9597156398104265, "recall": 0.26522593320235754, "support": 1527}},
  "ppcr": 0.6151908801696713
}
```
</details>
