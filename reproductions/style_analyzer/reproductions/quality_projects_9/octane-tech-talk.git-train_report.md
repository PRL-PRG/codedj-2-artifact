# Train report for javascript / file:///tmp/top-repos-quality-repos-yb5_97j3/octane-tech-talk.git HEAD a1c551bb88d0274b12e83fe4cf90fc86603b397d

### Classification report

PPCR: 0.670

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.994| 0.991| 0.853| 0.993| 0.918| 24169| 28098| 0.860 |
| `␣` | 0.979| 0.991| 0.677| 0.985| 0.800| 14186| 20778| 0.683 |
| `'` | 0.996| 1.000| 0.651| 0.998| 0.788| 2039| 3130| 0.651 |
| `⏎` | 0.976| 0.948| 0.375| 0.962| 0.542| 1354| 3424| 0.395 |
| `"` | 1.000| 0.933| 0.075| 0.966| 0.139| 120| 1495| 0.080 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 33| 1854| 0.018 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 26| 1579| 0.016 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 1591| 0.006 |
| `⏎⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 326| 0.009 |
| `⏎⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 293| 0.000 |
| `weighted avg` | 0.987| 0.989| 0.663| 0.988| 0.750| 41939| 62568| 0.670 |
| `macro avg` | 0.495| 0.486| 0.263| 0.490| 0.319| 41939| 62568| 0.670 |
| `micro avg` | 0.989| 0.989| 0.663| 0.989| 0.793| 41939| 62568| 0.670 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎⏎| ⏎⇥⁻| ⏎⇥⁺| "| ⏎⏎⇥⁺| ⏎⏎⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3929 |23961 |204 |4 |0 |0 |0 |0 |0 |0 |0 |
|6592 |125 |14061 |0 |0 |0 |0 |0 |0 |0 |0 |
|2070 |14 |56 |1284 |0 |0 |0 |0 |0 |0 |0 |
|1091 |0 |0 |0 |2039 |0 |0 |0 |0 |0 |0 |
|1821 |3 |3 |27 |0 |0 |0 |0 |0 |0 |0 |
|1582 |4 |5 |0 |0 |0 |0 |0 |0 |0 |0 |
|1553 |0 |26 |0 |0 |0 |0 |0 |0 |0 |0 |
|1375 |0 |0 |0 |8 |0 |0 |0 |112 |0 |0 |
|323 |0 |3 |0 |0 |0 |0 |0 |0 |0 |0 |
|293 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| js/reveal.js | 183 |
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
  "cl_report": {"\"": {"f1-score": 0.9655172413793104, "precision": 1.0, "recall": 0.9333333333333333, "support": 120}, "\u0027": {"f1-score": 0.9980420949583945, "precision": 0.9960918417195896, "recall": 1.0, "support": 2039}, "macro avg": {"f1-score": 0.49036004189730004, "precision": 0.49457760328282535, "recall": 0.4864217092806456, "support": 41939}, "micro avg": {"f1-score": 0.9885071174801497, "precision": 0.9885071174801497, "recall": 0.9885071174801497, "support": 41939}, "weighted avg": {"f1-score": 0.9876648491004533, "precision": 0.9868688871498467, "recall": 0.9885071174801497, "support": 41939}, "\u2205": {"f1-score": 0.9926671638081034, "precision": 0.9939436678143277, "recall": 0.9913939343787497, "support": 24169}, "\u23ce": {"f1-score": 0.9621581116523042, "precision": 0.976425855513308, "recall": 0.948301329394387, "support": 1354}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 26}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 33}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9852158071748879, "precision": 0.979314667781028, "recall": 0.9911884956999859, "support": 14186}},
  "cl_report_full": {"\"": {"f1-score": 0.13939016801493465, "precision": 1.0, "recall": 0.07491638795986622, "support": 1495}, "\u0027": {"f1-score": 0.787714892795055, "precision": 0.9960918417195896, "recall": 0.6514376996805111, "support": 3130}, "macro avg": {"f1-score": 0.31873252678049596, "precision": 0.49457760328282535, "recall": 0.2630844791631793, "support": 62568}, "micro avg": {"f1-score": 0.793382261475308, "precision": 0.9885071174801497, "recall": 0.6625911008822402, "support": 62568}, "weighted avg": {"f1-score": 0.7504210848003549, "precision": 0.8987354388706857, "recall": 0.6625911008822402, "support": 62568}, "\u2205": {"f1-score": 0.9179580499952111, "precision": 0.9939436678143277, "recall": 0.8527653213751868, "support": 28098}, "\u23ce": {"f1-score": 0.5418864739396496, "precision": 0.976425855513308, "recall": 0.375, "support": 3424}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1579}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1591}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1854}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 326}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 293}, "\u2423": {"f1-score": 0.8003756830601093, "precision": 0.979314667781028, "recall": 0.6767253826162287, "support": 20778}},
  "ppcr": 0.6702947193453522
}
```
</details>
