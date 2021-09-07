# Train report for javascript / file:///tmp/top-repos-quality-repos-d6ote4gs/governance-portal.git HEAD b7b1fa0a0940c1ef2638db91e802a34a04409cbe

### Classification report

PPCR: 0.782

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.968| 0.996| 0.935| 0.982| 0.951| 42782| 45576| 0.939 |
| `␣` | 0.970| 0.961| 0.657| 0.965| 0.784| 13618| 19901| 0.684 |
| `'` | 1.000| 1.000| 0.828| 1.000| 0.906| 3941| 4757| 0.828 |
| `⏎` | 0.961| 0.911| 0.328| 0.935| 0.490| 1715| 4756| 0.361 |
| `"` | 1.000| 1.000| 0.997| 1.000| 0.998| 1312| 1316| 0.997 |
| `⏎␣⁻␣⁻` | 0.968| 0.624| 0.274| 0.759| 0.427| 1121| 2552| 0.439 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 450| 2637| 0.171 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 101| 1521| 0.066 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 38| 128| 0.297 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 26| 151| 0.172 |
| `weighted avg` | 0.962| 0.971| 0.759| 0.965| 0.816| 65104| 83295| 0.782 |
| `macro avg` | 0.587| 0.549| 0.402| 0.564| 0.456| 65104| 83295| 0.782 |
| `micro avg` | 0.971| 0.971| 0.759| 0.971| 0.852| 65104| 83295| 0.782 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2794 |42599 |183 |0 |0 |0 |0 |0 |0 |0 |0 |
|6283 |487 |13083 |0 |48 |0 |0 |0 |0 |0 |0 |
|816 |0 |0 |3941 |0 |0 |0 |0 |0 |0 |0 |
|3041 |122 |31 |0 |1562 |0 |0 |0 |0 |0 |0 |
|2187 |289 |161 |0 |0 |0 |0 |0 |0 |0 |0 |
|1431 |400 |9 |0 |13 |0 |699 |0 |0 |0 |0 |
|1420 |101 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|4 |0 |0 |0 |0 |0 |0 |0 |1312 |0 |0 |
|125 |7 |19 |0 |0 |0 |0 |0 |0 |0 |0 |
|90 |13 |0 |0 |2 |0 |23 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| test/reducers/accounts.test.js | 131 |
| src/reducers/accounts.js | 114 |
| src/reducers/polling.js | 83 |
| src/pages/Polling.js | 75 |
| src/utils/misc.js | 65 |
| src/reducers/proxy.js | 51 |
| src/components/Modules/MKRBurn.js | 51 |
| src/components/Onboarding/LockMKR.js | 47 |
| src/pages/CreatePoll.js | 42 |
| src/pages/Timeline.js | 41 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1312}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 3941}, "macro avg": {"f1-score": 0.564081153126166, "precision": 0.586725515987369, "recall": 0.5490773834754245, "support": 65104}, "micro avg": {"f1-score": 0.9706930449741953, "precision": 0.9706930449741952, "recall": 0.9706930449741952, "support": 65104}, "weighted avg": {"f1-score": 0.9653253497643443, "precision": 0.9615492198288442, "recall": 0.9706930449741952, "support": 65104}, "\u2205": {"f1-score": 0.9815437788018433, "precision": 0.9677631877868145, "recall": 0.9957225001168716, "support": 42782}, "\u23ce": {"f1-score": 0.9353293413173652, "precision": 0.9612307692307692, "recall": 0.9107871720116618, "support": 1715}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 101}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 450}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 26}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7585458491589798, "precision": 0.9681440443213296, "recall": 0.623550401427297, "support": 1121}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 38}, "\u2423": {"f1-score": 0.965392561983471, "precision": 0.9701171585347768, "recall": 0.9607137611984139, "support": 13618}},
  "cl_report_full": {"\"": {"f1-score": 0.9984779299847794, "precision": 1.0, "recall": 0.9969604863221885, "support": 1316}, "\u0027": {"f1-score": 0.9061853299609106, "precision": 1.0, "recall": 0.8284633172167333, "support": 4757}, "macro avg": {"f1-score": 0.4555894734821752, "precision": 0.586725515987369, "recall": 0.4019838558804657, "support": 83295}, "micro avg": {"f1-score": 0.851703852451836, "precision": 0.9706930449741952, "recall": 0.7587010024611321, "support": 83295}, "weighted avg": {"f1-score": 0.8161284001957272, "precision": 0.9187633562415753, "recall": 0.7587010024611321, "support": 83295}, "\u2205": {"f1-score": 0.9509342143447106, "precision": 0.9677631877868145, "recall": 0.9346805336141829, "support": 45576}, "\u23ce": {"f1-score": 0.489578435981821, "precision": 0.9612307692307692, "recall": 0.32842724978973925, "support": 4756}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1521}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2637}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 151}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.4270006108735492, "precision": 0.9681440443213296, "recall": 0.2739028213166144, "support": 2552}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 128}, "\u2423": {"f1-score": 0.7837182136759816, "precision": 0.9701171585347768, "recall": 0.6574041505451987, "support": 19901}},
  "ppcr": 0.7816075394681553
}
```
</details>
