# Train report for javascript / file:///tmp/top-repos-quality-repos-mfw870w_/earthweb.git HEAD 18b3ad88d53dba587c19de8de9f13a970d457b2f

### Classification report

PPCR: 0.606

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.988| 0.998| 0.941| 0.993| 0.964| 31136| 32996| 0.944 |
| `␣` | 0.982| 0.972| 0.290| 0.977| 0.448| 4586| 15356| 0.299 |
| `'` | 1.000| 1.000| 0.133| 1.000| 0.235| 430| 3228| 0.133 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 133| 1611| 0.083 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 72| 2512| 0.029 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 52| 1987| 0.026 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 1280| 0.006 |
| `"` | 1.000| 1.000| 0.005| 1.000| 0.011| 2| 373| 0.005 |
| `⏎⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 525| 0.000 |
| `⏎⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 218| 0.000 |
| `macro avg` | 0.397| 0.397| 0.137| 0.397| 0.166| 36419| 60086| 0.606 |
| `micro avg` | 0.987| 0.987| 0.598| 0.987| 0.745| 36419| 60086| 0.606 |
| `weighted avg` | 0.980| 0.987| 0.598| 0.983| 0.657| 36419| 60086| 0.606 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺␣⁺␣⁺| "| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| ⏎⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1860 |31059 |77 |0 |0 |0 |0 |0 |0 |0 |0 |
|10770 |129 |4457 |0 |0 |0 |0 |0 |0 |0 |0 |
|2440 |69 |3 |0 |0 |0 |0 |0 |0 |0 |0 |
|2798 |0 |0 |0 |430 |0 |0 |0 |0 |0 |0 |
|1935 |49 |3 |0 |0 |0 |0 |0 |0 |0 |0 |
|371 |0 |0 |0 |0 |0 |2 |0 |0 |0 |0 |
|1478 |133 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1272 |8 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|525 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|218 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| test/lib/trx.test.js | 92 |
| test/lib/transactionBuilder.test.js | 71 |
| src/lib/trx.js | 60 |
| src/lib/transactionBuilder.js | 25 |
| karma.conf.js | 20 |
| webpack.config.js | 19 |
| src/lib/contract/method.js | 18 |
| src/index.js | 12 |
| test/utils/code.test.js | 11 |
| test/helpers/waitChainData.js | 10 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 2}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 430}, "macro avg": {"f1-score": 0.3969339536293698, "precision": 0.39693798418120185, "recall": 0.3969397889886957, "support": 36419}, "micro avg": {"f1-score": 0.9870671902029161, "precision": 0.9870671902029161, "recall": 0.9870671902029161, "support": 36419}, "weighted avg": {"f1-score": 0.9834460333352545, "precision": 0.9798730392710969, "recall": 0.9870671902029161, "support": 36419}, "\u2205": {"f1-score": 0.9925698672163367, "precision": 0.98766178013801, "recall": 0.9975269784172662, "support": 31136}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 72}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u23ce\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 52}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 133}, "\u2423": {"f1-score": 0.9767696690773614, "precision": 0.9817180616740088, "recall": 0.9718709114696904, "support": 4586}},
  "cl_report_full": {"\"": {"f1-score": 0.010666666666666666, "precision": 1.0, "recall": 0.005361930294906166, "support": 373}, "\u0027": {"f1-score": 0.235101148168398, "precision": 1.0, "recall": 0.1332094175960347, "support": 3228}, "macro avg": {"f1-score": 0.1657719174699058, "precision": 0.39693798418120185, "recall": 0.13701121179783649, "support": 60086}, "micro avg": {"f1-score": 0.7449976685145847, "precision": 0.9870671902029161, "recall": 0.5982758046799588, "support": 60086}, "weighted avg": {"f1-score": 0.6565320934940785, "precision": 0.8531962628981768, "recall": 0.5982758046799588, "support": 60086}, "\u2205": {"f1-score": 0.9639216051394256, "precision": 0.98766178013801, "recall": 0.941295914656322, "support": 32996}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2512}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1280}, "\u23ce\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 218}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 525}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1987}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1611}, "\u2423": {"f1-score": 0.44802975472456774, "precision": 0.9817180616740088, "recall": 0.29024485543110184, "support": 15356}},
  "ppcr": 0.6061145691175981
}
```
</details>
