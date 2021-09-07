# Train report for javascript / file:///tmp/top-repos-quality-repos-_1j9kygo/slurmprio200428.git HEAD 6fd1dead667cb3aa5d5f5bc3d8a2920f396cb643

### Classification report

PPCR: 0.670

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.994| 0.991| 0.853| 0.993| 0.918| 24276| 28213| 0.860 |
| `␣` | 0.979| 0.991| 0.676| 0.985| 0.800| 14241| 20878| 0.682 |
| `'` | 0.996| 1.000| 0.651| 0.998| 0.788| 2048| 3144| 0.651 |
| `⏎` | 0.977| 0.949| 0.377| 0.963| 0.544| 1367| 3443| 0.397 |
| `"` | 1.000| 0.933| 0.075| 0.966| 0.139| 120| 1495| 0.080 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 33| 1864| 0.018 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 26| 1586| 0.016 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 1597| 0.006 |
| `⏎⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 330| 0.009 |
| `⏎⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 298| 0.000 |
| `weighted avg` | 0.987| 0.989| 0.663| 0.988| 0.750| 42123| 62848| 0.670 |
| `macro avg` | 0.495| 0.486| 0.263| 0.490| 0.319| 42123| 62848| 0.670 |
| `micro avg` | 0.989| 0.989| 0.663| 0.989| 0.793| 42123| 62848| 0.670 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎⏎| ⏎⇥⁻| ⏎⇥⁺| "| ⏎⏎⇥⁺| ⏎⏎⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3937 |24068 |204 |4 |0 |0 |0 |0 |0 |0 |0 |
|6637 |126 |14115 |0 |0 |0 |0 |0 |0 |0 |0 |
|2076 |14 |56 |1297 |0 |0 |0 |0 |0 |0 |0 |
|1096 |0 |0 |0 |2048 |0 |0 |0 |0 |0 |0 |
|1831 |3 |3 |27 |0 |0 |0 |0 |0 |0 |0 |
|1588 |4 |5 |0 |0 |0 |0 |0 |0 |0 |0 |
|1560 |0 |26 |0 |0 |0 |0 |0 |0 |0 |0 |
|1375 |0 |0 |0 |8 |0 |0 |0 |112 |0 |0 |
|327 |0 |3 |0 |0 |0 |0 |0 |0 |0 |0 |
|298 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| js/reveal.js | 184 |
| test/qunit-2.5.0.js | 169 |
| Gruntfile.js | 30 |
| test/test.js | 26 |
| plugin/markdown/markdown.js | 24 |
| plugin/zoom-js/zoom.js | 10 |
| plugin/multiplex/index.js | 9 |
| plugin/notes-server/index.js | 7 |
| plugin/math/math.js | 6 |
| plugin/notes/notes.js | 5 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9655172413793104, "precision": 1.0, "recall": 0.9333333333333333, "support": 120}, "\u0027": {"f1-score": 0.9980506822612086, "precision": 0.9961089494163424, "recall": 1.0, "support": 2048}, "macro avg": {"f1-score": 0.49040073165228054, "precision": 0.49460871317254573, "recall": 0.4864710484240364, "support": 42123}, "micro avg": {"f1-score": 0.9885335802293284, "precision": 0.9885335802293284, "recall": 0.9885335802293284, "support": 42123}, "weighted avg": {"f1-score": 0.9876950360481446, "precision": 0.9869020337478942, "recall": 0.9885335802293284, "support": 42123}, "\u2205": {"f1-score": 0.9926790538450434, "precision": 0.9939293826140821, "recall": 0.9914318668643928, "support": 24276}, "\u23ce": {"f1-score": 0.9625231910946197, "precision": 0.9766566265060241, "recall": 0.9487929773226043, "support": 1367}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 26}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 33}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9852371479426237, "precision": 0.9793921731890092, "recall": 0.9911523067200337, "support": 14241}},
  "cl_report_full": {"\"": {"f1-score": 0.13939016801493465, "precision": 1.0, "recall": 0.07491638795986622, "support": 1495}, "\u0027": {"f1-score": 0.7876923076923077, "precision": 0.9961089494163424, "recall": 0.6513994910941476, "support": 3144}, "macro avg": {"f1-score": 0.31888626800304687, "precision": 0.49460871317254573, "recall": 0.2632174657217047, "support": 62848}, "micro avg": {"f1-score": 0.7933619761648454, "precision": 0.9885335802293284, "recall": 0.662550916496945, "support": 62848}, "weighted avg": {"f1-score": 0.7504045732499893, "precision": 0.8986582670181463, "recall": 0.662550916496945, "support": 62848}, "\u2205": {"f1-score": 0.9181353475242237, "precision": 0.9939293826140821, "recall": 0.8530819125934853, "support": 28213}, "\u23ce": {"f1-score": 0.5437015300775518, "precision": 0.9766566265060241, "recall": 0.3767063607319198, "support": 3443}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1586}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1597}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1864}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 330}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 298}, "\u2423": {"f1-score": 0.7999433267214509, "precision": 0.9793921731890092, "recall": 0.6760705048376281, "support": 20878}},
  "ppcr": 0.6702361252545825
}
```
</details>
