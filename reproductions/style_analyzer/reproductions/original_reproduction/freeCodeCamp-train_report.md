# Train report for javascript / file:///tmp/top-repos-quality-repos-xhfwaqrx/freeCodeCamp HEAD cf65516cce60645a417e44c4fcea7418ca920572

### Classification report

PPCR: 0.961

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.946| 0.984| 0.978| 0.964| 0.962| 59946| 60315| 0.994 |
| `␣` | 0.930| 0.939| 0.901| 0.935| 0.915| 28629| 29838| 0.959 |
| `'` | 0.981| 1.000| 0.999| 0.991| 0.990| 11032| 11046| 0.999 |
| `⏎` | 0.882| 0.839| 0.699| 0.860| 0.780| 7995| 9596| 0.833 |
| `⏎␣⁺␣⁺` | 0.821| 0.682| 0.646| 0.745| 0.723| 4216| 4452| 0.947 |
| `⏎␣⁻␣⁻` | 0.906| 0.656| 0.571| 0.761| 0.701| 3514| 4036| 0.871 |
| `⏎⏎` | 0.837| 0.583| 0.374| 0.687| 0.517| 1460| 2278| 0.641 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 208| 208| 1.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 191| 227| 0.841 |
| `weighted avg` | 0.931| 0.936| 0.899| 0.932| 0.909| 117191| 121996| 0.961 |
| `macro avg` | 0.700| 0.631| 0.574| 0.660| 0.621| 117191| 121996| 0.961 |
| `micro avg` | 0.936| 0.936| 0.899| 0.936| 0.917| 117191| 121996| 0.961 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|369 |58991 |910 |0 |0 |12 |32 |1 |0 |0 |
|1209 |650 |26883 |0 |404 |610 |78 |4 |0 |0 |
|14 |0 |0 |11032 |0 |0 |0 |0 |0 |0 |
|1601 |765 |365 |0 |6705 |4 |1 |155 |0 |0 |
|236 |965 |376 |0 |1 |2874 |0 |0 |0 |0 |
|522 |809 |335 |0 |59 |1 |2304 |6 |0 |0 |
|818 |151 |29 |0 |429 |0 |0 |851 |0 |0 |
|0 |0 |0 |208 |0 |0 |0 |0 |0 |0 |
|36 |60 |1 |0 |3 |0 |127 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| api-server/common/models/user.js | 427 |
| api-server/public/js/calculator.js | 386 |
| api-server/server/utils/user-stats.test.js | 184 |
| api-server/server/boot/challenge.js | 175 |
| curriculum/test/test-challenges.js | 156 |
| client/src/components/settings/Portfolio.js | 126 |
| api-server/server/utils/map.js | 123 |
| api-server/server/boot/certificate.js | 111 |
| api-server/common/utils/ajax-stream.js | 104 |
| curriculum/unpackedChallenge.js | 102 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 208}, "\u0027": {"f1-score": 0.99066091954023, "precision": 0.9814946619217082, "recall": 1.0, "support": 11032}, "macro avg": {"f1-score": 0.6602636643649004, "precision": 0.7003795857082105, "recall": 0.6313288465667294, "support": 117191}, "micro avg": {"f1-score": 0.9355667244071645, "precision": 0.9355667244071644, "recall": 0.9355667244071644, "support": 117191}, "weighted avg": {"f1-score": 0.9317224333911329, "precision": 0.9306099914195365, "recall": 0.9355667244071644, "support": 117191}, "\u2205": {"f1-score": 0.9644016119407866, "precision": 0.945504960651376, "recall": 0.9840689954292197, "support": 59946}, "\u23ce": {"f1-score": 0.859835855347525, "precision": 0.8821207735824234, "recall": 0.8386491557223265, "support": 7995}, "\u23ce\u23ce": {"f1-score": 0.6871215179652805, "precision": 0.8367748279252704, "recall": 0.5828767123287671, "support": 1460}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7448490345989374, "precision": 0.8209083119108826, "recall": 0.6816888045540797, "support": 4216}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7608982826948482, "precision": 0.9063729346970889, "recall": 0.6556630620375641, "support": 3514}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 191}, "\u2423": {"f1-score": 0.9346057571964955, "precision": 0.9302398006851448, "recall": 0.9390128890286074, "support": 28629}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 208}, "\u0027": {"f1-score": 0.9900385892488557, "precision": 0.9814946619217082, "recall": 0.9987325728770595, "support": 11046}, "macro avg": {"f1-score": 0.6207221071091086, "precision": 0.7003795857082105, "recall": 0.5740514567401427, "support": 121996}, "micro avg": {"f1-score": 0.9167722326046148, "precision": 0.9355667244071644, "recall": 0.8987179907537952, "support": 121996}, "weighted avg": {"f1-score": 0.9094245830984771, "precision": 0.9288011564354052, "recall": 0.8987179907537952, "support": 121996}, "\u2205": {"f1-score": 0.9615014750704937, "precision": 0.945504960651376, "recall": 0.9780485782972727, "support": 60315}, "\u23ce": {"f1-score": 0.7797871721811944, "precision": 0.8821207735824234, "recall": 0.698728636932055, "support": 9596}, "\u23ce\u23ce": {"f1-score": 0.5165402124430956, "precision": 0.8367748279252704, "recall": 0.3735733099209833, "support": 2278}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7227461335345152, "precision": 0.8209083119108826, "recall": 0.6455525606469003, "support": 4452}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7005168744299178, "precision": 0.9063729346970889, "recall": 0.5708622398414271, "support": 4036}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 227}, "\u2423": {"f1-score": 0.9153685070739057, "precision": 0.9302398006851448, "recall": 0.9009652121455861, "support": 29838}},
  "ppcr": 0.9606134627364832
}
```
</details>
