# Train report for javascript / file:///tmp/top-repos-quality-repos-mwgl8e7c/dashboard.git HEAD 265e8beaa7cf94381ac208074d1405a3b046b475

### Classification report

PPCR: 0.323

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.969| 1.000| 0.354| 0.984| 0.518| 807| 2282| 0.354 |
| `␣` | 0.976| 0.928| 0.245| 0.951| 0.391| 347| 1315| 0.264 |
| `"` | 1.000| 1.000| 0.482| 1.000| 0.650| 266| 552| 0.482 |
| `⏎` | 0.938| 1.000| 0.548| 0.968| 0.692| 167| 305| 0.548 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 12| 208| 0.058 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 216| 0.037 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 97| 0.000 |
| `macro avg` | 0.555| 0.561| 0.233| 0.558| 0.322| 1607| 4975| 0.323 |
| `micro avg` | 0.972| 0.972| 0.314| 0.972| 0.475| 1607| 4975| 0.323 |
| `weighted avg` | 0.960| 0.972| 0.314| 0.966| 0.456| 1607| 4975| 0.323 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1475 |807 |0 |0 |0 |0 |0 |0 |
|968 |15 |322 |0 |10 |0 |0 |0 |
|286 |0 |0 |266 |0 |0 |0 |0 |
|138 |0 |0 |0 |167 |0 |0 |0 |
|208 |0 |8 |0 |0 |0 |0 |0 |
|196 |11 |0 |0 |1 |0 |0 |0 |
|97 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| front/src/components/TokenInfo/mock.js | 10 |
| front/src/components/CommunityInfo/CommunityInfo.js | 9 |
| front/src/index.js | 7 |
| front/src/components/MarketcapChart/MarketcapChart.js | 6 |
| front/src/utils/formatNumValueWithCurrency.js | 6 |
| front/src/components/TokenInfo/TokenInfo.js | 5 |
| front/src/utils/formatNumValue.js | 1 |
| front/src/utils/apiClient.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 266}, "macro avg": {"f1-score": 0.5576454032336685, "precision": 0.55467819113637, "recall": 0.5611362700699877, "support": 1607}, "micro avg": {"f1-score": 0.9719975108898569, "precision": 0.9719975108898569, "recall": 0.9719975108898569, "support": 1607}, "weighted avg": {"f1-score": 0.9657542824906871, "precision": 0.9602235088230376, "recall": 0.9719975108898569, "support": 1607}, "\u2205": {"f1-score": 0.9841463414634146, "precision": 0.9687875150060024, "recall": 1.0, "support": 807}, "\u23ce": {"f1-score": 0.9681159420289854, "precision": 0.9382022471910112, "recall": 1.0, "support": 167}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u2423": {"f1-score": 0.9512555391432791, "precision": 0.9757575757575757, "recall": 0.9279538904899135, "support": 347}},
  "cl_report_full": {"\"": {"f1-score": 0.6503667481662592, "precision": 1.0, "recall": 0.48188405797101447, "support": 552}, "macro avg": {"f1-score": 0.32164364839507076, "precision": 0.55467819113637, "recall": 0.23256130315932713, "support": 4975}, "micro avg": {"f1-score": 0.47462777271346096, "precision": 0.9719975108898569, "recall": 0.31396984924623117, "support": 4975}, "weighted avg": {"f1-score": 0.45570109345666615, "precision": 0.8707630164338026, "recall": 0.31396984924623117, "support": 4975}, "\u2205": {"f1-score": 0.5181380417335473, "precision": 0.9687875150060024, "recall": 0.35363716038562665, "support": 2282}, "\u23ce": {"f1-score": 0.6915113871635611, "precision": 0.9382022471910112, "recall": 0.5475409836065573, "support": 305}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 97}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 216}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 208}, "\u2423": {"f1-score": 0.3914893617021276, "precision": 0.9757575757575757, "recall": 0.24486692015209124, "support": 1315}},
  "ppcr": 0.3230150753768844
}
```
</details>
