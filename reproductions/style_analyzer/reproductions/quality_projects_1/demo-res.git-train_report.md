# Train report for javascript / file:///tmp/top-repos-quality-repos-5d2sjbql/demo-res.git HEAD 8f36e4277e498c1d59138f8aa80adb8f8e9d7db7

### Classification report

PPCR: 0.678

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.969| 0.985| 0.882| 0.977| 0.924| 19063| 21284| 0.896 |
| `␣` | 0.949| 0.918| 0.442| 0.933| 0.603| 6202| 12882| 0.481 |
| `'` | 0.980| 1.000| 0.826| 0.990| 0.896| 3090| 3743| 0.826 |
| `⏎` | 0.942| 0.970| 0.450| 0.956| 0.609| 1163| 2509| 0.464 |
| `⏎␣⁻␣⁻` | 0.998| 0.852| 0.618| 0.919| 0.763| 514| 709| 0.725 |
| `⏎␣⁺␣⁺` | 0.981| 0.935| 0.219| 0.958| 0.359| 169| 720| 0.235 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 62| 113| 0.549 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 32| 507| 0.063 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 17| 1255| 0.014 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 542| 0.007 |
| `⏎⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 203| 0.010 |
| `⏎⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 232| 0.004 |
| `macro avg` | 0.485| 0.472| 0.286| 0.478| 0.346| 30319| 44699| 0.678 |
| `micro avg` | 0.966| 0.966| 0.655| 0.966| 0.781| 30319| 44699| 0.678 |
| `weighted avg` | 0.962| 0.966| 0.655| 0.964| 0.741| 30319| 44699| 0.678 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎⇥⁺| ⏎⇥⁻| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎⇥⁻| ⏎⏎⇥⁺| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2221 |18777 |284 |0 |0 |0 |0 |0 |2 |0 |0 |0 |0 |
|6680 |453 |5692 |0 |56 |0 |0 |0 |0 |1 |0 |0 |0 |
|653 |0 |0 |3090 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1346 |22 |13 |0 |1128 |0 |0 |0 |0 |0 |0 |0 |0 |
|1238 |1 |2 |0 |14 |0 |0 |0 |0 |0 |0 |0 |0 |
|538 |3 |0 |0 |0 |0 |0 |0 |1 |0 |0 |0 |0 |
|475 |32 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|551 |8 |3 |0 |0 |0 |0 |0 |158 |0 |0 |0 |0 |
|195 |76 |0 |0 |0 |0 |0 |0 |0 |438 |0 |0 |0 |
|231 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|201 |0 |2 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|51 |0 |0 |62 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| docs/js/reveal.js | 311 |
| web/src/redux/middleware/api_middleware.js | 213 |
| web/src/redux/middleware/api_middleware.test.js | 78 |
| web/src/redux/actions/api_actions.js | 54 |
| web/src/redux/middleware/archive_retro_middleware.test.js | 45 |
| web/src/test_support/fetch_matchers.js | 40 |
| web/.eslintrc.js | 29 |
| web/src/redux/reducers/retro_reducer.js | 25 |
| web/public/config.js | 19 |
| deployment/heroku/config/config.js | 19 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 62}, "\u0027": {"f1-score": 0.9900672861262416, "precision": 0.9803299492385786, "recall": 1.0, "support": 3090}, "macro avg": {"f1-score": 0.4777244494036379, "precision": 0.48496023780728376, "recall": 0.4716435261721494, "support": 30319}, "micro avg": {"f1-score": 0.9658300075860021, "precision": 0.9658300075860021, "recall": 0.9658300075860021, "support": 30319}, "weighted avg": {"f1-score": 0.9637049369495982, "precision": 0.962005257277294, "recall": 0.9658300075860021, "support": 30319}, "\u2205": {"f1-score": 0.977052763034655, "precision": 0.9692355339906055, "recall": 0.9849971148297749, "support": 19063}, "\u23ce": {"f1-score": 0.9555273189326557, "precision": 0.9415692821368948, "recall": 0.9699054170249355, "support": 1163}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 32}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 17}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9575757575757575, "precision": 0.9813664596273292, "recall": 0.9349112426035503, "support": 169}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.919202518363064, "precision": 0.9977220956719818, "recall": 0.8521400778210116, "support": 514}, "\u2423": {"f1-score": 0.9332677488112805, "precision": 0.9492995330220146, "recall": 0.9177684617865205, "support": 6202}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 113}, "\u0027": {"f1-score": 0.896301667875272, "precision": 0.9803299492385786, "recall": 0.8255410098851189, "support": 3743}, "macro avg": {"f1-score": 0.3461115092466895, "precision": 0.48496023780728376, "recall": 0.28636727623556885, "support": 44699}, "micro avg": {"f1-score": 0.7806926337678957, "precision": 0.9658300075860021, "recall": 0.655115326964809, "support": 44699}, "weighted avg": {"f1-score": 0.7407068009460479, "precision": 0.9016717786861053, "recall": 0.655115326964809, "support": 44699}, "\u2205": {"f1-score": 0.9236785793344321, "precision": 0.9692355339906055, "recall": 0.8822119902274008, "support": 21284}, "\u23ce": {"f1-score": 0.608578365254923, "precision": 0.9415692821368948, "recall": 0.44958150657632523, "support": 2509}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 542}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 507}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1255}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 203}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 232}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.358683314415437, "precision": 0.9813664596273292, "recall": 0.21944444444444444, "support": 720}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7630662020905924, "precision": 0.9977220956719818, "recall": 0.6177715091678421, "support": 709}, "\u2423": {"f1-score": 0.6030299819896175, "precision": 0.9492995330220146, "recall": 0.4418568545256948, "support": 12882}},
  "ppcr": 0.6782925792523322
}
```
</details>
