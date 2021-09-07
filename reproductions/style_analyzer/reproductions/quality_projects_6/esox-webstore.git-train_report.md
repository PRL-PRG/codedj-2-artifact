# Train report for javascript / file:///tmp/top-repos-quality-repos-yh3x8nlw/esox-webstore.git HEAD a939a58e6ef0b90efba760b9dfb27f1dff0dfd8b

### Classification report

PPCR: 0.287

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.960| 1.000| 0.559| 0.980| 0.707| 1258| 2249| 0.559 |
| `␣` | 1.000| 0.767| 0.107| 0.868| 0.193| 159| 1144| 0.139 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 13| 94| 0.138 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 422| 0.002 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 105| 0.010 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 222| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 762| 0.000 |
| `micro avg` | 0.964| 0.964| 0.276| 0.964| 0.429| 1432| 4998| 0.287 |
| `weighted avg` | 0.955| 0.964| 0.276| 0.957| 0.362| 1432| 4998| 0.287 |
| `macro avg` | 0.280| 0.252| 0.095| 0.264| 0.129| 1432| 4998| 0.287 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|991 |1258 |0 |0 |0 |0 |0 |0 |
|985 |37 |122 |0 |0 |0 |0 |0 |
|421 |1 |0 |0 |0 |0 |0 |0 |
|222 |0 |0 |0 |0 |0 |0 |0 |
|762 |0 |0 |0 |0 |0 |0 |0 |
|104 |1 |0 |0 |0 |0 |0 |0 |
|81 |13 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/actions/index.js | 8 |
| server/app.js | 6 |
| server/user.model.js | 6 |
| server/user.controller.js | 4 |
| server/user.router.js | 4 |
| src/store/configureStore.js | 4 |
| src/store/selectors.js | 4 |
| server/database.js | 3 |
| src/reducers/index.js | 3 |
| server/auth.router.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.26401116870734725, "precision": 0.2800436205016358, "recall": 0.252470799640611, "support": 1432}, "micro avg": {"f1-score": 0.9636871508379888, "precision": 0.9636871508379888, "recall": 0.9636871508379888, "support": 1432}, "weighted avg": {"f1-score": 0.9571162965714249, "precision": 0.9546537165763999, "recall": 0.9636871508379888, "support": 1432}, "\u2205": {"f1-score": 0.9797507788161993, "precision": 0.9603053435114504, "recall": 1.0, "support": 1258}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 13}, "\u2423": {"f1-score": 0.8683274021352313, "precision": 1.0, "recall": 0.7672955974842768, "support": 159}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 762}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 222}, "macro avg": {"f1-score": 0.12852473844365436, "precision": 0.2800436205016358, "recall": 0.09514329601034803, "support": 4998}, "micro avg": {"f1-score": 0.42923794712286156, "precision": 0.9636871508379888, "recall": 0.27611044417767105, "support": 4998}, "weighted avg": {"f1-score": 0.36222388417691853, "precision": 0.6610097474104145, "recall": 0.27611044417767105, "support": 4998}, "\u2205": {"f1-score": 0.7069401517280134, "precision": 0.9603053435114504, "recall": 0.5593597154290796, "support": 2249}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 422}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 105}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 94}, "\u2423": {"f1-score": 0.19273301737756712, "precision": 1.0, "recall": 0.10664335664335664, "support": 1144}},
  "ppcr": 0.28651460584233696
}
```
</details>
