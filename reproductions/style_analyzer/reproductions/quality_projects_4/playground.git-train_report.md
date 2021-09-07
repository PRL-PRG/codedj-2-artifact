# Train report for javascript / file:///tmp/top-repos-quality-repos-jj5i1dx5/playground.git HEAD 0dd9efc81280379d819888dd7cde90205e93c71d

### Classification report

PPCR: 0.121

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.994| 1.000| 0.201| 0.997| 0.334| 1122| 5584| 0.201 |
| `␣` | 1.000| 0.969| 0.073| 0.984| 0.136| 192| 2544| 0.075 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 889| 0.001 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 818| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 387| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 385| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 144| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 84| 0.000 |
| `macro avg` | 0.249| 0.246| 0.034| 0.248| 0.059| 1315| 10835| 0.121 |
| `weighted avg` | 0.994| 0.995| 0.121| 0.994| 0.204| 1315| 10835| 0.121 |
| `micro avg` | 0.995| 0.995| 0.121| 0.995| 0.215| 1315| 10835| 0.121 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|4462 |1122 |0 |0 |0 |0 |0 |0 |0 |
|2352 |6 |186 |0 |0 |0 |0 |0 |0 |
|888 |1 |0 |0 |0 |0 |0 |0 |0 |
|818 |0 |0 |0 |0 |0 |0 |0 |0 |
|387 |0 |0 |0 |0 |0 |0 |0 |0 |
|385 |0 |0 |0 |0 |0 |0 |0 |0 |
|144 |0 |0 |0 |0 |0 |0 |0 |0 |
|84 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| js/client/components/Fireworks.js | 3 |
| js/client/components/mapMarker.js | 1 |
| js/server/models/boardObjective.js | 1 |
| babel.config.js | 1 |
| js/server/api/index.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.24762715688970688, "precision": 0.24922497785651018, "recall": 0.24609375, "support": 1315}, "micro avg": {"f1-score": 0.9946768060836502, "precision": 0.9946768060836502, "recall": 0.9946768060836502, "support": 1315}, "weighted avg": {"f1-score": 0.9942686425885275, "precision": 0.99394935455516, "recall": 0.9946768060836502, "support": 1315}, "\u2205": {"f1-score": 0.9968902709906708, "precision": 0.9937998228520815, "recall": 1.0, "support": 1122}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9841269841269841, "precision": 1.0, "recall": 0.96875, "support": 192}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 84}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 818}, "macro avg": {"f1-score": 0.058817564083466056, "precision": 0.24922497785651018, "recall": 0.03425555495485754, "support": 10835}, "micro avg": {"f1-score": 0.21530864197530863, "precision": 0.9946768060836502, "recall": 0.12071988924780803, "support": 10835}, "weighted avg": {"f1-score": 0.2042691706963394, "precision": 0.74696614774398, "recall": 0.12071988924780803, "support": 10835}, "\u2205": {"f1-score": 0.3342767764039922, "precision": 0.9937998228520815, "recall": 0.20093123209169053, "support": 5584}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 889}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 144}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 387}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 385}, "\u2423": {"f1-score": 0.13626373626373625, "precision": 1.0, "recall": 0.07311320754716981, "support": 2544}},
  "ppcr": 0.12136594370096908
}
```
</details>
