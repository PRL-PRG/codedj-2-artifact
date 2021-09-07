# Train report for javascript / file:///tmp/top-repos-quality-repos-16_69c5c/1-4_presents.git HEAD 5407ba0197e9b4a77ab861496cbc2e20d166bac6

### Classification report

PPCR: 0.770

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.973| 0.978| 0.927| 0.976| 0.949| 18604| 19627| 0.948 |
| `␣` | 0.955| 0.961| 0.716| 0.958| 0.819| 12608| 16914| 0.745 |
| `'` | 1.000| 1.000| 0.934| 1.000| 0.966| 2905| 3109| 0.934 |
| `⏎` | 0.972| 0.898| 0.416| 0.933| 0.583| 1179| 2544| 0.463 |
| `⏎⏎` | 0.950| 0.885| 0.171| 0.916| 0.289| 260| 1347| 0.193 |
| `⏎⏎⇥⁺` | 0.739| 1.000| 0.473| 0.850| 0.577| 116| 245| 0.473 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 94| 1239| 0.076 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 16| 1147| 0.014 |
| `⏎⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 305| 0.000 |
| `micro avg` | 0.968| 0.968| 0.745| 0.968| 0.842| 35782| 46477| 0.770 |
| `macro avg` | 0.621| 0.636| 0.404| 0.626| 0.465| 35782| 46477| 0.770 |
| `weighted avg` | 0.965| 0.968| 0.745| 0.966| 0.807| 35782| 46477| 0.770 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎⇥⁺| ⏎⇥⁻| ⏎⏎⇥⁻| ⏎⏎⇥⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1023 |18196 |407 |0 |0 |1 |0 |0 |0 |0 |
|4306 |486 |12116 |0 |0 |0 |0 |0 |0 |6 |
|204 |0 |0 |2905 |0 |0 |0 |0 |0 |0 |
|1365 |6 |103 |0 |1059 |11 |0 |0 |0 |0 |
|1087 |1 |6 |0 |23 |230 |0 |0 |0 |0 |
|1145 |6 |52 |0 |1 |0 |0 |0 |0 |35 |
|1131 |6 |3 |0 |7 |0 |0 |0 |0 |0 |
|305 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|129 |0 |0 |0 |0 |0 |0 |0 |0 |116 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| test/qunit-1.12.0.js | 628 |
| js/reveal.js | 291 |
| plugin/markdown/markdown.js | 50 |
| plugin/multiplex/index.js | 44 |
| plugin/zoom-js/zoom.js | 41 |
| test/test.js | 38 |
| Gruntfile.js | 16 |
| plugin/notes-server/index.js | 12 |
| plugin/print-pdf/print-pdf.js | 10 |
| plugin/multiplex/client.js | 10 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 2905}, "macro avg": {"f1-score": 0.6259004804482887, "precision": 0.620979528449638, "recall": 0.6357645115461628, "support": 35782}, "micro avg": {"f1-score": 0.9675814655413336, "precision": 0.9675814655413336, "recall": 0.9675814655413336, "support": 35782}, "weighted avg": {"f1-score": 0.9661061781899617, "precision": 0.9648834095257449, "recall": 0.9675814655413336, "support": 35782}, "\u2205": {"f1-score": 0.9755260688915695, "precision": 0.9729960964654296, "recall": 0.9780692324231348, "support": 18604}, "\u23ce": {"f1-score": 0.9334508594094314, "precision": 0.9715596330275229, "recall": 0.8982188295165394, "support": 1179}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 94}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u23ce\u23ce": {"f1-score": 0.9163346613545816, "precision": 0.9504132231404959, "recall": 0.8846153846153846, "support": 260}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.8498168498168498, "precision": 0.7388535031847133, "recall": 1.0, "support": 116}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9579758845621664, "precision": 0.9549933002285804, "recall": 0.9609771573604061, "support": 12608}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9660791486531427, "precision": 1.0, "recall": 0.9343840463171438, "support": 3109}, "macro avg": {"f1-score": 0.4648469197483023, "precision": 0.620979528449638, "recall": 0.4042551926615187, "support": 46477}, "micro avg": {"f1-score": 0.8417802307346309, "precision": 0.9675814655413336, "recall": 0.7449275985971556, "support": 46477}, "weighted avg": {"f1-score": 0.8068369674296223, "precision": 0.9099473823970075, "recall": 0.7449275985971556, "support": 46477}, "\u2205": {"f1-score": 0.9494886245042787, "precision": 0.9729960964654296, "recall": 0.9270902328425129, "support": 19627}, "\u23ce": {"f1-score": 0.5828288387451843, "precision": 0.9715596330275229, "recall": 0.41627358490566035, "support": 2544}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1239}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1147}, "\u23ce\u23ce": {"f1-score": 0.28949024543738205, "precision": 0.9504132231404959, "recall": 0.17074981440237566, "support": 1347}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.5771144278606964, "precision": 0.7388535031847133, "recall": 0.47346938775510206, "support": 245}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 305}, "\u2423": {"f1-score": 0.8186209925340359, "precision": 0.9549933002285804, "recall": 0.7163296677308738, "support": 16914}},
  "ppcr": 0.7698861802612045
}
```
</details>
