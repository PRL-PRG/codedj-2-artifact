# Train report for javascript / file:///tmp/top-repos-quality-repos-zlnskj7g/bookable.git HEAD 82fe398320ce73f2895f576907461013ba1e454c

### Classification report

PPCR: 0.221

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.934| 0.998| 0.276| 0.965| 0.426| 412| 1489| 0.277 |
| `␣` | 0.994| 1.000| 0.163| 0.997| 0.280| 171| 1051| 0.163 |
| `'` | 1.000| 1.000| 0.500| 1.000| 0.667| 127| 254| 0.500 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 18| 228| 0.079 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 11| 183| 0.060 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 140| 0.000 |
| `weighted avg` | 0.923| 0.959| 0.212| 0.940| 0.328| 739| 3345| 0.221 |
| `micro avg` | 0.959| 0.959| 0.212| 0.959| 0.347| 739| 3345| 0.221 |
| `macro avg` | 0.488| 0.500| 0.156| 0.494| 0.229| 739| 3345| 0.221 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|1077 |411 |1 |0 |0 |0 |0 |
|880 |0 |171 |0 |0 |0 |0 |
|127 |0 |0 |127 |0 |0 |0 |
|210 |18 |0 |0 |0 |0 |0 |
|172 |11 |0 |0 |0 |0 |0 |
|140 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| resources/js/serviceWorker.js | 5 |
| resources/js/actions/wishlist.js | 4 |
| resources/js/actions/cart.js | 4 |
| resources/js/utils/path.js | 4 |
| resources/js/actions/categories.js | 3 |
| resources/js/actions/books.js | 3 |
| webpack.mix.js | 3 |
| resources/js/actions/user.js | 3 |
| resources/js/app.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 127}, "macro avg": {"f1-score": 0.49364554674988703, "precision": 0.4880461592670895, "recall": 0.49959546925566345, "support": 739}, "micro avg": {"f1-score": 0.959404600811908, "precision": 0.959404600811908, "recall": 0.959404600811908, "support": 739}, "weighted avg": {"f1-score": 0.9404525243199274, "precision": 0.9226674810540499, "recall": 0.959404600811908, "support": 739}, "\u2205": {"f1-score": 0.9647887323943661, "precision": 0.9340909090909091, "recall": 0.9975728155339806, "support": 412}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 18}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9970845481049563, "precision": 0.9941860465116279, "recall": 1.0, "support": 171}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 254}, "macro avg": {"f1-score": 0.22873907047134293, "precision": 0.4880461592670895, "recall": 0.15645439428203486, "support": 3345}, "micro avg": {"f1-score": 0.34720861900097943, "precision": 0.959404600811908, "recall": 0.21195814648729447, "support": 3345}, "weighted avg": {"f1-score": 0.32817312465759024, "precision": 0.8041108814708774, "recall": 0.21195814648729447, "support": 3345}, "\u2205": {"f1-score": 0.4261275272161742, "precision": 0.9340909090909091, "recall": 0.27602417730020146, "support": 1489}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 228}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 183}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 140}, "\u2423": {"f1-score": 0.2796402289452167, "precision": 0.9941860465116279, "recall": 0.16270218839200762, "support": 1051}},
  "ppcr": 0.22092675635276532
}
```
</details>
