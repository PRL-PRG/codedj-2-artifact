# Train report for javascript / file:///tmp/top-repos-quality-repos-7v9f2yq6/citgm HEAD 0c4c7ccdd1cad8ce9506e34ca523787ba18cafe2

### Classification report

PPCR: 0.981

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.947| 0.987| 0.987| 0.966| 0.966| 11813| 11813| 1.000 |
| `␣` | 0.956| 0.942| 0.942| 0.949| 0.949| 5577| 5577| 1.000 |
| `'` | 1.000| 1.000| 1.000| 1.000| 1.000| 3256| 3256| 1.000 |
| `⏎` | 0.874| 0.885| 0.761| 0.879| 0.814| 1171| 1361| 0.860 |
| `⏎␣⁺␣⁺` | 0.977| 0.887| 0.887| 0.930| 0.930| 772| 772| 1.000 |
| `⏎␣⁻␣⁻` | 0.994| 0.734| 0.734| 0.845| 0.845| 723| 723| 1.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 170| 442| 0.385 |
| `weighted avg` | 0.948| 0.955| 0.936| 0.950| 0.935| 23482| 23944| 0.981 |
| `macro avg` | 0.821| 0.776| 0.759| 0.796| 0.786| 23482| 23944| 0.981 |
| `micro avg` | 0.955| 0.955| 0.936| 0.955| 0.945| 23482| 23944| 0.981 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|0 |11655 |154 |0 |0 |4 |0 |0 |
|0 |273 |5251 |0 |41 |9 |3 |0 |
|0 |0 |0 |3256 |0 |0 |0 |0 |
|190 |90 |44 |0 |1036 |1 |0 |0 |
|0 |39 |43 |0 |5 |685 |0 |0 |
|0 |161 |0 |0 |29 |2 |531 |0 |
|272 |95 |0 |0 |75 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| bin/citgm-all.js | 80 |
| test/test-lookup.js | 72 |
| lib/lookup.js | 65 |
| lib/match-conditions.js | 63 |
| lib/citgm.js | 53 |
| test/bin/test-citgm-all.js | 46 |
| bin/citgm.js | 45 |
| lib/out.js | 39 |
| lib/npm/test.js | 36 |
| test/test-check-tags.js | 35 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 3256}, "macro avg": {"f1-score": 0.7955686316737164, "precision": 0.8211086390860837, "recall": 0.7763757131218219, "support": 23482}, "micro avg": {"f1-score": 0.9545183544842858, "precision": 0.9545183544842858, "recall": 0.9545183544842858, "support": 23482}, "weighted avg": {"f1-score": 0.9504745422788061, "precision": 0.9482243654559377, "recall": 0.9545183544842858, "support": 23482}, "\u2205": {"f1-score": 0.9661775677692116, "precision": 0.946560545764639, "recall": 0.9866249047659358, "support": 11813}, "\u23ce": {"f1-score": 0.8790835808230801, "precision": 0.8735244519392917, "recall": 0.8847139197267293, "support": 1171}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 170}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9300746775288528, "precision": 0.9771754636233951, "recall": 0.8873056994818653, "support": 772}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8448687350835323, "precision": 0.9943820224719101, "recall": 0.7344398340248963, "support": 723}, "\u2423": {"f1-score": 0.9487758605113379, "precision": 0.9561179898033503, "recall": 0.9415456338533261, "support": 5577}},
  "cl_report_full": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 3256}, "macro avg": {"f1-score": 0.786200418069118, "precision": 0.8211086390860837, "recall": 0.758731581207465, "support": 23944}, "micro avg": {"f1-score": 0.9452199215620124, "precision": 0.9545183544842858, "recall": 0.9361009021049115, "support": 23944}, "weighted avg": {"f1-score": 0.9353833157006474, "precision": 0.9368599313191108, "recall": 0.9361009021049115, "support": 23944}, "\u2205": {"f1-score": 0.9661775677692116, "precision": 0.946560545764639, "recall": 0.9866249047659358, "support": 11813}, "\u23ce": {"f1-score": 0.8135060855908912, "precision": 0.8735244519392917, "recall": 0.7612049963262307, "support": 1361}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 442}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9300746775288528, "precision": 0.9771754636233951, "recall": 0.8873056994818653, "support": 772}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8448687350835323, "precision": 0.9943820224719101, "recall": 0.7344398340248963, "support": 723}, "\u2423": {"f1-score": 0.9487758605113379, "precision": 0.9561179898033503, "recall": 0.9415456338533261, "support": 5577}},
  "ppcr": 0.9807049782826596
}
```
</details>
