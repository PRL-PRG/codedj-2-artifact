# Train report for javascript / file:///tmp/top-repos-quality-repos-abiu0ipc/best-webdriver.git HEAD e01c2a3c03591cb568e8a6d982cf9a2d8f578bad

### Classification report

PPCR: 0.492

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.999| 0.998| 0.599| 0.998| 0.749| 2685| 4473| 0.600 |
| `'` | 1.000| 1.000| 0.996| 1.000| 0.998| 966| 970| 0.996 |
| `␣` | 0.962| 0.996| 0.282| 0.978| 0.436| 685| 2419| 0.283 |
| `⏎` | 0.880| 0.932| 0.334| 0.905| 0.484| 251| 701| 0.358 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 25| 271| 0.092 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 285| 0.028 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 284| 0.018 |
| `weighted avg` | 0.979| 0.986| 0.485| 0.982| 0.607| 4625| 9403| 0.492 |
| `macro avg` | 0.549| 0.561| 0.316| 0.555| 0.381| 4625| 9403| 0.492 |
| `micro avg` | 0.986| 0.986| 0.485| 0.986| 0.650| 4625| 9403| 0.492 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1788 |2680 |5 |0 |0 |0 |0 |0 |
|1734 |3 |682 |0 |0 |0 |0 |0 |
|4 |0 |0 |966 |0 |0 |0 |0 |
|450 |0 |17 |0 |234 |0 |0 |0 |
|279 |0 |2 |0 |3 |0 |0 |0 |
|277 |1 |3 |0 |4 |0 |0 |0 |
|246 |0 |0 |0 |25 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| .jsdoc.config.js | 18 |
| tests/testdrive.js | 15 |
| tests/index.js | 9 |
| src/drivers/Driver.js | 5 |
| docs/scripts/pagelocation.js | 4 |
| src/actions/Actions.js | 4 |
| src/KEY.js | 2 |
| src/actions/Pointer.js | 2 |
| docs/scripts/linenumber.js | 1 |
| src/drivers/FirefoxDriver.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 966}, "macro avg": {"f1-score": 0.554575049126017, "precision": 0.5485895899707024, "recall": 0.5608613081282774, "support": 4625}, "micro avg": {"f1-score": 0.9863783783783784, "precision": 0.9863783783783784, "recall": 0.9863783783783784, "support": 4625}, "weighted avg": {"f1-score": 0.9824796201552181, "precision": 0.9787496182256997, "recall": 0.9863783783783784, "support": 4625}, "\u2205": {"f1-score": 0.9983237101881169, "precision": 0.9985096870342772, "recall": 0.9981378026070763, "support": 2685}, "\u23ce": {"f1-score": 0.9052224371373306, "precision": 0.8796992481203008, "recall": 0.9322709163346613, "support": 251}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 25}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u2423": {"f1-score": 0.9784791965566715, "precision": 0.9619181946403385, "recall": 0.9956204379562044, "support": 685}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9979338842975207, "precision": 1.0, "recall": 0.9958762886597938, "support": 970}, "macro avg": {"f1-score": 0.3809833505562695, "precision": 0.5485895899707024, "recall": 0.31582432503237784, "support": 9403}, "micro avg": {"f1-score": 0.6504134587966924, "precision": 0.9863783783783784, "recall": 0.4851643092630012, "support": 9403}, "weighted avg": {"f1-score": 0.6074655361177994, "precision": 0.8911925040807861, "recall": 0.4851643092630012, "support": 9403}, "\u2205": {"f1-score": 0.7489171440547716, "precision": 0.9985096870342772, "recall": 0.5991504583053879, "support": 4473}, "\u23ce": {"f1-score": 0.483971044467425, "precision": 0.8796992481203008, "recall": 0.3338088445078459, "support": 701}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 271}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 284}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 285}, "\u2423": {"f1-score": 0.43606138107416875, "precision": 0.9619181946403385, "recall": 0.2819346837536172, "support": 2419}},
  "ppcr": 0.49186429862809744
}
```
</details>
