# Train report for javascript / file:///tmp/top-repos-quality-repos-fbchg_em/whats-new-in-python3.git HEAD 09af9d8a0f446f384866bb06b40fd9e26b9a115d

### Classification report

PPCR: 0.649

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.978| 0.994| 0.892| 0.986| 0.933| 17728| 19749| 0.898 |
| `␣` | 0.973| 0.962| 0.575| 0.967| 0.723| 10073| 16851| 0.598 |
| `'` | 0.927| 1.000| 0.684| 0.962| 0.787| 2077| 3035| 0.684 |
| `⏎` | 0.968| 0.788| 0.155| 0.869| 0.267| 499| 2540| 0.196 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 164| 780| 0.210 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 58| 1230| 0.047 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 19| 1342| 0.014 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 11| 1135| 0.010 |
| `⏎⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 242| 0.012 |
| `⏎⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 305| 0.000 |
| `micro avg` | 0.972| 0.972| 0.631| 0.972| 0.765| 30632| 47209| 0.649 |
| `weighted avg` | 0.964| 0.972| 0.631| 0.968| 0.713| 30632| 47209| 0.649 |
| `macro avg` | 0.385| 0.374| 0.231| 0.378| 0.271| 30632| 47209| 0.649 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎⏎| ⏎⇥⁻| ⏎⇥⁺| "| ⏎⏎⇥⁺| ⏎⏎⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2021 |17618 |110 |0 |0 |0 |0 |0 |0 |0 |0 |
|6778 |382 |9691 |0 |0 |0 |0 |0 |0 |0 |0 |
|2041 |14 |92 |393 |0 |0 |0 |0 |0 |0 |0 |
|958 |0 |0 |0 |2077 |0 |0 |0 |0 |0 |0 |
|1323 |3 |3 |13 |0 |0 |0 |0 |0 |0 |0 |
|1124 |6 |5 |0 |0 |0 |0 |0 |0 |0 |0 |
|1172 |0 |58 |0 |0 |0 |0 |0 |0 |0 |0 |
|616 |0 |0 |0 |164 |0 |0 |0 |0 |0 |0 |
|239 |0 |3 |0 |0 |0 |0 |0 |0 |0 |0 |
|305 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| test/qunit-1.12.0.js | 521 |
| js/reveal.js | 187 |
| test/test.js | 37 |
| plugin/markdown/markdown.js | 27 |
| Gruntfile.js | 24 |
| plugin/zoom-js/zoom.js | 14 |
| plugin/multiplex/index.js | 9 |
| plugin/math/math.js | 8 |
| plugin/notes-server/index.js | 7 |
| plugin/notes/notes.js | 6 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 164}, "\u0027": {"f1-score": 0.9620194534506716, "precision": 0.92681838464971, "recall": 1.0, "support": 2077}, "macro avg": {"f1-score": 0.3783529586964366, "precision": 0.3845124020710279, "recall": 0.3743447115729146, "support": 30632}, "micro avg": {"f1-score": 0.9721533037346566, "precision": 0.9721533037346566, "recall": 0.9721533037346566, "support": 30632}, "weighted avg": {"f1-score": 0.9679033656473113, "precision": 0.9642411010575804, "recall": 0.9721533037346566, "support": 30632}, "\u2205": {"f1-score": 0.9855948085368241, "precision": 0.9775287133107696, "recall": 0.9937951263537906, "support": 17728}, "\u23ce": {"f1-score": 0.8685082872928177, "precision": 0.9679802955665024, "recall": 0.7875751503006012, "support": 499}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 58}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 19}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9674070376840529, "precision": 0.9727966271832965, "recall": 0.9620768390747543, "support": 10073}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 780}, "\u0027": {"f1-score": 0.7873388931008338, "precision": 0.92681838464971, "recall": 0.6843492586490939, "support": 3035}, "macro avg": {"f1-score": 0.2709859971902763, "precision": 0.3845124020710279, "recall": 0.23062688710460605, "support": 47209}, "micro avg": {"f1-score": 0.7651237779576316, "precision": 0.9721533037346566, "recall": 0.630790739053994, "support": 47209}, "weighted avg": {"f1-score": 0.7132367910844286, "precision": 0.8678297416168719, "recall": 0.630790739053994, "support": 47209}, "\u2205": {"f1-score": 0.9328603198136187, "precision": 0.9775287133107696, "recall": 0.8920958023191048, "support": 19749}, "\u23ce": {"f1-score": 0.2668024439918534, "precision": 0.9679802955665024, "recall": 0.1547244094488189, "support": 2540}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1230}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1135}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1342}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 242}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 305}, "\u2423": {"f1-score": 0.7228583149964569, "precision": 0.9727966271832965, "recall": 0.5750994006290427, "support": 16851}},
  "ppcr": 0.6488593276705713
}
```
</details>
