# Train report for javascript / file:///tmp/top-repos-quality-repos-rkqfxt83/noodles-admin.git HEAD 5040633a0569823450b59bfbc284b0d65adff4b4

### Classification report

PPCR: 0.422

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.978| 0.997| 0.751| 0.987| 0.850| 2385| 3166| 0.753 |
| `␣` | 0.985| 0.971| 0.359| 0.978| 0.527| 790| 2134| 0.370 |
| `⏎` | 0.952| 0.889| 0.297| 0.920| 0.453| 225| 673| 0.334 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 16| 340| 0.047 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 128| 0.016 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 332| 0.003 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1299| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 35| 0.000 |
| `weighted avg` | 0.973| 0.978| 0.412| 0.975| 0.508| 3419| 8107| 0.422 |
| `micro avg` | 0.978| 0.978| 0.412| 0.978| 0.580| 3419| 8107| 0.422 |
| `macro avg` | 0.364| 0.357| 0.176| 0.361| 0.229| 3419| 8107| 0.422 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|781 |2377 |8 |0 |0 |0 |0 |0 |0 |
|1344 |14 |767 |0 |9 |0 |0 |0 |0 |
|1299 |0 |0 |0 |0 |0 |0 |0 |0 |
|448 |25 |0 |0 |200 |0 |0 |0 |0 |
|35 |0 |0 |0 |0 |0 |0 |0 |0 |
|324 |11 |4 |0 |1 |0 |0 |0 |0 |
|331 |1 |0 |0 |0 |0 |0 |0 |0 |
|126 |2 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| vue.config.js | 27 |
| src/router/index.js | 11 |
| src/utils/request.js | 8 |
| src/store/modules/user.js | 6 |
| src/utils/index.js | 6 |
| mock/index.js | 5 |
| tests/unit/components/Breadcrumb.spec.js | 2 |
| src/layout/mixin/ResizeHandler.js | 2 |
| mock/mock-server.js | 2 |
| src/store/index.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.3605705355419596, "precision": 0.36439573602781417, "recall": 0.3570525833930419, "support": 3419}, "micro avg": {"f1-score": 0.9780637613337233, "precision": 0.9780637613337233, "recall": 0.9780637613337233, "support": 3419}, "weighted avg": {"f1-score": 0.9751561593570905, "precision": 0.9725351704466001, "recall": 0.9780637613337233, "support": 3419}, "\u2205": {"f1-score": 0.987331256490135, "precision": 0.9781893004115226, "recall": 0.9966457023060796, "support": 2385}, "\u23ce": {"f1-score": 0.9195402298850575, "precision": 0.9523809523809523, "recall": 0.8888888888888888, "support": 225}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.9776927979604844, "precision": 0.9845956354300385, "recall": 0.970886075949367, "support": 790}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 35}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1299}, "macro avg": {"f1-score": 0.22864267370230812, "precision": 0.36439573602781417, "recall": 0.17592317396451232, "support": 8107}, "micro avg": {"f1-score": 0.5802533402741629, "precision": 0.9780637613337233, "recall": 0.412483039348711, "support": 8107}, "weighted avg": {"f1-score": 0.5079898341572089, "precision": 0.7202450711808269, "recall": 0.412483039348711, "support": 8107}, "\u2205": {"f1-score": 0.8495353824160113, "precision": 0.9781893004115226, "recall": 0.7507896399241946, "support": 3166}, "\u23ce": {"f1-score": 0.45300113250283125, "precision": 0.9523809523809523, "recall": 0.2971768202080238, "support": 673}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 128}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 340}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 332}, "\u2423": {"f1-score": 0.5266048746996224, "precision": 0.9845956354300385, "recall": 0.35941893158388005, "support": 2134}},
  "ppcr": 0.4217343036881707
}
```
</details>
