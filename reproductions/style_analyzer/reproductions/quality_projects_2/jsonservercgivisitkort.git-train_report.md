# Train report for javascript / file:///tmp/top-repos-quality-repos-sq94abof/jsonservercgivisitkort.git HEAD 4e24abd73073984c8d84968f5aa51f6d443f1be5

### Classification report

PPCR: 0.617

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.985| 0.998| 0.911| 0.991| 0.946| 7450| 8163| 0.913 |
| `'` | 1.000| 1.000| 0.500| 1.000| 0.667| 817| 1634| 0.500 |
| `␣` | 0.997| 0.946| 0.206| 0.971| 0.341| 771| 3544| 0.218 |
| `⏎␣⁺␣⁺` | 0.962| 0.889| 0.570| 0.924| 0.716| 397| 619| 0.641 |
| `⏎⏎` | 0.955| 1.000| 0.542| 0.977| 0.691| 168| 310| 0.542 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 26| 387| 0.067 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 950| 0.006 |
| `weighted avg` | 0.983| 0.986| 0.609| 0.984| 0.684| 9635| 15607| 0.617 |
| `micro avg` | 0.986| 0.986| 0.609| 0.986| 0.753| 9635| 15607| 0.617 |
| `macro avg` | 0.700| 0.690| 0.390| 0.695| 0.480| 9635| 15607| 0.617 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|713 |7433 |2 |0 |0 |13 |0 |2 |
|2773 |41 |729 |0 |0 |1 |0 |0 |
|817 |0 |0 |817 |0 |0 |0 |0 |
|944 |1 |0 |0 |0 |0 |0 |5 |
|222 |44 |0 |0 |0 |353 |0 |0 |
|361 |25 |0 |0 |0 |0 |0 |1 |
|142 |0 |0 |0 |0 |0 |0 |168 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/front/index.js | 29 |
| src/server/router/plural.js | 26 |
| __tests__/server/plural.js | 20 |
| __tests__/cli/index.js | 19 |
| src/cli/run.js | 18 |
| src/server/utils.js | 7 |
| src/server/mixins.js | 6 |
| src/server/defaults.js | 3 |
| src/server/router/get-full-url.js | 2 |
| src/cli/index.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 817}, "macro avg": {"f1-score": 0.6947138476209345, "precision": 0.6998498082461727, "recall": 0.6903445969110338, "support": 9635}, "micro avg": {"f1-score": 0.9859885832900882, "precision": 0.9859885832900882, "recall": 0.9859885832900882, "support": 9635}, "weighted avg": {"f1-score": 0.9842002720780308, "precision": 0.9827185126067761, "recall": 0.9859885832900882, "support": 9635}, "\u2205": {"f1-score": 0.9914632519674537, "precision": 0.9852863202545069, "recall": 0.9977181208053691, "support": 7450}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u23ce": {"f1-score": 0.9767441860465117, "precision": 0.9545454545454546, "recall": 1.0, "support": 168}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9240837696335078, "precision": 0.9618528610354223, "recall": 0.889168765743073, "support": 397}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 26}, "\u2423": {"f1-score": 0.9707057256990679, "precision": 0.9972640218878249, "recall": 0.9455252918287937, "support": 771}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 1634}, "macro avg": {"f1-score": 0.4802226653435487, "precision": 0.6998498082461727, "recall": 0.38978314117726803, "support": 15607}, "micro avg": {"f1-score": 0.7527137310831155, "precision": 0.9859885832900882, "recall": 0.6087012238098289, "support": 15607}, "weighted avg": {"f1-score": 0.6844039439860198, "precision": 0.9036004317100025, "recall": 0.6087012238098289, "support": 15607}, "\u2205": {"f1-score": 0.9464569936970778, "precision": 0.9852863202545069, "recall": 0.9105720935930418, "support": 8163}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 950}, "\u23ce\u23ce": {"f1-score": 0.691358024691358, "precision": 0.9545454545454546, "recall": 0.5419354838709678, "support": 310}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.716024340770791, "precision": 0.9618528610354223, "recall": 0.5702746365105008, "support": 619}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 387}, "\u2423": {"f1-score": 0.3410526315789474, "precision": 0.9972640218878249, "recall": 0.20569977426636568, "support": 3544}},
  "ppcr": 0.6173511885692318
}
```
</details>
