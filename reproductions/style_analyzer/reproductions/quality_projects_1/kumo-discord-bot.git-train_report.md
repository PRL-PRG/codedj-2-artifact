# Train report for javascript / file:///tmp/top-repos-quality-repos-17d8geqp/kumo-discord-bot.git HEAD 95eb2f0cca8a5fceeb3025dd4cc13d3c10a32c29

### Classification report

PPCR: 0.706

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.994| 0.999| 0.948| 0.997| 0.970| 13996| 14752| 0.949 |
| `␣` | 0.993| 0.999| 0.480| 0.996| 0.647| 3502| 7281| 0.481 |
| `⏎` | 0.962| 0.980| 0.729| 0.971| 0.829| 1047| 1408| 0.744 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 1.000| 0.798| 0.607| 0.888| 0.756| 387| 509| 0.760 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 21| 160| 0.131 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 15| 544| 0.028 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1578| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 647| 0.000 |
| `macro avg` | 0.494| 0.472| 0.346| 0.481| 0.400| 18968| 26879| 0.706 |
| `micro avg` | 0.992| 0.992| 0.700| 0.992| 0.821| 18968| 26879| 0.706 |
| `weighted avg` | 0.990| 0.992| 0.700| 0.991| 0.766| 18968| 26879| 0.706 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| '| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|756 |13984 |12 |0 |0 |0 |0 |0 |0 |
|3779 |3 |3498 |0 |1 |0 |0 |0 |0 |
|1578 |0 |0 |0 |0 |0 |0 |0 |0 |
|361 |11 |10 |0 |1026 |0 |0 |0 |0 |
|647 |0 |0 |0 |0 |0 |0 |0 |0 |
|529 |8 |4 |0 |3 |0 |0 |0 |0 |
|122 |63 |0 |0 |15 |0 |0 |309 |0 |
|139 |0 |0 |0 |21 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| commands/coursemology.js | 38 |
| commands/music.js | 23 |
| commands/anime.js | 17 |
| bot.js | 13 |
| db/user.js | 10 |
| commands/gomoku.js | 10 |
| commands/covid.js | 8 |
| commands/minesweeper.js | 7 |
| commands/solver24.js | 7 |
| commands/minecraft.js | 7 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.4814170013302302, "precision": 0.49363211455187916, "recall": 0.47204908919164407, "support": 18968}, "micro avg": {"f1-score": 0.9920392239561366, "precision": 0.9920392239561366, "recall": 0.9920392239561366, "support": 18968}, "weighted avg": {"f1-score": 0.990883418754442, "precision": 0.9902106865102903, "recall": 0.9920392239561366, "support": 18968}, "\u2205": {"f1-score": 0.9965437377516478, "precision": 0.9939583481413036, "recall": 0.9991426121749071, "support": 13996}, "\u23ce": {"f1-score": 0.9711310932323711, "precision": 0.9624765478424016, "recall": 0.9799426934097422, "support": 1047}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 21}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 15}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.8879310344827586, "precision": 1.0, "recall": 0.7984496124031008, "support": 387}, "\u2423": {"f1-score": 0.9957301451750641, "precision": 0.9926220204313281, "recall": 0.9988577955454027, "support": 3502}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1578}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 647}, "macro avg": {"f1-score": 0.4003510997320164, "precision": 0.49363211455187916, "recall": 0.34551670605126056, "support": 26879}, "micro avg": {"f1-score": 0.8208606888127904, "precision": 0.9920392239561366, "recall": 0.7000632464005357, "support": 26879}, "weighted avg": {"f1-score": 0.765730207108531, "precision": 0.8837501939024186, "recall": 0.7000632464005357, "support": 26879}, "\u2205": {"f1-score": 0.9704035252073142, "precision": 0.9939583481413036, "recall": 0.9479392624728851, "support": 14752}, "\u23ce": {"f1-score": 0.8294260307194825, "precision": 0.9624765478424016, "recall": 0.7286931818181818, "support": 1408}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 160}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 544}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.7555012224938876, "precision": 1.0, "recall": 0.6070726915520629, "support": 509}, "\u2423": {"f1-score": 0.6474780194354466, "precision": 0.9926220204313281, "recall": 0.48042851256695507, "support": 7281}},
  "ppcr": 0.7056810149187098
}
```
</details>
