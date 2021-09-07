# Train report for javascript / file:///tmp/top-repos-quality-repos-78dwbsdh/code-fellows-coursework.git HEAD 8f350d1822a37ec7a1c377c532120772225691ad

### Classification report

PPCR: 0.804

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.980| 0.985| 0.929| 0.982| 0.954| 57842| 61321| 0.943 |
| `␣` | 0.942| 0.952| 0.708| 0.947| 0.808| 18602| 25027| 0.743 |
| `'` | 0.969| 1.000| 0.975| 0.984| 0.972| 7892| 8096| 0.975 |
| `⏎` | 0.904| 0.718| 0.104| 0.801| 0.187| 863| 5944| 0.145 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 254| 320| 0.794 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 173| 2120| 0.082 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 83| 2204| 0.038 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 29| 374| 0.078 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 23| 706| 0.033 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 334| 0.018 |
| `⏎⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 150| 0.027 |
| `⏎⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 136| 0.022 |
| `weighted avg` | 0.963| 0.970| 0.779| 0.966| 0.822| 85774| 106732| 0.804 |
| `micro avg` | 0.970| 0.970| 0.779| 0.970| 0.864| 85774| 106732| 0.804 |
| `macro avg` | 0.316| 0.305| 0.226| 0.310| 0.243| 85774| 106732| 0.804 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3479 |56964 |878 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|6425 |888 |17714 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|204 |0 |0 |7892 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|5081 |102 |141 |0 |620 |0 |0 |0 |0 |0 |0 |0 |0 |
|2121 |51 |23 |0 |9 |0 |0 |0 |0 |0 |0 |0 |0 |
|1947 |112 |20 |0 |41 |0 |0 |0 |0 |0 |0 |0 |0 |
|683 |0 |11 |0 |12 |0 |0 |0 |0 |0 |0 |0 |0 |
|66 |0 |0 |254 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|345 |16 |9 |0 |4 |0 |0 |0 |0 |0 |0 |0 |0 |
|328 |4 |2 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|146 |0 |4 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|133 |0 |3 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| first-game/client/game_files/js/lib/tween.js | 595 |
| code-blog/deployment/scripts/lib/jQueryRotate.js | 196 |
| deployment-code-blog/scripts/lib/jQueryRotate.js | 196 |
| code-blog/scripts/lib/jQueryRotate.js | 196 |
| first-game/client/game_files/js/lib/howler.core.js | 191 |
| saturday/test/unit_tests.js | 124 |
| code-blog/scripts/lib/html5sql.js | 72 |
| code-blog/deployment/scripts/objects.js | 67 |
| deployment-code-blog/scripts/objects.js | 67 |
| code-blog/scripts/objects.js | 67 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 254}, "\u0027": {"f1-score": 0.9841626137922435, "precision": 0.9688190522956052, "recall": 1.0, "support": 7892}, "macro avg": {"f1-score": 0.3095075252918847, "precision": 0.31620134792393734, "recall": 0.3046256681653799, "support": 85774}, "micro avg": {"f1-score": 0.9698743208897801, "precision": 0.9698743208897801, "recall": 0.9698743208897801, "support": 85774}, "weighted avg": {"f1-score": 0.966433129972024, "precision": 0.9632711569258545, "recall": 0.9698743208897801, "support": 85774}, "\u2205": {"f1-score": 0.9823157640607352, "precision": 0.9798235203054854, "recall": 0.9848207185090418, "support": 57842}, "\u23ce": {"f1-score": 0.800516462233699, "precision": 0.9037900874635568, "recall": 0.7184241019698725, "support": 863}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 23}, "\u23ce\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 83}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 173}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 29}, "\u2423": {"f1-score": 0.9470954634159382, "precision": 0.9419835150226004, "recall": 0.9522631975056446, "support": 18602}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 320}, "\u0027": {"f1-score": 0.9718015022780446, "precision": 0.9688190522956052, "recall": 0.974802371541502, "support": 8096}, "macro avg": {"f1-score": 0.24340047260268372, "precision": 0.31620134792393734, "recall": 0.22632104043363163, "support": 106732}, "micro avg": {"f1-score": 0.8642847495662473, "precision": 0.9698743208897801, "recall": 0.7794288498294795, "support": 106732}, "weighted avg": {"f1-score": 0.8215926554944475, "precision": 0.907642195839972, "recall": 0.7794288498294795, "support": 106732}, "\u2205": {"f1-score": 0.9537075792328685, "precision": 0.9798235203054854, "recall": 0.9289476688247094, "support": 61321}, "\u23ce": {"f1-score": 0.1870286576168929, "precision": 0.9037900874635568, "recall": 0.10430686406460296, "support": 5944}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 706}, "\u23ce\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 136}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 150}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2204}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 334}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2120}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 374}, "\u2423": {"f1-score": 0.8082679321043986, "precision": 0.9419835150226004, "recall": 0.7077955807727654, "support": 25027}},
  "ppcr": 0.8036390210995765
}
```
</details>
