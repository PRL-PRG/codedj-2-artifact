# Train report for javascript / file:///tmp/top-repos-quality-repos-6vi0e0f1/learnvuejs.git HEAD 714a4570c202aa412e8a5dbe044aed6271cdb685

### Classification report

PPCR: 0.778

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.977| 0.988| 0.937| 0.982| 0.957| 7934| 8364| 0.949 |
| `␣` | 0.967| 0.899| 0.428| 0.932| 0.593| 2155| 4529| 0.476 |
| `'` | 0.950| 1.000| 0.900| 0.974| 0.924| 1870| 2078| 0.900 |
| `⏎` | 0.922| 0.975| 0.623| 0.948| 0.744| 1245| 1949| 0.639 |
| `⏎␣⁻␣⁻` | 0.913| 0.935| 0.869| 0.924| 0.891| 689| 741| 0.930 |
| `⏎␣⁺␣⁺` | 0.930| 0.952| 0.853| 0.941| 0.890| 683| 762| 0.896 |
| `"` | 1.000| 0.437| 0.380| 0.608| 0.551| 174| 200| 0.870 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 45| 401| 0.112 |
| `macro avg` | 0.832| 0.773| 0.624| 0.789| 0.694| 14795| 19024| 0.778 |
| `micro avg` | 0.962| 0.962| 0.748| 0.962| 0.842| 14795| 19024| 0.778 |
| `weighted avg` | 0.960| 0.962| 0.748| 0.959| 0.815| 14795| 19024| 0.778 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|430 |7840 |67 |0 |8 |4 |15 |0 |0 |
|2374 |87 |1938 |0 |39 |45 |46 |0 |0 |
|208 |0 |0 |1870 |0 |0 |0 |0 |0 |
|704 |31 |0 |0 |1214 |0 |0 |0 |0 |
|79 |30 |0 |0 |3 |650 |0 |0 |0 |
|52 |38 |0 |0 |7 |0 |644 |0 |0 |
|356 |0 |0 |0 |45 |0 |0 |0 |0 |
|26 |0 |0 |98 |0 |0 |0 |0 |76 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| Day6/03_webpack/webpack1.config.js | 36 |
| Day6/03_webpack/build/base.config.js | 31 |
| Day7/04_learnvuerouter/src/router/index.js | 26 |
| Day8/03_vuex/src/store/index.js | 23 |
| Day7/01_vueCli2Test/build/webpack.prod.conf.js | 19 |
| Day8/04_axios/build/webpack.prod.conf.js | 19 |
| Day8/03_vuex/build/webpack.prod.conf.js | 19 |
| Day7/02_vuecli2test/build/webpack.prod.conf.js | 19 |
| Day8/01_tabbar/build/webpack.prod.conf.js | 19 |
| Day7/04_learnvuerouter/build/webpack.prod.conf.js | 19 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.608, "precision": 1.0, "recall": 0.4367816091954023, "support": 174}, "\u0027": {"f1-score": 0.9744658676393955, "precision": 0.9502032520325203, "recall": 1.0, "support": 1870}, "macro avg": {"f1-score": 0.7886681835251346, "precision": 0.8324349432945406, "recall": 0.7732137391195028, "support": 14795}, "micro avg": {"f1-score": 0.9619466035822913, "precision": 0.9619466035822913, "recall": 0.9619466035822913, "support": 14795}, "weighted avg": {"f1-score": 0.959118391974262, "precision": 0.9595815290441814, "recall": 0.9619466035822913, "support": 14795}, "\u2205": {"f1-score": 0.9824561403508771, "precision": 0.9768253177174184, "recall": 0.9881522561129317, "support": 7934}, "\u23ce": {"f1-score": 0.9480671612651308, "precision": 0.9224924012158054, "recall": 0.9751004016064257, "support": 1245}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 45}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9406657018813314, "precision": 0.9298998569384835, "recall": 0.9516837481698389, "support": 683}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9239598278335724, "precision": 0.9134751773049645, "recall": 0.9346879535558781, "support": 689}, "\u2423": {"f1-score": 0.9317307692307693, "precision": 0.9665835411471322, "recall": 0.8993039443155453, "support": 2155}},
  "cl_report_full": {"\"": {"f1-score": 0.5507246376811594, "precision": 1.0, "recall": 0.38, "support": 200}, "\u0027": {"f1-score": 0.9243697478991597, "precision": 0.9502032520325203, "recall": 0.8999037536092397, "support": 2078}, "macro avg": {"f1-score": 0.6936449178513984, "precision": 0.8324349432945406, "recall": 0.623770131682434, "support": 19024}, "micro avg": {"f1-score": 0.8416570566841126, "precision": 0.9619466035822913, "recall": 0.748107653490328, "support": 19024}, "weighted avg": {"f1-score": 0.8151132389424933, "precision": 0.9412191263828393, "recall": 0.748107653490328, "support": 19024}, "\u2205": {"f1-score": 0.9566809029896277, "precision": 0.9768253177174184, "recall": 0.937350549976088, "support": 8364}, "\u23ce": {"f1-score": 0.7436447166921899, "precision": 0.9224924012158054, "recall": 0.6228835300153925, "support": 1949}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 401}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8898015058179328, "precision": 0.9298998569384835, "recall": 0.8530183727034121, "support": 762}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8907330567081604, "precision": 0.9134751773049645, "recall": 0.8690958164642375, "support": 741}, "\u2423": {"f1-score": 0.5932047750229569, "precision": 0.9665835411471322, "recall": 0.4279090306911018, "support": 4529}},
  "ppcr": 0.777701850294365
}
```
</details>
