# Train report for javascript / file:///tmp/top-repos-quality-repos-k0djppcj/pomodo.git HEAD 9d78c3c8efecbebe67841e0d82ce82efbbfad201

### Classification report

PPCR: 0.270

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.997| 0.999| 0.365| 0.998| 0.534| 760| 2080| 0.365 |
| `␣` | 1.000| 1.000| 0.181| 1.000| 0.306| 177| 980| 0.181 |
| `⏎␣⁺␣⁺` | 0.991| 1.000| 0.991| 0.996| 0.991| 111| 112| 0.991 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 113| 0.018 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 268| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 245| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 89| 0.000 |
| `macro avg` | 0.427| 0.428| 0.220| 0.428| 0.262| 1050| 3887| 0.270 |
| `weighted avg` | 0.995| 0.997| 0.269| 0.996| 0.392| 1050| 3887| 0.270 |
| `micro avg` | 0.997| 0.997| 0.269| 0.997| 0.424| 1050| 3887| 0.270 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1320 |759 |0 |0 |0 |1 |0 |0 |
|803 |0 |177 |0 |0 |0 |0 |0 |
|268 |0 |0 |0 |0 |0 |0 |0 |
|245 |0 |0 |0 |0 |0 |0 |0 |
|1 |0 |0 |0 |0 |111 |0 |0 |
|111 |2 |0 |0 |0 |0 |0 |0 |
|89 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| js/calendario.js | 3 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.4276490440684991, "precision": 0.4269204725254096, "recall": 0.42838345864661653, "support": 1050}, "micro avg": {"f1-score": 0.9971428571428571, "precision": 0.9971428571428571, "recall": 0.9971428571428571, "support": 1050}, "weighted avg": {"f1-score": 0.9961935508054861, "precision": 0.9952491016117353, "recall": 0.9971428571428571, "support": 1050}, "\u2205": {"f1-score": 0.9980276134122288, "precision": 0.9973718791064389, "recall": 0.9986842105263158, "support": 760}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9955156950672646, "precision": 0.9910714285714286, "recall": 1.0, "support": 111}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u2423": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 177}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 89}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 268}, "macro avg": {"f1-score": 0.26162200422695675, "precision": 0.4269204725254096, "recall": 0.21951250280331913, "support": 3887}, "micro avg": {"f1-score": 0.4241442171359125, "precision": 0.9971428571428571, "recall": 0.26935940313866735, "support": 3887}, "weighted avg": {"f1-score": 0.39162020606420894, "precision": 0.8143898915722647, "recall": 0.26935940313866735, "support": 3887}, "\u2205": {"f1-score": 0.5343189017951425, "precision": 0.9973718791064389, "recall": 0.36490384615384613, "support": 2080}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 245}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9910714285714286, "precision": 0.9910714285714286, "recall": 0.9910714285714286, "support": 112}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 113}, "\u2423": {"f1-score": 0.3059636992221262, "precision": 1.0, "recall": 0.18061224489795918, "support": 980}},
  "ppcr": 0.2701312065860561
}
```
</details>
