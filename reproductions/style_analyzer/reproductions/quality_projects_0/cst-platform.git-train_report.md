# Train report for javascript / file:///tmp/top-repos-quality-repos-0lcmfhcf/cst-platform.git HEAD e2f0d63d4f0e3ea5f2861acc2d7f5bb5d6fa77ff

### Classification report

PPCR: 0.675

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.983| 0.986| 0.893| 0.984| 0.936| 16195| 17877| 0.906 |
| `␣` | 0.952| 0.978| 0.594| 0.965| 0.732| 6258| 10298| 0.608 |
| `"` | 1.000| 1.000| 0.552| 1.000| 0.711| 454| 823| 0.552 |
| `⏎` | 0.925| 0.687| 0.100| 0.788| 0.181| 268| 1838| 0.146 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 75| 847| 0.089 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 49| 844| 0.058 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 24| 538| 0.045 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1477| 0.000 |
| `macro avg` | 0.482| 0.456| 0.267| 0.467| 0.320| 23323| 34542| 0.675 |
| `micro avg` | 0.974| 0.974| 0.658| 0.974| 0.785| 23323| 34542| 0.675 |
| `weighted avg` | 0.968| 0.974| 0.658| 0.971| 0.729| 23323| 34542| 0.675 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1682 |15965 |230 |0 |0 |0 |0 |0 |0 |
|4040 |140 |6118 |0 |0 |0 |0 |0 |0 |
|1570 |31 |53 |184 |0 |0 |0 |0 |0 |
|1477 |0 |0 |0 |0 |0 |0 |0 |0 |
|369 |0 |0 |0 |0 |454 |0 |0 |0 |
|772 |61 |14 |0 |0 |0 |0 |0 |0 |
|795 |44 |3 |2 |0 |0 |0 |0 |0 |
|514 |1 |10 |13 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/utils/echarts.js | 70 |
| src/components/Draggler/ReactGridLayout.js | 43 |
| src/redux/grid.js | 39 |
| src/pages/manufacturer/PropertyPanel_c.js | 36 |
| src/pages/manufacturer/index.js | 32 |
| src/pages/manufacturer/Panel/index.js | 28 |
| src/components/Draggler/GridItem.js | 24 |
| src/pages/manufacturer/PropertyPanel/InputForm.js | 22 |
| src/components/Draggler/utils.js | 20 |
| src/router/authRoute.js | 18 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 454}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.4671132615659287, "precision": 0.48241775707782386, "recall": 0.4562492356693525, "support": 23323}, "micro avg": {"f1-score": 0.9741885692235133, "precision": 0.9741885692235133, "recall": 0.9741885692235133, "support": 23323}, "weighted avg": {"f1-score": 0.9708471784212019, "precision": 0.9680057910798613, "recall": 0.9741885692235133, "support": 23323}, "\u2205": {"f1-score": 0.9843697012670715, "precision": 0.9829454500677256, "recall": 0.9857980858289596, "support": 16195}, "\u23ce": {"f1-score": 0.7880085653104925, "precision": 0.9246231155778895, "recall": 0.6865671641791045, "support": 268}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 24}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 49}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 75}, "\u2423": {"f1-score": 0.964527825949866, "precision": 0.9517734909769757, "recall": 0.9776286353467561, "support": 6258}},
  "cl_report_full": {"\"": {"f1-score": 0.7110415035238841, "precision": 1.0, "recall": 0.551640340218712, "support": 823}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1477}, "macro avg": {"f1-score": 0.31988714935889323, "precision": 0.48241775707782386, "recall": 0.26736150336476616, "support": 34542}, "micro avg": {"f1-score": 0.7853106368270976, "precision": 0.9741885692235133, "recall": 0.6577789357883157, "support": 34542}, "weighted avg": {"f1-score": 0.7289921675886547, "precision": 0.8654952378951359, "recall": 0.6577789357883157, "support": 34542}, "\u2205": {"f1-score": 0.9358421993610598, "precision": 0.9829454500677256, "recall": 0.8930469318118253, "support": 17877}, "\u23ce": {"f1-score": 0.18065783014236625, "precision": 0.9246231155778895, "recall": 0.10010881392818281, "support": 1838}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 538}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 844}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 847}, "\u2423": {"f1-score": 0.7315556618438358, "precision": 0.9517734909769757, "recall": 0.5940959409594095, "support": 10298}},
  "ppcr": 0.6752069943836488
}
```
</details>
