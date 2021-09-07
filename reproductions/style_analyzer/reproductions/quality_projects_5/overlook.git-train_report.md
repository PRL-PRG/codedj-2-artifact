# Train report for javascript / file:///tmp/top-repos-quality-repos-gjj7uwer/overlook.git HEAD 3304b501b9fe71244891a33c692c735eec4b7f89

### Classification report

PPCR: 0.372

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.961| 0.998| 0.508| 0.979| 0.665| 1653| 3245| 0.509 |
| `"` | 1.000| 1.000| 0.487| 1.000| 0.655| 466| 957| 0.487 |
| `␣` | 1.000| 0.995| 0.144| 0.997| 0.251| 199| 1378| 0.144 |
| `⏎␣⁺␣⁺` | 0.929| 0.856| 0.609| 0.891| 0.736| 153| 215| 0.712 |
| `'` | 1.000| 1.000| 0.328| 1.000| 0.494| 152| 463| 0.328 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 36| 663| 0.054 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 14| 187| 0.075 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 79| 0.000 |
| `micro avg` | 0.971| 0.971| 0.361| 0.971| 0.527| 2673| 7187| 0.372 |
| `macro avg` | 0.611| 0.606| 0.260| 0.608| 0.350| 2673| 7187| 0.372 |
| `weighted avg` | 0.953| 0.971| 0.361| 0.962| 0.489| 2673| 7187| 0.372 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1592 |1649 |0 |0 |0 |0 |4 |0 |0 |
|1179 |0 |198 |0 |0 |0 |1 |0 |0 |
|491 |0 |0 |466 |0 |0 |0 |0 |0 |
|627 |31 |0 |0 |0 |0 |5 |0 |0 |
|311 |0 |0 |0 |0 |152 |0 |0 |0 |
|62 |22 |0 |0 |0 |0 |131 |0 |0 |
|173 |14 |0 |0 |0 |0 |0 |0 |0 |
|79 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/index.js | 49 |
| src/domUpdates.js | 11 |
| src/Guest.js | 9 |
| src/Manager.js | 7 |
| src/Hotel.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 466}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 152}, "macro avg": {"f1-score": 0.6084453835096936, "precision": 0.6112542156425136, "recall": 0.606095522748554, "support": 2673}, "micro avg": {"f1-score": 0.9711934156378601, "precision": 0.9711934156378601, "recall": 0.9711934156378601, "support": 2673}, "weighted avg": {"f1-score": 0.9618441923563378, "precision": 0.95308968439207, "recall": 0.9711934156378601, "support": 2673}, "\u2205": {"f1-score": 0.9789254971801722, "precision": 0.960955710955711, "recall": 0.9975801572897761, "support": 1653}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 36}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.891156462585034, "precision": 0.9290780141843972, "recall": 0.8562091503267973, "support": 153}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u2423": {"f1-score": 0.9974811083123425, "precision": 1.0, "recall": 0.9949748743718593, "support": 199}},
  "cl_report_full": {"\"": {"f1-score": 0.6549543218552355, "precision": 1.0, "recall": 0.48693834900731453, "support": 957}, "\u0027": {"f1-score": 0.4943089430894309, "precision": 1.0, "recall": 0.3282937365010799, "support": 463}, "macro avg": {"f1-score": 0.3501590852745803, "precision": 0.6112542156425136, "recall": 0.2595484153910229, "support": 7187}, "micro avg": {"f1-score": 0.5265720081135903, "precision": 0.9711934156378601, "recall": 0.3612077361903437, "support": 7187}, "weighted avg": {"f1-score": 0.48940639711753287, "precision": 0.8509883198971654, "recall": 0.3612077361903437, "support": 7187}, "\u2205": {"f1-score": 0.6647853255392058, "precision": 0.960955710955711, "recall": 0.5081664098613251, "support": 3245}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 663}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 79}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7359550561797753, "precision": 0.9290780141843972, "recall": 0.6093023255813953, "support": 215}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 187}, "\u2423": {"f1-score": 0.25126903553299496, "precision": 1.0, "recall": 0.14368650217706821, "support": 1378}},
  "ppcr": 0.37192152497565045
}
```
</details>
