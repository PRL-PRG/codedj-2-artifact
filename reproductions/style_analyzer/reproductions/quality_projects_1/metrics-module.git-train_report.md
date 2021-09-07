# Train report for javascript / file:///tmp/top-repos-quality-repos-110j7_xc/metrics-module.git HEAD 08be393709d9bca4221dc9d05653cfd1b5935a7b

### Classification report

PPCR: 0.587

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.956| 0.996| 0.819| 0.976| 0.882| 2674| 3252| 0.822 |
| `␣` | 0.968| 0.852| 0.272| 0.906| 0.424| 568| 1781| 0.319 |
| `'` | 1.000| 1.000| 0.762| 1.000| 0.865| 407| 534| 0.762 |
| `⏎␣⁺␣⁺` | 0.925| 0.838| 0.801| 0.879| 0.859| 265| 277| 0.957 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 11| 270| 0.041 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 465| 0.017 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 123| 0.000 |
| `macro avg` | 0.550| 0.527| 0.379| 0.537| 0.433| 3933| 6702| 0.587 |
| `micro avg` | 0.960| 0.960| 0.564| 0.960| 0.710| 3933| 6702| 0.587 |
| `weighted avg` | 0.956| 0.960| 0.564| 0.957| 0.645| 3933| 6702| 0.587 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|578 |2664 |10 |0 |0 |0 |0 |0 |
|1213 |66 |484 |0 |0 |18 |0 |0 |
|127 |0 |0 |407 |0 |0 |0 |0 |
|457 |7 |1 |0 |0 |0 |0 |0 |
|12 |38 |5 |0 |0 |222 |0 |0 |
|259 |11 |0 |0 |0 |0 |0 |0 |
|123 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| mongodb.js | 55 |
| test/unit/js/timeAsyncMethodTests.js | 22 |
| http.js | 17 |
| index.js | 14 |
| memory.js | 14 |
| prom_wrapper.js | 13 |
| timeAsyncMethod.js | 7 |
| open_sockets.js | 6 |
| event_loop.js | 5 |
| test/acceptance/metrics_tests.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 407}, "macro avg": {"f1-score": 0.5373427339735367, "precision": 0.5498870885037431, "recall": 0.5265869727616203, "support": 3933}, "micro avg": {"f1-score": 0.9603356216628528, "precision": 0.9603356216628528, "recall": 0.9603356216628528, "support": 3933}, "weighted avg": {"f1-score": 0.9570710461643772, "precision": 0.9557217194541229, "recall": 0.9603356216628528, "support": 3933}, "\u2205": {"f1-score": 0.9758241758241758, "precision": 0.9562096195262024, "recall": 0.9962602842183994, "support": 2674}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8792079207920792, "precision": 0.925, "recall": 0.8377358490566038, "support": 265}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u2423": {"f1-score": 0.9063670411985019, "precision": 0.968, "recall": 0.852112676056338, "support": 568}},
  "cl_report_full": {"\u0027": {"f1-score": 0.8650371944739639, "precision": 1.0, "recall": 0.7621722846441947, "support": 534}, "macro avg": {"f1-score": 0.4329463766674965, "precision": 0.5498870885037431, "recall": 0.3792231370697235, "support": 6702}, "micro avg": {"f1-score": 0.710296191819464, "precision": 0.9603356216628528, "recall": 0.563563115487914, "support": 6702}, "weighted avg": {"f1-score": 0.6453643530408796, "precision": 0.8391266312592077, "recall": 0.563563115487914, "support": 6702}, "\u2205": {"f1-score": 0.8824113945014905, "precision": 0.9562096195262024, "recall": 0.8191881918819188, "support": 3252}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 465}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 123}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8588007736943907, "precision": 0.925, "recall": 0.8014440433212996, "support": 277}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 270}, "\u2423": {"f1-score": 0.4243752740026304, "precision": 0.968, "recall": 0.2717574396406513, "support": 1781}},
  "ppcr": 0.5868397493285586
}
```
</details>
