# Train report for javascript / file:///tmp/top-repos-quality-repos-36b2r0a0/front-jehova-jireth-manager.git HEAD ce0b7044d85f1608ca888d72f8b69856f0b4df0a

### Classification report

PPCR: 0.043

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.950| 1.000| 0.066| 0.974| 0.124| 226| 3418| 0.066 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 1077| 0.009 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 119| 0.017 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 232| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 230| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 166| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 132| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 114| 0.000 |
| `macro avg` | 0.119| 0.125| 0.008| 0.122| 0.015| 238| 5488| 0.043 |
| `micro avg` | 0.950| 0.950| 0.041| 0.950| 0.079| 238| 5488| 0.043 |
| `weighted avg` | 0.902| 0.950| 0.041| 0.925| 0.077| 238| 5488| 0.043 |

### Confusion matrix

|refusal|  ∅| ␣| "| '| ⏎| ⏎⏎| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3192 |226 |0 |0 |0 |0 |0 |0 |0 |
|1067 |10 |0 |0 |0 |0 |0 |0 |0 |
|232 |0 |0 |0 |0 |0 |0 |0 |0 |
|230 |0 |0 |0 |0 |0 |0 |0 |0 |
|166 |0 |0 |0 |0 |0 |0 |0 |0 |
|132 |0 |0 |0 |0 |0 |0 |0 |0 |
|117 |2 |0 |0 |0 |0 |0 |0 |0 |
|114 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/components/shopingCart/ShopingCartForm/ShopingCartForm.js | 2 |
| src/redux/reducers/index.js | 2 |
| src/components/meal-viewer/filter/filter.js | 2 |
| src/components/Header/Header.js | 1 |
| src/components/shopingCart/shopingCartList/shopingCartList.js | 1 |
| src/components/Card/Card.js | 1 |
| src/components/shopingCart/ShopingCart.js | 1 |
| src/components/shopingCalculator/shopingCalculator.js | 1 |
| src/components/meal-viewer/Viewer.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.12176724137931035, "precision": 0.11869747899159663, "recall": 0.125, "support": 238}, "micro avg": {"f1-score": 0.9495798319327731, "precision": 0.9495798319327731, "recall": 0.9495798319327731, "support": 238}, "weighted avg": {"f1-score": 0.9250217328310635, "precision": 0.9017018572134736, "recall": 0.9495798319327731, "support": 238}, "\u2205": {"f1-score": 0.9741379310344828, "precision": 0.9495798319327731, "recall": 1.0, "support": 226}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 232}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 230}, "macro avg": {"f1-score": 0.015454048140043763, "precision": 0.11869747899159663, "recall": 0.008265067290813341, "support": 5488}, "micro avg": {"f1-score": 0.07893817673768774, "precision": 0.9495798319327731, "recall": 0.041180758017492713, "support": 5488}, "weighted avg": {"f1-score": 0.07699990749660289, "precision": 0.5914110542176054, "recall": 0.041180758017492713, "support": 5488}, "\u2205": {"f1-score": 0.1236323851203501, "precision": 0.9495798319327731, "recall": 0.06612053832650673, "support": 3418}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 166}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 132}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 114}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 119}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1077}},
  "ppcr": 0.04336734693877551
}
```
</details>
