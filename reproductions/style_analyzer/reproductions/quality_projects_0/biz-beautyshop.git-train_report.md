# Train report for javascript / file:///tmp/top-repos-quality-repos-7yig_9kf/biz-beautyshop.git HEAD 579d58e23c091102de4e3cc695238e389d366afa

### Classification report

PPCR: 0.415

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.968| 0.987| 0.574| 0.978| 0.721| 1336| 2298| 0.581 |
| `␣` | 0.972| 0.959| 0.551| 0.966| 0.704| 663| 1154| 0.575 |
| `⏎` | 0.978| 0.968| 0.557| 0.973| 0.710| 317| 551| 0.575 |
| `'` | 1.000| 1.000| 0.420| 1.000| 0.592| 179| 426| 0.420 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 98| 0.092 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 89| 0.034 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 127| 0.008 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 127| 0.008 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1104| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 76| 0.000 |
| `macro avg` | 0.392| 0.392| 0.210| 0.392| 0.273| 2509| 6050| 0.415 |
| `micro avg` | 0.973| 0.973| 0.403| 0.973| 0.570| 2509| 6050| 0.415 |
| `weighted avg` | 0.968| 0.973| 0.403| 0.970| 0.514| 2509| 6050| 0.415 |

### Confusion matrix

|refusal|  ∅| ␣| "| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|962 |1319 |17 |0 |0 |0 |0 |0 |0 |0 |0 |
|491 |20 |636 |0 |0 |7 |0 |0 |0 |0 |0 |
|1104 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|247 |0 |0 |0 |179 |0 |0 |0 |0 |0 |0 |
|234 |10 |0 |0 |0 |307 |0 |0 |0 |0 |0 |
|89 |8 |1 |0 |0 |0 |0 |0 |0 |0 |0 |
|86 |3 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|76 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|126 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|126 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| gulpfile.js | 34 |
| backstop_data/engine_scripts/casper/waitForHelperHelper.js | 4 |
| server/middleware/test.js | 4 |
| components/src/add-to-cart.js | 3 |
| server/middleware/reference.js | 3 |
| server/static.js | 3 |
| components/src/cart-element.js | 3 |
| components/src/cart-list.js | 2 |
| server/middleware/approve.js | 2 |
| server/middleware/approval.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 179}, "macro avg": {"f1-score": 0.3916651376833846, "precision": 0.3918612851793722, "recall": 0.39150057258764226, "support": 2509}, "micro avg": {"f1-score": 0.972897568752491, "precision": 0.972897568752491, "recall": 0.972897568752491, "support": 2509}, "weighted avg": {"f1-score": 0.9701454493634011, "precision": 0.9675194365427592, "recall": 0.972897568752491, "support": 2509}, "\u2205": {"f1-score": 0.977761304670126, "precision": 0.9684287812041116, "recall": 0.9872754491017964, "support": 1336}, "\u23ce": {"f1-score": 0.9730586370839936, "precision": 0.9777070063694268, "recall": 0.9684542586750788, "support": 317}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u2423": {"f1-score": 0.9658314350797267, "precision": 0.9724770642201835, "recall": 0.9592760180995475, "support": 663}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1104}, "\u0027": {"f1-score": 0.5917355371900826, "precision": 1.0, "recall": 0.42018779342723006, "support": 426}, "macro avg": {"f1-score": 0.2725866977116712, "precision": 0.3918612851793722, "recall": 0.21024604655482415, "support": 6050}, "micro avg": {"f1-score": 0.5703937375861665, "precision": 0.972897568752491, "recall": 0.4034710743801653, "support": 6050}, "weighted avg": {"f1-score": 0.5142804592147314, "precision": 0.7127941209630899, "recall": 0.4034710743801653, "support": 6050}, "\u2205": {"f1-score": 0.7207650273224044, "precision": 0.9684287812041116, "recall": 0.5739773716275022, "support": 2298}, "\u23ce": {"f1-score": 0.7098265895953758, "precision": 0.9777070063694268, "recall": 0.5571687840290381, "support": 551}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 76}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 127}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 98}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 127}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 89}, "\u2423": {"f1-score": 0.7035398230088495, "precision": 0.9724770642201835, "recall": 0.5511265164644714, "support": 1154}},
  "ppcr": 0.4147107438016529
}
```
</details>
