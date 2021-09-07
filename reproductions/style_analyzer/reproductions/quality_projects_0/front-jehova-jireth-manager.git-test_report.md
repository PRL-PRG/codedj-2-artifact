# Test report for javascript / file:///tmp/top-repos-quality-repos-36b2r0a0/front-jehova-jireth-manager.git HEAD ce0b7044d85f1608ca888d72f8b69856f0b4df0a

### Classification report

PPCR: 0.823

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.766| 0.994| 0.933| 0.865| 0.841| 631| 672| 0.939 |
| `␣` | 0.810| 0.126| 0.087| 0.218| 0.157| 135| 195| 0.692 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 27| 33| 0.818 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 19| 33| 0.576 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 16| 28| 0.571 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 12| 20| 0.600 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 16| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 24| 0.000 |
| `macro avg` | 0.197| 0.140| 0.128| 0.135| 0.125| 840| 1021| 0.823 |
| `micro avg` | 0.767| 0.767| 0.631| 0.767| 0.692| 840| 1021| 0.823 |
| `weighted avg` | 0.705| 0.767| 0.631| 0.685| 0.584| 840| 1021| 0.823 |

### Confusion matrix

|refusal|  ∅| ␣| "| '| ⏎| ⏎⏎| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|41 |627 |4 |0 |0 |0 |0 |0 |0 |
|60 |118 |17 |0 |0 |0 |0 |0 |0 |
|16 |0 |0 |0 |0 |0 |0 |0 |0 |
|24 |0 |0 |0 |0 |0 |0 |0 |0 |
|8 |12 |0 |0 |0 |0 |0 |0 |0 |
|12 |16 |0 |0 |0 |0 |0 |0 |0 |
|14 |19 |0 |0 |0 |0 |0 |0 |0 |
|6 |27 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.1353470380194518, "precision": 0.19688644688644688, "recall": 0.13994834771379938, "support": 840}, "micro avg": {"f1-score": 0.7666666666666667, "precision": 0.7666666666666667, "recall": 0.7666666666666667, "support": 840}, "weighted avg": {"f1-score": 0.6846777188328911, "precision": 0.7051892551892552, "recall": 0.7666666666666667, "support": 840}, "\u2205": {"f1-score": 0.8648275862068965, "precision": 0.7655677655677655, "recall": 0.993660855784469, "support": 631}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 27}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 19}, "\u2423": {"f1-score": 0.21794871794871795, "precision": 0.8095238095238095, "recall": 0.1259259259259259, "support": 135}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 24}, "macro avg": {"f1-score": 0.12480671063417542, "precision": 0.19688644688644688, "recall": 0.1275269001831502, "support": 1021}, "micro avg": {"f1-score": 0.692101020956475, "precision": 0.7666666666666667, "recall": 0.6307541625857003, "support": 1021}, "weighted avg": {"f1-score": 0.583621491710082, "precision": 0.6584903832700111, "recall": 0.6307541625857003, "support": 1021}, "\u2205": {"f1-score": 0.8410462776659959, "precision": 0.7655677655677655, "recall": 0.9330357142857143, "support": 672}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 20}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 28}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 33}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 33}, "\u2423": {"f1-score": 0.1574074074074074, "precision": 0.8095238095238095, "recall": 0.08717948717948718, "support": 195}},
  "ppcr": 0.8227228207639569
}
```
</details>
