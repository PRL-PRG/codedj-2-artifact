# Train report for javascript / file:///tmp/top-repos-quality-repos-qnd_fe5u/zmartr.git HEAD f7e6aab88796f32073161765f1c5ab29f46e4f25

### Classification report

PPCR: 0.642

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.964| 0.992| 0.721| 0.978| 0.825| 10070| 13857| 0.727 |
| `␣` | 0.978| 0.943| 0.555| 0.960| 0.708| 5104| 8674| 0.588 |
| `'` | 0.999| 1.000| 0.947| 1.000| 0.972| 2130| 2250| 0.947 |
| `⏎␣⁺␣⁺` | 0.942| 0.897| 0.599| 0.919| 0.733| 602| 901| 0.668 |
| `⏎` | 0.991| 0.819| 0.177| 0.896| 0.301| 386| 1783| 0.216 |
| `⏎⏎` | 0.974| 0.970| 0.466| 0.972| 0.630| 231| 481| 0.480 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 20| 883| 0.023 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 77| 0.026 |
| `weighted avg` | 0.971| 0.972| 0.623| 0.971| 0.736| 18545| 28906| 0.642 |
| `micro avg` | 0.972| 0.972| 0.623| 0.972| 0.759| 18545| 28906| 0.642 |
| `macro avg` | 0.731| 0.703| 0.433| 0.716| 0.521| 18545| 28906| 0.642 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3787 |9993 |76 |0 |0 |1 |0 |0 |0 |
|3570 |258 |4814 |0 |0 |32 |0 |0 |0 |
|120 |0 |0 |2130 |0 |0 |0 |0 |0 |
|1397 |49 |15 |0 |316 |0 |0 |6 |0 |
|299 |55 |7 |0 |0 |540 |0 |0 |0 |
|863 |8 |12 |0 |0 |0 |0 |0 |0 |
|250 |4 |0 |0 |3 |0 |0 |224 |0 |
|75 |0 |0 |2 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| zmartr-frontend/src/redux/reducers/tasks.js | 28 |
| zmartr-frontend/src/serviceWorker.js | 25 |
| zmartr-frontend/src/components/Tags/DisplayTag.js | 23 |
| zmartr-frontend/src/redux/reducers/stats.js | 23 |
| zmartr-frontend/src/modules/History/HistoryDateField.js | 16 |
| zmartr-frontend/src/components/Tags/AddNewTag.js | 14 |
| zmartr-frontend/src/utils/stats/mergeTimeLists.js | 13 |
| zmartr-frontend/src/modules/Stats/Filter.js | 13 |
| zmartr-frontend/src/components/Metrics/NumberEditField.js | 12 |
| zmartr-frontend/src/utils/stats/toTimeEachDay.js | 11 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u0027": {"f1-score": 0.999530736743313, "precision": 0.99906191369606, "recall": 1.0, "support": 2130}, "macro avg": {"f1-score": 0.7156222345706165, "precision": 0.7309454217091271, "recall": 0.7026118912149837, "support": 18545}, "micro avg": {"f1-score": 0.9715287139390671, "precision": 0.9715287139390671, "recall": 0.9715287139390671, "support": 18545}, "weighted avg": {"f1-score": 0.9706676925091337, "precision": 0.9705779990379282, "recall": 0.9715287139390671, "support": 18545}, "\u2205": {"f1-score": 0.9779321818270783, "precision": 0.9639239895823285, "recall": 0.9923535253227408, "support": 10070}, "\u23ce": {"f1-score": 0.8964539007092198, "precision": 0.9905956112852664, "recall": 0.8186528497409327, "support": 386}, "\u23ce\u23ce": {"f1-score": 0.9718004338394793, "precision": 0.9739130434782609, "recall": 0.9696969696969697, "support": 231}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9191489361702129, "precision": 0.9424083769633508, "recall": 0.8970099667774086, "support": 602}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 20}, "\u2423": {"f1-score": 0.9601116872756282, "precision": 0.9776604386677498, "recall": 0.9431818181818182, "support": 5104}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 77}, "\u0027": {"f1-score": 0.9721588315837517, "precision": 0.99906191369606, "recall": 0.9466666666666667, "support": 2250}, "macro avg": {"f1-score": 0.5210897863663133, "precision": 0.7309454217091271, "recall": 0.4331337860803123, "support": 28906}, "micro avg": {"f1-score": 0.7593939010769004, "precision": 0.9715287139390671, "recall": 0.6232962014806614, "support": 28906}, "weighted avg": {"f1-score": 0.7355213957998828, "precision": 0.9399088345305879, "recall": 0.6232962014806614, "support": 28906}, "\u2205": {"f1-score": 0.825049537648613, "precision": 0.9639239895823285, "recall": 0.7211517644511799, "support": 13857}, "\u23ce": {"f1-score": 0.3006660323501427, "precision": 0.9905956112852664, "recall": 0.1772293886707796, "support": 1783}, "\u23ce\u23ce": {"f1-score": 0.6300984528832629, "precision": 0.9739130434782609, "recall": 0.4656964656964657, "support": 481}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7327001356852103, "precision": 0.9424083769633508, "recall": 0.5993340732519423, "support": 901}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 883}, "\u2423": {"f1-score": 0.7080453007795264, "precision": 0.9776604386677498, "recall": 0.5549919299054646, "support": 8674}},
  "ppcr": 0.6415623054037224
}
```
</details>
