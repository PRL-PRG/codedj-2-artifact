# Train report for javascript / file:///tmp/top-repos-quality-repos-fp6ay4ue/core-js HEAD 4a85fe5f9678296bc9ffd5cfc44b32d34b18e52f

### Classification report

PPCR: 0.991

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.986| 0.996| 0.994| 0.991| 0.990| 163263| 163616| 0.998 |
| `␣` | 0.979| 0.971| 0.957| 0.975| 0.968| 75656| 76697| 0.986 |
| `'` | 1.000| 1.000| 0.998| 1.000| 0.999| 34884| 34952| 0.998 |
| `⏎` | 0.950| 0.956| 0.917| 0.953| 0.934| 21676| 22598| 0.959 |
| `⏎␣⁺␣⁺` | 0.959| 0.926| 0.910| 0.942| 0.934| 5067| 5161| 0.982 |
| `⏎␣⁻␣⁻` | 0.975| 0.955| 0.947| 0.965| 0.961| 4977| 5018| 0.992 |
| `⏎⏎` | 0.893| 0.572| 0.534| 0.697| 0.668| 2345| 2513| 0.933 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `weighted avg` | 0.982| 0.982| 0.974| 0.982| 0.977| 307868| 310555| 0.991 |
| `macro avg` | 0.843| 0.797| 0.782| 0.815| 0.807| 307868| 310555| 0.991 |
| `micro avg` | 0.982| 0.982| 0.974| 0.982| 0.978| 307868| 310555| 0.991 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|353 |162605 |650 |0 |8 |0 |0 |0 |
|1041 |1824 |73435 |0 |84 |200 |102 |11 |
|68 |0 |0 |34884 |0 |0 |0 |0 |
|922 |247 |529 |0 |20733 |0 |20 |147 |
|94 |113 |241 |0 |19 |4694 |0 |0 |
|41 |45 |156 |0 |23 |0 |4751 |2 |
|168 |47 |9 |0 |948 |0 |0 |1341 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| packages/core-js/modules/web.url.js | 298 |
| tests/compat/tests.js | 256 |
| .eslintrc.js | 221 |
| tests/pure/web.url.js | 87 |
| tests/tests/web.url.js | 86 |
| packages/core-js/modules/es.symbol.js | 84 |
| packages/core-js/internals/array-buffer.js | 82 |
| packages/core-js/modules/es.string.split.js | 82 |
| packages/core-js/modules/web.url-search-params.js | 74 |
| packages/core-js/modules/es.promise.js | 64 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 34884}, "macro avg": {"f1-score": 0.8154664435578125, "precision": 0.8428714529836895, "recall": 0.7969926683695092, "support": 307868}, "micro avg": {"f1-score": 0.9823788116985201, "precision": 0.9823788116985201, "recall": 0.9823788116985201, "support": 307868}, "weighted avg": {"f1-score": 0.9819497470353523, "precision": 0.982106459349804, "recall": 0.9823788116985201, "support": 307868}, "\u2205": {"f1-score": 0.9910588034521429, "precision": 0.986196105069717, "recall": 0.9959696930719146, "support": 163263}, "\u23ce": {"f1-score": 0.953438642477754, "precision": 0.9504011001604401, "recall": 0.9564956634065326, "support": 21676}, "\u23ce\u23ce": {"f1-score": 0.6973478939157566, "precision": 0.893404397068621, "recall": 0.5718550106609808, "support": 2345}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9424756550547133, "precision": 0.9591336330200245, "recall": 0.9263864219459246, "support": 5067}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9646700507614213, "precision": 0.9749640878309049, "recall": 0.9545911191480811, "support": 4977}, "\u2423": {"f1-score": 0.9747405028007115, "precision": 0.978872300719808, "recall": 0.9706434387226394, "support": 75656}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.9990262901655307, "precision": 1.0, "recall": 0.9980544747081712, "support": 34952}, "macro avg": {"f1-score": 0.8066519526347382, "precision": 0.8428714529836895, "recall": 0.782093163277243, "support": 310555}, "micro avg": {"f1-score": 0.9781104519075131, "precision": 0.9823788116985201, "recall": 0.9738790230393972, "support": 310555}, "weighted avg": {"f1-score": 0.9774771378808224, "precision": 0.9819541741701303, "recall": 0.9738790230393972, "support": 310555}, "\u2205": {"f1-score": 0.9899938203393029, "precision": 0.986196105069717, "recall": 0.9938208977117152, "support": 163616}, "\u23ce": {"f1-score": 0.9336455542296176, "precision": 0.9504011001604401, "recall": 0.9174705726170458, "support": 22598}, "\u23ce\u23ce": {"f1-score": 0.6681614349775785, "precision": 0.893404397068621, "recall": 0.533625149224035, "support": 2513}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9336648433615118, "precision": 0.9591336330200245, "recall": 0.9095136601433831, "support": 5161}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9606713173592154, "precision": 0.9749640878309049, "recall": 0.9467915504184934, "support": 5018}, "\u2423": {"f1-score": 0.9680523606451484, "precision": 0.978872300719808, "recall": 0.9574690013951002, "support": 76697}},
  "ppcr": 0.9913477483859542
}
```
</details>
