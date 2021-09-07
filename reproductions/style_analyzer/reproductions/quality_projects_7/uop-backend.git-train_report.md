# Train report for javascript / file:///tmp/top-repos-quality-repos-x46pxq3p/uop-backend.git HEAD f5ca4fa230a01423236ad95753710e2efe37c320

### Classification report

PPCR: 0.703

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.935| 0.999| 0.937| 0.966| 0.936| 3549| 3781| 0.939 |
| `␣` | 0.987| 0.771| 0.386| 0.866| 0.555| 1050| 2100| 0.500 |
| `⏎␣⁻␣⁻` | 0.957| 0.957| 0.944| 0.957| 0.950| 280| 284| 0.986 |
| `⏎` | 0.974| 0.983| 0.392| 0.979| 0.560| 232| 581| 0.399 |
| `'` | 1.000| 1.000| 0.524| 1.000| 0.687| 221| 422| 0.524 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 302| 0.033 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 142| 0.042 |
| `micro avg` | 0.948| 0.948| 0.666| 0.948| 0.783| 5348| 7612| 0.703 |
| `weighted avg` | 0.948| 0.948| 0.666| 0.945| 0.734| 5348| 7612| 0.703 |
| `macro avg` | 0.693| 0.673| 0.455| 0.681| 0.527| 5348| 7612| 0.703 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|232 |3544 |5 |0 |0 |0 |0 |0 |
|1050 |228 |810 |0 |0 |0 |12 |0 |
|349 |4 |0 |228 |0 |0 |0 |0 |
|201 |0 |0 |0 |221 |0 |0 |0 |
|292 |5 |5 |0 |0 |0 |0 |0 |
|4 |11 |1 |0 |0 |0 |268 |0 |
|136 |0 |0 |6 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| routes/statistics.js | 140 |
| routes/auth.js | 24 |
| app.js | 18 |
| routes/reported.js | 18 |
| lib/socket-service.js | 15 |
| routes/users.js | 15 |
| routes/opinions.js | 14 |
| bin/www | 11 |
| helpers/tools.js | 8 |
| helpers/middlewares.js | 7 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 221}, "macro avg": {"f1-score": 0.6810095429650203, "precision": 0.6932432418367843, "recall": 0.6728458859569129, "support": 5348}, "micro avg": {"f1-score": 0.9482049364248317, "precision": 0.9482049364248317, "recall": 0.9482049364248317, "support": 5348}, "weighted avg": {"f1-score": 0.9446238461321876, "precision": 0.9476206951409806, "recall": 0.9482049364248317, "support": 5348}, "\u2205": {"f1-score": 0.965536030513554, "precision": 0.9345991561181435, "recall": 0.9985911524373062, "support": 3549}, "\u23ce": {"f1-score": 0.9785407725321887, "precision": 0.9743589743589743, "recall": 0.9827586206896551, "support": 232}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9571428571428572, "precision": 0.9571428571428572, "recall": 0.9571428571428572, "support": 280}, "\u2423": {"f1-score": 0.865847140566542, "precision": 0.9866017052375152, "recall": 0.7714285714285715, "support": 1050}},
  "cl_report_full": {"\u0027": {"f1-score": 0.687402799377916, "precision": 1.0, "recall": 0.523696682464455, "support": 422}, "macro avg": {"f1-score": 0.5268325553522205, "precision": 0.6932432418367843, "recall": 0.45468828000917894, "support": 7612}, "micro avg": {"f1-score": 0.7825617283950617, "precision": 0.9482049364248317, "recall": 0.6661849710982659, "support": 7612}, "weighted avg": {"f1-score": 0.7341804268372188, "precision": 0.9019330170536808, "recall": 0.6661849710982659, "support": 7612}, "\u2205": {"f1-score": 0.9359566882345174, "precision": 0.9345991561181435, "recall": 0.9373181697963502, "support": 3781}, "\u23ce": {"f1-score": 0.5595092024539878, "precision": 0.9743589743589743, "recall": 0.3924268502581756, "support": 581}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 142}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 302}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9503546099290779, "precision": 0.9571428571428572, "recall": 0.9436619718309859, "support": 284}, "\u2423": {"f1-score": 0.5546045874700445, "precision": 0.9866017052375152, "recall": 0.38571428571428573, "support": 2100}},
  "ppcr": 0.7025748817656332
}
```
</details>
