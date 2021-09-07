# Train report for javascript / file:///tmp/top-repos-quality-repos-3wu4z8ze/nodejs_course_project.git HEAD 2b5b25cb120679e3aee6b79b1ad3dd0877a8f498

### Classification report

PPCR: 0.264

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.976| 1.000| 0.341| 0.988| 0.505| 1659| 4865| 0.341 |
| `␣` | 1.000| 0.940| 0.188| 0.969| 0.316| 348| 1740| 0.200 |
| `⏎` | 1.000| 1.000| 0.387| 1.000| 0.558| 224| 579| 0.387 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 16| 225| 0.071 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 197| 0.015 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 458| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 462| 0.000 |
| `macro avg` | 0.425| 0.420| 0.131| 0.422| 0.197| 2250| 8526| 0.264 |
| `weighted avg` | 0.974| 0.982| 0.259| 0.978| 0.391| 2250| 8526| 0.264 |
| `micro avg` | 0.982| 0.982| 0.259| 0.982| 0.410| 2250| 8526| 0.264 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|3206 |1659 |0 |0 |0 |0 |0 |0 |
|1392 |21 |327 |0 |0 |0 |0 |0 |
|355 |0 |0 |224 |0 |0 |0 |0 |
|458 |0 |0 |0 |0 |0 |0 |0 |
|462 |0 |0 |0 |0 |0 |0 |0 |
|209 |16 |0 |0 |0 |0 |0 |0 |
|194 |3 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| test/app.test.js | 16 |
| client/src/Components/Account.js | 9 |
| client/src/App.js | 4 |
| client/src/Components/FormSignUp.js | 3 |
| client/src/Components/Home.js | 2 |
| app.js | 2 |
| client/src/Components/FormSignIn.js | 2 |
| client/src/serviceWorker.js | 1 |
| client/src/Components/NavBar.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.42242529094226533, "precision": 0.42520810560834105, "recall": 0.41995073891625617, "support": 2250}, "micro avg": {"f1-score": 0.9822222222222222, "precision": 0.9822222222222222, "recall": 0.9822222222222222, "support": 2250}, "weighted avg": {"f1-score": 0.9779606979463085, "precision": 0.9741963246354064, "recall": 0.9822222222222222, "support": 2250}, "\u2205": {"f1-score": 0.9880881477069685, "precision": 0.9764567392583873, "recall": 1.0, "support": 1659}, "\u23ce": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 224}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u2423": {"f1-score": 0.9688888888888889, "precision": 1.0, "recall": 0.9396551724137931, "support": 348}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 462}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 458}, "macro avg": {"f1-score": 0.19711326954647151, "precision": 0.42520810560834105, "recall": 0.1308303070400057, "support": 8526}, "micro avg": {"f1-score": 0.4101707498144024, "precision": 0.9822222222222222, "recall": 0.2592071311283134, "support": 8526}, "weighted avg": {"f1-score": 0.39089227703842894, "precision": 0.8291651461989272, "recall": 0.2592071311283134, "support": 8526}, "\u2205": {"f1-score": 0.5054844606946983, "precision": 0.9764567392583873, "recall": 0.3410071942446043, "support": 4865}, "\u23ce": {"f1-score": 0.5579078455790785, "precision": 1.0, "recall": 0.38687392055267705, "support": 579}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 225}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 197}, "\u2423": {"f1-score": 0.3164005805515239, "precision": 1.0, "recall": 0.1879310344827586, "support": 1740}},
  "ppcr": 0.2638986629134412
}
```
</details>
