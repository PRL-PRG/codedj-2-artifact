# Test report for javascript / file:///tmp/top-repos-quality-repos-0sko0fhr/otus_web.git HEAD b90ad69e1b5c1828fa2ace165710422d113d1d17

### Classification report

PPCR: 0.924

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.912| 0.981| 0.975| 0.945| 0.942| 591| 595| 0.993 |
| `␣` | 0.909| 0.909| 0.895| 0.909| 0.902| 253| 257| 0.984 |
| `'` | 0.959| 0.914| 0.914| 0.936| 0.936| 152| 152| 1.000 |
| `⏎` | 0.889| 0.500| 0.276| 0.640| 0.421| 32| 58| 0.552 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 10| 1.000 |
| `⏎⏎` | 1.000| 0.556| 0.294| 0.714| 0.455| 9| 17| 0.529 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 13| 0.462 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 13| 0.308 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 14| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 15| 0.000 |
| `macro avg` | 0.389| 0.322| 0.280| 0.345| 0.305| 1057| 1144| 0.924 |
| `micro avg` | 0.918| 0.918| 0.848| 0.918| 0.881| 1057| 1144| 0.924 |
| `weighted avg` | 0.901| 0.918| 0.848| 0.906| 0.845| 1057| 1144| 0.924 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎⇥⁺| ⏎⇥⁻| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|4 |580 |11 |0 |0 |0 |0 |0 |0 |0 |0 |
|4 |23 |230 |0 |0 |0 |0 |0 |0 |0 |0 |
|0 |13 |0 |139 |0 |0 |0 |0 |0 |0 |0 |
|26 |9 |7 |0 |16 |0 |0 |0 |0 |0 |0 |
|8 |3 |0 |0 |1 |5 |0 |0 |0 |0 |0 |
|14 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|15 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|9 |3 |0 |0 |1 |0 |0 |0 |0 |0 |0 |
|7 |3 |3 |0 |0 |0 |0 |0 |0 |0 |0 |
|0 |2 |2 |6 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u0027": {"f1-score": 0.936026936026936, "precision": 0.9586206896551724, "recall": 0.9144736842105263, "support": 152}, "macro avg": {"f1-score": 0.3453999027022662, "precision": 0.38904584776413015, "recall": 0.32170896897553325, "support": 1057}, "micro avg": {"f1-score": 0.9176915799432356, "precision": 0.9176915799432356, "recall": 0.9176915799432356, "support": 1057}, "weighted avg": {"f1-score": 0.90625664338849, "precision": 0.9007729928315743, "recall": 0.9176915799432356, "support": 1057}, "\u2205": {"f1-score": 0.9453952730236349, "precision": 0.9119496855345912, "recall": 0.9813874788494078, "support": 591}, "\u23ce": {"f1-score": 0.64, "precision": 0.8888888888888888, "recall": 0.5, "support": 32}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.7142857142857143, "precision": 1.0, "recall": 0.5555555555555556, "support": 9}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u2423": {"f1-score": 0.9090909090909091, "precision": 0.9090909090909091, "recall": 0.9090909090909091, "support": 253}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u0027": {"f1-score": 0.936026936026936, "precision": 0.9586206896551724, "recall": 0.9144736842105263, "support": 152}, "macro avg": {"f1-score": 0.3046590934036348, "precision": 0.38904584776413015, "recall": 0.2795154125368749, "support": 1144}, "micro avg": {"f1-score": 0.8814175374829623, "precision": 0.9176915799432356, "recall": 0.8479020979020979, "support": 1144}, "weighted avg": {"f1-score": 0.8452016724290324, "precision": 0.8658324535949189, "recall": 0.8479020979020979, "support": 1144}, "\u2205": {"f1-score": 0.942323314378554, "precision": 0.9119496855345912, "recall": 0.9747899159663865, "support": 595}, "\u23ce": {"f1-score": 0.42105263157894735, "precision": 0.8888888888888888, "recall": 0.27586206896551724, "support": 58}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.45454545454545453, "precision": 1.0, "recall": 0.29411764705882354, "support": 17}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 13}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 15}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 13}, "\u2423": {"f1-score": 0.9019607843137255, "precision": 0.9090909090909091, "recall": 0.8949416342412452, "support": 257}},
  "ppcr": 0.923951048951049
}
```
</details>
