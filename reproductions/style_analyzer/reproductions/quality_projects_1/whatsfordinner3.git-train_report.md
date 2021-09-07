# Train report for javascript / file:///tmp/top-repos-quality-repos-yyo5obn_/whatsfordinner3.git HEAD 52f4c0e9e4995cd22746d0c8bb8c28c7a26f5342

### Classification report

PPCR: 0.532

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.964| 1.000| 0.774| 0.982| 0.859| 4150| 5362| 0.774 |
| `"` | 1.000| 1.000| 0.997| 1.000| 0.999| 372| 373| 0.997 |
| `␣` | 1.000| 0.650| 0.089| 0.788| 0.164| 306| 2226| 0.137 |
| `'` | 1.000| 1.000| 0.500| 1.000| 0.667| 285| 570| 0.500 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 16| 118| 0.136 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 13| 148| 0.088 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 499| 0.014 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 146| 0.048 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 126| 0.040 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 130| 0.000 |
| `macro avg` | 0.396| 0.365| 0.236| 0.377| 0.269| 5161| 9698| 0.532 |
| `micro avg` | 0.970| 0.970| 0.516| 0.970| 0.674| 5161| 9698| 0.532 |
| `weighted avg` | 0.962| 0.970| 0.516| 0.963| 0.590| 5161| 9698| 0.532 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⇥⁻| ⏎⇥⁺| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1212 |4150 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1920 |107 |199 |0 |0 |0 |0 |0 |0 |0 |0 |
|285 |0 |0 |285 |0 |0 |0 |0 |0 |0 |0 |
|492 |7 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1 |0 |0 |0 |0 |372 |0 |0 |0 |0 |0 |
|121 |5 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|102 |16 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|135 |13 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|139 |7 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|130 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/Recipes/SingleRecipe.js | 29 |
| src/List/List.js | 23 |
| src/Recipes/Recipes.js | 17 |
| src/FavoriteRecipes/RecipeInstructions.js | 14 |
| src/FavoriteRecipes/FavoriteRecipes.js | 12 |
| src/App.js | 11 |
| src/serviceWorker.js | 10 |
| src/Navbar.js | 8 |
| src/User/SignUp.js | 5 |
| src/User/Login.js | 5 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 372}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 285}, "macro avg": {"f1-score": 0.37697864641579476, "precision": 0.3963995354239257, "recall": 0.36503267973856207, "support": 5161}, "micro avg": {"f1-score": 0.9699670606471614, "precision": 0.9699670606471614, "recall": 0.9699670606471614, "support": 5161}, "weighted avg": {"f1-score": 0.9633956817252847, "precision": 0.9617478628352869, "recall": 0.9699670606471614, "support": 5161}, "\u2205": {"f1-score": 0.9816676522767592, "precision": 0.9639953542392566, "recall": 1.0, "support": 4150}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 13}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u2423": {"f1-score": 0.7881188118811882, "precision": 1.0, "recall": 0.6503267973856209, "support": 306}},
  "cl_report_full": {"\"": {"f1-score": 0.9986577181208054, "precision": 1.0, "recall": 0.9973190348525469, "support": 373}, "\u0027": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 570}, "macro avg": {"f1-score": 0.26880391791937797, "precision": 0.3963995354239257, "recall": 0.23606819966686343, "support": 9698}, "micro avg": {"f1-score": 0.6738003903358235, "precision": 0.9699670606471614, "recall": 0.5161889049288513, "support": 9698}, "weighted avg": {"f1-score": 0.5899777374409976, "precision": 0.8597590317004428, "recall": 0.5161889049288513, "support": 9698}, "\u2205": {"f1-score": 0.8585910830661011, "precision": 0.9639953542392566, "recall": 0.7739649384558001, "support": 5362}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 499}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 146}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 148}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 130}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 126}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 118}, "\u2423": {"f1-score": 0.16412371134020617, "precision": 1.0, "recall": 0.08939802336028752, "support": 2226}},
  "ppcr": 0.532171581769437
}
```
</details>
