# Train report for javascript / file:///tmp/top-repos-quality-repos-p4_292mt/mailor.git HEAD 069eece82d49974ed848e26daca641b2fec24868

### Classification report

PPCR: 0.523

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.988| 1.000| 0.706| 0.994| 0.824| 4239| 6001| 0.706 |
| `'` | 1.000| 1.000| 0.835| 1.000| 0.910| 633| 758| 0.835 |
| `␣` | 1.000| 0.934| 0.160| 0.966| 0.277| 441| 2568| 0.172 |
| `⏎␣⁺␣⁺` | 0.988| 0.969| 0.697| 0.978| 0.817| 254| 353| 0.720 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 12| 334| 0.036 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 350| 0.017 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 309| 0.000 |
| `weighted avg` | 0.987| 0.990| 0.518| 0.988| 0.621| 5585| 10673| 0.523 |
| `micro avg` | 0.990| 0.990| 0.518| 0.990| 0.680| 5585| 10673| 0.523 |
| `macro avg` | 0.568| 0.558| 0.343| 0.563| 0.404| 5585| 10673| 0.523 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1762 |4239 |0 |0 |0 |0 |0 |0 |
|2127 |26 |412 |0 |0 |3 |0 |0 |
|125 |0 |0 |633 |0 |0 |0 |0 |
|344 |6 |0 |0 |0 |0 |0 |0 |
|99 |8 |0 |0 |0 |246 |0 |0 |
|322 |12 |0 |0 |0 |0 |0 |0 |
|309 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| bin/watch.js | 14 |
| lib/maildev.js | 10 |
| lib/compiler.js | 8 |
| public/app.js | 8 |
| bin/send.js | 3 |
| lib/worker.js | 3 |
| tests/main.test.js | 3 |
| bin/util.js | 3 |
| bin/cli.js | 2 |
| lib/mailer.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 633}, "macro avg": {"f1-score": 0.5625767751554648, "precision": 0.5679762028438019, "recall": 0.5575348999742379, "support": 5585}, "micro avg": {"f1-score": 0.990152193375112, "precision": 0.990152193375112, "recall": 0.990152193375112, "support": 5585}, "weighted avg": {"f1-score": 0.9884710584500086, "precision": 0.9870313187425076, "recall": 0.990152193375112, "support": 5585}, "\u2205": {"f1-score": 0.9939038686987104, "precision": 0.9878816126776975, "recall": 1.0, "support": 4239}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.978131212723658, "precision": 0.9879518072289156, "recall": 0.968503937007874, "support": 254}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u2423": {"f1-score": 0.9660023446658852, "precision": 1.0, "recall": 0.9342403628117913, "support": 441}},
  "cl_report_full": {"\u0027": {"f1-score": 0.910136592379583, "precision": 1.0, "recall": 0.8350923482849604, "support": 758}, "macro avg": {"f1-score": 0.40395271518605863, "precision": 0.5679762028438019, "recall": 0.3426849439527942, "support": 10673}, "micro avg": {"f1-score": 0.6802804773034813, "precision": 0.990152193375112, "recall": 0.5181298603953902, "support": 10673}, "weighted avg": {"f1-score": 0.6213588560521891, "precision": 0.8997493249911618, "recall": 0.5181298603953902, "support": 10673}, "\u2205": {"f1-score": 0.8237465993004275, "precision": 0.9878816126776975, "recall": 0.7063822696217297, "support": 6001}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 350}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 309}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8172757475083057, "precision": 0.9879518072289156, "recall": 0.6968838526912181, "support": 353}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 334}, "\u2423": {"f1-score": 0.2765100671140939, "precision": 1.0, "recall": 0.16043613707165108, "support": 2568}},
  "ppcr": 0.5232830506886537
}
```
</details>
