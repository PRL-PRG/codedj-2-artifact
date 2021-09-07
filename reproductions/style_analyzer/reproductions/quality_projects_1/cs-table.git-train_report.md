# Train report for javascript / file:///tmp/top-repos-quality-repos-7lawyjuc/cs-table.git HEAD 17226957035448b5b3ad84f54d8dff261d473dbb

### Classification report

PPCR: 0.267

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `␣` | 0.996| 0.493| 0.148| 0.660| 0.258| 969| 3224| 0.301 |
| `∅` | 0.890| 0.998| 0.172| 0.941| 0.289| 939| 5440| 0.173 |
| `⏎` | 0.521| 0.794| 0.418| 0.629| 0.464| 506| 961| 0.527 |
| `'` | 1.000| 1.000| 0.500| 1.000| 0.667| 411| 822| 0.500 |
| `⏎␣⁻␣⁻` | 0.245| 0.430| 0.180| 0.312| 0.207| 165| 395| 0.418 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 16| 407| 0.039 |
| `macro avg` | 0.609| 0.619| 0.236| 0.590| 0.314| 3006| 11249| 0.267 |
| `micro avg` | 0.765| 0.765| 0.204| 0.765| 0.323| 3006| 11249| 0.267 |
| `weighted avg` | 0.837| 0.765| 0.204| 0.766| 0.309| 3006| 11249| 0.267 |

### Confusion matrix

|refusal|  ∅| '| ␣| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|4501 |937 |0 |2 |0 |0 |0 |
|411 |0 |411 |0 |0 |0 |0 |
|2255 |105 |0 |478 |271 |0 |115 |
|455 |0 |0 |0 |402 |0 |104 |
|391 |11 |0 |0 |5 |0 |0 |
|230 |0 |0 |0 |94 |0 |71 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/ctable/index.js | 125 |
| src/stable/index.js | 116 |
| src/ctable/CTable.js | 65 |
| src/stable/useRowHeader.js | 51 |
| src/stable/useColHeader.js | 49 |
| src/stable/util.js | 43 |
| src/stable/RowHeader.js | 33 |
| src/ctable/Header.js | 32 |
| src/stable/ColHeader.js | 30 |
| src/ctable/DataArea.js | 25 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 411}, "macro avg": {"f1-score": 0.5902873834891555, "precision": 0.6085374774410816, "recall": 0.6193219269460079, "support": 3006}, "micro avg": {"f1-score": 0.7648037258815702, "precision": 0.7648037258815702, "recall": 0.7648037258815702, "support": 3006}, "weighted avg": {"f1-score": 0.7663048168382703, "precision": 0.8367945784812599, "recall": 0.7648037258815702, "support": 3006}, "\u2205": {"f1-score": 0.9407630522088353, "precision": 0.8898385565052231, "recall": 0.9978700745473909, "support": 939}, "\u23ce": {"f1-score": 0.6291079812206573, "precision": 0.5207253886010362, "recall": 0.7944664031620553, "support": 506}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.31208791208791203, "precision": 0.24482758620689654, "recall": 0.4303030303030303, "support": 165}, "\u2423": {"f1-score": 0.6597653554175293, "precision": 0.9958333333333333, "recall": 0.4932920536635707, "support": 969}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 822}, "macro avg": {"f1-score": 0.31410319553706817, "precision": 0.6085374774410816, "recall": 0.23642779429674962, "support": 11249}, "micro avg": {"f1-score": 0.3225534900035075, "precision": 0.7648037258815702, "recall": 0.2043737221086319, "support": 11249}, "weighted avg": {"f1-score": 0.30917611538561063, "precision": 0.8418892709620768, "recall": 0.2043737221086319, "support": 11249}, "\u2205": {"f1-score": 0.28861851224395507, "precision": 0.8898385565052231, "recall": 0.17224264705882353, "support": 5440}, "\u23ce": {"f1-score": 0.463935372186959, "precision": 0.5207253886010362, "recall": 0.4183142559833507, "support": 961}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 407}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.20729927007299265, "precision": 0.24482758620689654, "recall": 0.17974683544303796, "support": 395}, "\u2423": {"f1-score": 0.2580993520518358, "precision": 0.9958333333333333, "recall": 0.14826302729528537, "support": 3224}},
  "ppcr": 0.2672237532225087
}
```
</details>
