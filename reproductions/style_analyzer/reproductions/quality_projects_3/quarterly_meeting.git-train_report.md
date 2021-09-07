# Train report for javascript / file:///tmp/top-repos-quality-repos-ejy812t_/quarterly_meeting.git HEAD 33bed47daca3f08c396215415e6ece005970734a

### Classification report

PPCR: 0.658

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.978| 0.994| 0.892| 0.986| 0.933| 17710| 19749| 0.897 |
| `␣` | 0.980| 0.963| 0.577| 0.971| 0.727| 10098| 16851| 0.599 |
| `'` | 0.926| 1.000| 0.658| 0.961| 0.769| 2019| 3068| 0.658 |
| `⏎` | 0.980| 0.918| 0.368| 0.948| 0.536| 1014| 2527| 0.401 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 162| 800| 0.203 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 25| 1230| 0.020 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 24| 1338| 0.018 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 11| 1135| 0.010 |
| `⏎⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 242| 0.012 |
| `⏎⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 305| 0.000 |
| `macro avg` | 0.386| 0.388| 0.250| 0.387| 0.296| 31066| 47245| 0.658 |
| `weighted avg` | 0.968| 0.975| 0.641| 0.971| 0.728| 31066| 47245| 0.658 |
| `micro avg` | 0.975| 0.975| 0.641| 0.975| 0.774| 31066| 47245| 0.658 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎⏎| ⏎⇥⁻| ⏎⇥⁺| "| ⏎⏎⇥⁺| ⏎⏎⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2039 |17612 |98 |0 |0 |0 |0 |0 |0 |0 |0 |
|6753 |369 |9729 |0 |0 |0 |0 |0 |0 |0 |0 |
|1513 |14 |69 |931 |0 |0 |0 |0 |0 |0 |0 |
|1049 |0 |0 |0 |2019 |0 |0 |0 |0 |0 |0 |
|1314 |3 |3 |18 |0 |0 |0 |0 |0 |0 |0 |
|1124 |6 |5 |0 |0 |0 |0 |0 |0 |0 |0 |
|1205 |0 |24 |1 |0 |0 |0 |0 |0 |0 |0 |
|638 |0 |0 |0 |162 |0 |0 |0 |0 |0 |0 |
|239 |0 |3 |0 |0 |0 |0 |0 |0 |0 |0 |
|305 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| test/qunit-1.12.0.js | 478 |
| js/reveal.js | 163 |
| Gruntfile.js | 30 |
| test/test.js | 24 |
| plugin/markdown/markdown.js | 23 |
| plugin/multiplex/index.js | 12 |
| plugin/zoom-js/zoom.js | 10 |
| plugin/notes-server/index.js | 7 |
| plugin/math/math.js | 6 |
| plugin/print-pdf/print-pdf.js | 5 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 162}, "\u0027": {"f1-score": 0.9614285714285714, "precision": 0.9257221458046767, "recall": 1.0, "support": 2019}, "macro avg": {"f1-score": 0.38672649723443203, "precision": 0.38636088580538774, "recall": 0.38760704702864845, "support": 31066}, "micro avg": {"f1-score": 0.975053112727741, "precision": 0.975053112727741, "recall": 0.975053112727741, "support": 31066}, "weighted avg": {"f1-score": 0.9714671587611909, "precision": 0.9682533128939721, "recall": 0.975053112727741, "support": 31066}, "\u2205": {"f1-score": 0.986279890239122, "precision": 0.9782270606531882, "recall": 0.9944664031620554, "support": 17710}, "\u23ce": {"f1-score": 0.9480651731160896, "precision": 0.98, "recall": 0.9181459566074951, "support": 1014}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 25}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 24}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9714913375605374, "precision": 0.9796596515960125, "recall": 0.9634581105169341, "support": 10098}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 800}, "\u0027": {"f1-score": 0.7692893884549438, "precision": 0.9257221458046767, "recall": 0.6580834419817471, "support": 3068}, "macro avg": {"f1-score": 0.29643532060826094, "precision": 0.38636088580538774, "recall": 0.24956509437571306, "support": 47245}, "micro avg": {"f1-score": 0.7736077945627051, "precision": 0.975053112727741, "recall": 0.6411472113451159, "support": 47245}, "weighted avg": {"f1-score": 0.7277445845708912, "precision": 0.8708609705410724, "recall": 0.6411472113451159, "support": 47245}, "\u2205": {"f1-score": 0.9330119460705111, "precision": 0.9782270606531882, "recall": 0.8917919894678211, "support": 19749}, "\u23ce": {"f1-score": 0.53551912568306, "precision": 0.98, "recall": 0.3684210526315789, "support": 2527}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1230}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1135}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1338}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 242}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 305}, "\u2423": {"f1-score": 0.7265327458740947, "precision": 0.9796596515960125, "recall": 0.5773544596759836, "support": 16851}},
  "ppcr": 0.6575510636046142
}
```
</details>
