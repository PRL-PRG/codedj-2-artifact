# Train report for javascript / file:///tmp/top-repos-quality-repos-g_8x5w7c/booking-server.git HEAD 79f5916a3295e18ce614e96eb890a456673b2b77

### Classification report

PPCR: 0.576

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.986| 0.997| 0.895| 0.992| 0.938| 3935| 4383| 0.898 |
| `␣` | 0.954| 0.867| 0.145| 0.909| 0.252| 264| 1580| 0.167 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.962| 0.978| 0.843| 0.970| 0.899| 231| 268| 0.862 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 17| 453| 0.038 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 237| 0.017 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 481| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 210| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 119| 0.000 |
| `macro avg` | 0.363| 0.355| 0.235| 0.359| 0.261| 4451| 7731| 0.576 |
| `micro avg` | 0.983| 0.983| 0.566| 0.983| 0.719| 4451| 7731| 0.576 |
| `weighted avg` | 0.979| 0.983| 0.566| 0.981| 0.615| 4451| 7731| 0.576 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| '| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|448 |3922 |11 |0 |0 |2 |0 |0 |0 |
|1316 |28 |229 |0 |0 |7 |0 |0 |0 |
|481 |0 |0 |0 |0 |0 |0 |0 |0 |
|436 |17 |0 |0 |0 |0 |0 |0 |0 |
|37 |5 |0 |0 |0 |226 |0 |0 |0 |
|233 |4 |0 |0 |0 |0 |0 |0 |0 |
|210 |0 |0 |0 |0 |0 |0 |0 |0 |
|119 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/main/resources/public/web/appclient/libs/Bootstrap-4-Multi-Select-BsMultiSelect/js/src/MultiSelect.js | 29 |
| src/main/resources/public/web/appclient/libs/Bootstrap-4-Multi-Select-BsMultiSelect/js/src/Bs4Adapter.js | 13 |
| src/main/resources/public/web/appclient/libs/Bootstrap-4-Multi-Select-BsMultiSelect/build/rollup.config.js | 8 |
| src/main/resources/public/web/appclient/libs/Bootstrap-4-Multi-Select-BsMultiSelect/js/src/Bs4AdapterJs.js | 5 |
| src/main/resources/public/web/appclient/libs/Bootstrap-4-Multi-Select-BsMultiSelect/js/src/BsMultiSelect.js | 4 |
| src/main/resources/public/web/appclient/libs/Bootstrap-4-Multi-Select-BsMultiSelect/js/src/Bs4AdapterCss.js | 3 |
| src/main/resources/public/web/appclient/libs/Bootstrap-4-Multi-Select-BsMultiSelect/js/src/AddToJQueryPrototype.js | 3 |
| src/main/resources/public/web/appclient/js/core.js | 3 |
| src/main/resources/public/web/appclient/libs/Bootstrap-4-Multi-Select-BsMultiSelect/build/postcss.config.js | 3 |
| src/main/resources/public/web/appclient/js/detalleAutor.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.35877725252523845, "precision": 0.36278591317407993, "recall": 0.35530944198749154, "support": 4451}, "micro avg": {"f1-score": 0.9833745225791957, "precision": 0.9833745225791957, "recall": 0.9833745225791957, "support": 4451}, "weighted avg": {"f1-score": 0.9808219426540495, "precision": 0.9785688682398652, "recall": 0.9833745225791957, "support": 4451}, "\u2205": {"f1-score": 0.9915307799266845, "precision": 0.9864185110663984, "recall": 0.9966963151207116, "support": 3935}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 17}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9699570815450644, "precision": 0.9617021276595744, "recall": 0.9783549783549783, "support": 231}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u2423": {"f1-score": 0.9087301587301586, "precision": 0.9541666666666667, "recall": 0.8674242424242424, "support": 264}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 481}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 210}, "macro avg": {"f1-score": 0.2610808076361954, "precision": 0.36278591317407993, "recall": 0.23538014873474833, "support": 7731}, "micro avg": {"f1-score": 0.7186012149072402, "precision": 0.9833745225791957, "recall": 0.5661622041133101, "support": 7731}, "weighted avg": {"f1-score": 0.6145898007300828, "precision": 0.787581404417297, "recall": 0.5661622041133101, "support": 7731}, "\u2205": {"f1-score": 0.938389759540615, "precision": 0.9864185110663984, "recall": 0.8948208989276751, "support": 4383}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 453}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 119}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.8986083499005965, "precision": 0.9617021276595744, "recall": 0.8432835820895522, "support": 268}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 237}, "\u2423": {"f1-score": 0.25164835164835164, "precision": 0.9541666666666667, "recall": 0.1449367088607595, "support": 1580}},
  "ppcr": 0.5757340576898202
}
```
</details>
