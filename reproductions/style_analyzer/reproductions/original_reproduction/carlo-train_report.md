# Train report for javascript / file:///tmp/top-repos-quality-repos-4x8si5l7/carlo HEAD b8ce2bca042c757b13fc82a3e059980342ddd9a8

### Classification report

PPCR: 0.789

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.978| 0.981| 0.904| 0.979| 0.940| 3985| 4324| 0.922 |
| `␣` | 0.893| 0.967| 0.806| 0.928| 0.847| 1763| 2114| 0.834 |
| `'` | 1.000| 1.000| 0.746| 1.000| 0.854| 461| 618| 0.746 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 65| 401| 0.162 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 42| 253| 0.166 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 35| 148| 0.236 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 17| 210| 0.081 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `weighted avg` | 0.932| 0.954| 0.753| 0.942| 0.791| 6368| 8068| 0.789 |
| `macro avg` | 0.359| 0.368| 0.307| 0.363| 0.330| 6368| 8068| 0.789 |
| `micro avg` | 0.954| 0.954| 0.753| 0.954| 0.842| 6368| 8068| 0.789 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|339 |3910 |75 |0 |0 |0 |0 |0 |
|351 |59 |1704 |0 |0 |0 |0 |0 |
|157 |0 |0 |461 |0 |0 |0 |0 |
|336 |10 |55 |0 |0 |0 |0 |0 |
|211 |6 |36 |0 |0 |0 |0 |0 |
|193 |13 |4 |0 |0 |0 |0 |0 |
|113 |1 |34 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| lib/color.js | 80 |
| lib/find-chrome.js | 61 |
| lib/rpc.js | 57 |
| lib/carlo.js | 52 |
| lib/intercepted_request.js | 20 |
| examples/terminal/main.js | 14 |
| examples/systeminfo/main.js | 8 |
| index.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 461}, "macro avg": {"f1-score": 0.3634770392050787, "precision": 0.3588532746394146, "recall": 0.3684642174176987, "support": 6368}, "micro avg": {"f1-score": 0.9539886934673367, "precision": 0.9539886934673367, "recall": 0.9539886934673367, "support": 6368}, "weighted avg": {"f1-score": 0.9423426313409691, "precision": 0.9315035682394098, "recall": 0.9539886934673367, "support": 6368}, "\u2205": {"f1-score": 0.9794589178356714, "precision": 0.9777444361090273, "recall": 0.9811794228356336, "support": 3985}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 65}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 35}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 42}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 17}, "\u2423": {"f1-score": 0.9283573958049578, "precision": 0.8930817610062893, "recall": 0.9665343165059558, "support": 1763}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.8544949026876737, "precision": 1.0, "recall": 0.7459546925566343, "support": 618}, "macro avg": {"f1-score": 0.33017494942334746, "precision": 0.3588532746394146, "recall": 0.30703311049820103, "support": 8068}, "micro avg": {"f1-score": 0.841645885286783, "precision": 0.9539886934673367, "recall": 0.7529747149231532, "support": 8068}, "weighted avg": {"f1-score": 0.791030386008821, "precision": 0.8346234239591881, "recall": 0.7529747149231532, "support": 8068}, "\u2205": {"f1-score": 0.9395650606752374, "precision": 0.9777444361090273, "recall": 0.9042553191489362, "support": 4324}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 401}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 148}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 253}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 210}, "\u2423": {"f1-score": 0.8473396320238687, "precision": 0.8930817610062893, "recall": 0.8060548722800378, "support": 2114}},
  "ppcr": 0.7892910262766485
}
```
</details>
