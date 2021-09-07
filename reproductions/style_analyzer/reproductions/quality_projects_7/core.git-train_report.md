# Train report for javascript / file:///tmp/top-repos-quality-repos-x_mp__55/core.git HEAD 674480db75d91bfc265d9ef55ea592452ec394ce

### Classification report

PPCR: 0.488

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.991| 0.980| 0.518| 0.986| 0.680| 1841| 3482| 0.529 |
| `"` | 1.000| 1.000| 0.734| 1.000| 0.847| 750| 1022| 0.734 |
| `␣` | 0.945| 0.958| 0.330| 0.952| 0.489| 506| 1469| 0.344 |
| `⏎` | 0.776| 0.945| 0.414| 0.852| 0.540| 110| 251| 0.438 |
| `⏎␣⁺␣⁺` | 0.880| 0.864| 0.332| 0.872| 0.482| 110| 286| 0.385 |
| `⏎␣⁻␣⁻` | 0.938| 0.894| 0.275| 0.916| 0.426| 85| 276| 0.308 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 195| 0.021 |
| `micro avg` | 0.973| 0.973| 0.475| 0.973| 0.638| 3406| 6981| 0.488 |
| `weighted avg` | 0.973| 0.973| 0.475| 0.973| 0.622| 3406| 6981| 0.488 |
| `macro avg` | 0.790| 0.806| 0.372| 0.797| 0.495| 3406| 6981| 0.488 |

### Confusion matrix

|refusal|  ∅| "| ␣| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1641 |1804 |0 |11 |20 |6 |0 |0 |
|272 |0 |750 |0 |0 |0 |0 |0 |
|963 |6 |0 |485 |5 |5 |5 |0 |
|141 |2 |0 |3 |104 |1 |0 |0 |
|176 |2 |0 |13 |0 |95 |0 |0 |
|191 |6 |0 |1 |1 |1 |76 |0 |
|191 |0 |0 |0 |4 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| test/i3bar.js | 36 |
| sources/i3bar.js | 23 |
| example/utils.js | 13 |
| rollup.config.js | 12 |
| example/main.js | 3 |
| .eslintrc.js | 2 |
| sources/time.js | 1 |
| sources/type.js | 1 |
| example/blocks.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 750}, "macro avg": {"f1-score": 0.7967311459961556, "precision": 0.7900926474393725, "recall": 0.8059441152736663, "support": 3406}, "micro avg": {"f1-score": 0.9729888432178508, "precision": 0.9729888432178508, "recall": 0.9729888432178508, "support": 3406}, "weighted avg": {"f1-score": 0.9728390109648978, "precision": 0.9733067912212442, "recall": 0.9729888432178508, "support": 3406}, "\u2205": {"f1-score": 0.9855230811253756, "precision": 0.9912087912087912, "recall": 0.979902227050516, "support": 1841}, "\u23ce": {"f1-score": 0.8524590163934428, "precision": 0.7761194029850746, "recall": 0.9454545454545454, "support": 110}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8715596330275229, "precision": 0.8796296296296297, "recall": 0.8636363636363636, "support": 110}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9156626506024096, "precision": 0.9382716049382716, "recall": 0.8941176470588236, "support": 85}, "\u2423": {"f1-score": 0.9519136408243376, "precision": 0.9454191033138402, "recall": 0.958498023715415, "support": 506}},
  "cl_report_full": {"\"": {"f1-score": 0.8465011286681716, "precision": 1.0, "recall": 0.7338551859099804, "support": 1022}, "macro avg": {"f1-score": 0.494952463810917, "precision": 0.7900926474393725, "recall": 0.3719967979237667, "support": 6981}, "micro avg": {"f1-score": 0.6381053239626455, "precision": 0.9729888432178508, "recall": 0.4747170892422289, "support": 6981}, "weighted avg": {"f1-score": 0.6223449261960323, "precision": 0.9407753446416462, "recall": 0.4747170892422289, "support": 6981}, "\u2205": {"f1-score": 0.6804979253112033, "precision": 0.9912087912087912, "recall": 0.5180930499712809, "support": 3482}, "\u23ce": {"f1-score": 0.5402597402597402, "precision": 0.7761194029850746, "recall": 0.41434262948207173, "support": 251}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 195}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.4822335025380711, "precision": 0.8796296296296297, "recall": 0.3321678321678322, "support": 286}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.42577030812324923, "precision": 0.9382716049382716, "recall": 0.2753623188405797, "support": 276}, "\u2423": {"f1-score": 0.48940464177598386, "precision": 0.9454191033138402, "recall": 0.3301565690946222, "support": 1469}},
  "ppcr": 0.48789571694599626
}
```
</details>
