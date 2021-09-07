# Train report for javascript / file:///tmp/top-repos-quality-repos-zyp_vg58/node-multiplayer-snake.git HEAD c21d4a49f1b8dec6f0a2005dfa97ac50c465d81f

### Classification report

PPCR: 0.750

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.983| 1.000| 0.956| 0.992| 0.970| 13680| 14304| 0.956 |
| `␣` | 0.997| 0.922| 0.399| 0.958| 0.570| 2242| 5181| 0.433 |
| `'` | 1.000| 1.000| 0.914| 1.000| 0.955| 755| 826| 0.914 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.981| 0.985| 0.906| 0.983| 0.942| 688| 748| 0.920 |
| `⏎⏎` | 1.000| 1.000| 0.451| 1.000| 0.621| 214| 475| 0.451 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 59| 1252| 0.047 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 743| 0.005 |
| `macro avg` | 0.709| 0.701| 0.518| 0.705| 0.580| 17642| 23529| 0.750 |
| `weighted avg` | 0.983| 0.986| 0.739| 0.984| 0.791| 17642| 23529| 0.750 |
| `micro avg` | 0.986| 0.986| 0.739| 0.986| 0.845| 17642| 23529| 0.750 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|624 |13678 |2 |0 |0 |0 |0 |0 |
|2939 |168 |2068 |0 |0 |0 |6 |0 |
|1193 |53 |3 |0 |0 |0 |3 |0 |
|71 |0 |0 |0 |755 |0 |0 |0 |
|739 |0 |0 |0 |0 |0 |4 |0 |
|60 |9 |1 |0 |0 |0 |678 |0 |
|261 |0 |0 |0 |0 |0 |0 |214 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| app/services/board-occupancy-service.js | 54 |
| .eslintrc.js | 14 |
| app/services/player-service.js | 13 |
| app/models/direction.js | 13 |
| public/js/model/direction.js | 13 |
| public/js/view/canvas-view.js | 11 |
| app/configs/server-config.js | 10 |
| public/js/controller/game-controller.js | 9 |
| app/services/food-service.js | 8 |
| public/js/view/game-view.js | 8 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 755}, "macro avg": {"f1-score": 0.7047437873179957, "precision": 0.708822354392611, "recall": 0.7011013771453992, "support": 17642}, "micro avg": {"f1-score": 0.9858859539734725, "precision": 0.9858859539734725, "recall": 0.9858859539734725, "support": 17642}, "weighted avg": {"f1-score": 0.9839576268036971, "precision": 0.9825043030488397, "recall": 0.9858859539734725, "support": 17642}, "\u2205": {"f1-score": 0.9915905466144701, "precision": 0.9834627552487777, "recall": 0.9998538011695907, "support": 13680}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 59}, "\u23ce\u23ce": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 214}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9833212472806381, "precision": 0.9811866859623734, "recall": 0.9854651162790697, "support": 688}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u2423": {"f1-score": 0.9582947173308619, "precision": 0.9971070395371263, "recall": 0.9223907225691347, "support": 2242}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9550917141049968, "precision": 1.0, "recall": 0.914043583535109, "support": 826}, "macro avg": {"f1-score": 0.5797643994048667, "precision": 0.708822354392611, "recall": 0.5180533960887039, "support": 23529}, "micro avg": {"f1-score": 0.8449151101503486, "precision": 0.9858859539734725, "recall": 0.7392154362701348, "support": 23529}, "weighted avg": {"f1-score": 0.7910431274175186, "precision": 0.9039224133630932, "recall": 0.7392154362701348, "support": 23529}, "\u2205": {"f1-score": 0.9696583014320147, "precision": 0.9834627552487777, "recall": 0.9562360178970917, "support": 14304}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1252}, "\u23ce\u23ce": {"f1-score": 0.6211901306240929, "precision": 1.0, "recall": 0.45052631578947366, "support": 475}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9423210562890897, "precision": 0.9811866859623734, "recall": 0.9064171122994652, "support": 748}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 743}, "\u2423": {"f1-score": 0.5700895933838732, "precision": 0.9971070395371263, "recall": 0.3991507430997877, "support": 5181}},
  "ppcr": 0.7497981214671257
}
```
</details>
