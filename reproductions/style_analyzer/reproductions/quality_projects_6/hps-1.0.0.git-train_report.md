# Train report for javascript / file:///tmp/top-repos-quality-repos-473ujmdh/hps-1.0.0.git HEAD 019360dca032e4c80e747a88ad5f69108eb026c6

### Classification report

PPCR: 0.800

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.948| 0.996| 0.976| 0.971| 0.961| 20311| 20739| 0.979 |
| `␣` | 0.967| 0.926| 0.669| 0.946| 0.791| 7258| 10050| 0.722 |
| `'` | 0.968| 0.953| 0.931| 0.961| 0.949| 2172| 2224| 0.977 |
| `"` | 0.927| 0.950| 0.941| 0.938| 0.934| 1362| 1375| 0.991 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 230| 2484| 0.093 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 211| 848| 0.249 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 108| 327| 0.330 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 102| 815| 0.125 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 44| 265| 0.166 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 36| 657| 0.055 |
| `micro avg` | 0.952| 0.952| 0.762| 0.952| 0.847| 31834| 39784| 0.800 |
| `weighted avg` | 0.931| 0.952| 0.762| 0.941| 0.786| 31834| 39784| 0.800 |
| `macro avg` | 0.381| 0.383| 0.352| 0.382| 0.364| 31834| 39784| 0.800 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|428 |20232 |79 |0 |0 |0 |0 |0 |0 |0 |0 |
|2792 |534 |6724 |0 |0 |0 |0 |0 |0 |0 |0 |
|2254 |149 |81 |0 |0 |0 |0 |0 |0 |0 |0 |
|52 |0 |0 |0 |2070 |102 |0 |0 |0 |0 |0 |
|13 |0 |0 |0 |68 |1294 |0 |0 |0 |0 |0 |
|637 |178 |33 |0 |0 |0 |0 |0 |0 |0 |0 |
|713 |98 |4 |0 |0 |0 |0 |0 |0 |0 |0 |
|621 |25 |11 |0 |0 |0 |0 |0 |0 |0 |0 |
|219 |89 |19 |0 |0 |0 |0 |0 |0 |0 |0 |
|221 |41 |3 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| Client/config/webpack.config.js | 166 |
| Client/src/serviceWorker.js | 72 |
| Client/scripts/build.js | 66 |
| Client/scripts/start.js | 58 |
| Client/src/Components/Stocks/StockForm.js | 54 |
| Client/src/Components/FAQ/ListFAQ.js | 51 |
| Client/src/Components/Shared/UploadFile.js | 40 |
| Client/src/_controls/List.js | 35 |
| Client/src/Components/Register/Register.js | 34 |
| Client/config/env.js | 31 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9383611312545324, "precision": 0.9269340974212035, "recall": 0.9500734214390602, "support": 1362}, "\u0027": {"f1-score": 0.9605568445475638, "precision": 0.9681945743685687, "recall": 0.9530386740331491, "support": 2172}, "macro avg": {"f1-score": 0.3816521943513937, "precision": 0.38098664187719145, "recall": 0.38256485901527026, "support": 31834}, "micro avg": {"f1-score": 0.9524407865803858, "precision": 0.9524407865803858, "recall": 0.9524407865803858, "support": 31834}, "weighted avg": {"f1-score": 0.9411800811235455, "precision": 0.9309028883158232, "recall": 0.9524407865803858, "support": 31834}, "\u2205": {"f1-score": 0.9713613558345536, "precision": 0.9478122364845872, "recall": 0.996110482004825, "support": 20311}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 230}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 36}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 108}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 211}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 44}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 102}, "\u2423": {"f1-score": 0.9462426118772869, "precision": 0.9669255104975554, "recall": 0.9264260126756683, "support": 7258}},
  "cl_report_full": {"\"": {"f1-score": 0.9339588596174667, "precision": 0.9269340974212035, "recall": 0.9410909090909091, "support": 1375}, "\u0027": {"f1-score": 0.9491059147180192, "precision": 0.9681945743685687, "recall": 0.9307553956834532, "support": 2224}, "macro avg": {"f1-score": 0.3635420223718249, "precision": 0.38098664187719145, "recall": 0.35164543365092216, "support": 39784}, "micro avg": {"f1-score": 0.846714513111229, "precision": 0.9524407865803858, "recall": 0.762115423285743, "support": 39784}, "weighted avg": {"f1-score": 0.7863328468178831, "precision": 0.824504284896042, "recall": 0.762115423285743, "support": 39784}, "\u2205": {"f1-score": 0.9614827135558988, "precision": 0.9478122364845872, "recall": 0.9755533053667004, "support": 20739}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2484}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 657}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 327}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 848}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 265}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 815}, "\u2423": {"f1-score": 0.7908727358268643, "precision": 0.9669255104975554, "recall": 0.6690547263681592, "support": 10050}},
  "ppcr": 0.8001709229841142
}
```
</details>
