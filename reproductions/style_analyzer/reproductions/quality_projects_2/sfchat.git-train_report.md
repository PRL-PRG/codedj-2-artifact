# Train report for javascript / file:///tmp/top-repos-quality-repos-9_z3hcyn/sfchat.git HEAD 4422b4c2e172c238248b2d99ef2d82912710dbf9

### Classification report

PPCR: 0.833

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.958| 0.995| 0.974| 0.976| 0.966| 10021| 10242| 0.978 |
| `'` | 0.987| 1.000| 0.975| 0.994| 0.981| 1398| 1434| 0.975 |
| `␣` | 0.928| 0.864| 0.437| 0.895| 0.594| 1181| 2334| 0.506 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 1.000| 0.560| 0.559| 0.718| 0.717| 464| 465| 0.998 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.963| 0.940| 0.830| 0.951| 0.891| 415| 470| 0.883 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 63| 885| 0.071 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 32| 457| 0.070 |
| `"` | 1.000| 0.357| 0.233| 0.526| 0.377| 28| 43| 0.651 |
| `weighted avg` | 0.953| 0.959| 0.799| 0.954| 0.824| 13602| 16330| 0.833 |
| `micro avg` | 0.959| 0.959| 0.799| 0.959| 0.872| 13602| 16330| 0.833 |
| `macro avg` | 0.730| 0.590| 0.501| 0.633| 0.566| 13602| 16330| 0.833 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎⏎| ⏎␣⁻␣⁻␣⁻␣⁻| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|221 |9973 |35 |0 |0 |13 |0 |0 |0 |
|1153 |159 |1020 |0 |0 |2 |0 |0 |0 |
|36 |0 |0 |1398 |0 |0 |0 |0 |0 |
|822 |24 |39 |0 |0 |0 |0 |0 |0 |
|55 |23 |2 |0 |0 |390 |0 |0 |0 |
|425 |29 |3 |0 |0 |0 |0 |0 |0 |
|1 |204 |0 |0 |0 |0 |0 |260 |0 |
|15 |0 |0 |18 |0 |0 |0 |0 |10 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| bin/jasmine/tests/specs/sfchat/events/MessagesSpec.js | 104 |
| sfchat/static/js/app/sfchat/events/messages.js | 42 |
| bin/jasmine/tests/karma.conf.js | 38 |
| bin/jasmine/tests/specs/sfchat/api/v1/ClientSpec.js | 34 |
| sfchat/static/js/app/sfchat/errorHandler.js | 31 |
| bin/jasmine/tests/specs/sfchat/events/ChatSpec.js | 28 |
| sfchat/static/js/app/sfchat/api/v1/client.js | 21 |
| sfchat/static/js/app/sfchat/api/v1/auth.js | 20 |
| bin/jasmine/tests/requirejs-config.js | 19 |
| sfchat/static/js/app/sfchat/api/v1/renders/messages.js | 19 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.5263157894736842, "precision": 1.0, "recall": 0.35714285714285715, "support": 28}, "\u0027": {"f1-score": 0.9936034115138593, "precision": 0.9872881355932204, "recall": 1.0, "support": 1398}, "macro avg": {"f1-score": 0.6325342006821946, "precision": 0.7295255848874582, "recall": 0.5895164539463116, "support": 13602}, "micro avg": {"f1-score": 0.9594912512865755, "precision": 0.9594912512865755, "recall": 0.9594912512865755, "support": 13602}, "weighted avg": {"f1-score": 0.9535843209697056, "precision": 0.9532752303542378, "recall": 0.9594912512865755, "support": 13602}, "\u2205": {"f1-score": 0.9761660059707334, "precision": 0.9578371110257395, "recall": 0.9952100588763596, "support": 10021}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 63}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 32}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9512195121951219, "precision": 0.9629629629629629, "recall": 0.9397590361445783, "support": 415}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.718232044198895, "precision": 1.0, "recall": 0.5603448275862069, "support": 464}, "\u2423": {"f1-score": 0.894736842105263, "precision": 0.9281164695177434, "recall": 0.8636748518204911, "support": 1181}},
  "cl_report_full": {"\"": {"f1-score": 0.37735849056603776, "precision": 1.0, "recall": 0.23255813953488372, "support": 43}, "\u0027": {"f1-score": 0.9810526315789474, "precision": 0.9872881355932204, "recall": 0.9748953974895398, "support": 1434}, "macro avg": {"f1-score": 0.5658793060456222, "precision": 0.7295255848874582, "recall": 0.5008917686734674, "support": 16330}, "micro avg": {"f1-score": 0.8720432981424562, "precision": 0.9594912512865755, "recall": 0.7992039191671769, "support": 16330}, "weighted avg": {"f1-score": 0.8238454879218899, "precision": 0.8789194923461915, "recall": 0.7992039191671769, "support": 16330}, "\u2205": {"f1-score": 0.9657209257286724, "precision": 0.9578371110257395, "recall": 0.9737355985159148, "support": 10242}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 885}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 457}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.8914285714285713, "precision": 0.9629629629629629, "recall": 0.8297872340425532, "support": 470}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.7172413793103448, "precision": 1.0, "recall": 0.5591397849462365, "support": 465}, "\u2423": {"f1-score": 0.5942324497524032, "precision": 0.9281164695177434, "recall": 0.4370179948586118, "support": 2334}},
  "ppcr": 0.8329454990814452
}
```
</details>
