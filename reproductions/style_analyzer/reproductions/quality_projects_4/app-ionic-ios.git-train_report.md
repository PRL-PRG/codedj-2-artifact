# Train report for javascript / file:///tmp/top-repos-quality-repos-sdjea3s4/app-ionic-ios.git HEAD e702dd34b3c1ddf728ce67ed7662679fed3dacc6

### Classification report

PPCR: 0.721

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.957| 0.991| 0.929| 0.974| 0.943| 20780| 22152| 0.938 |
| `␣` | 0.953| 0.947| 0.751| 0.950| 0.840| 8531| 10765| 0.792 |
| `'` | 1.000| 1.000| 0.230| 1.000| 0.374| 683| 2973| 0.230 |
| `⏎` | 0.930| 0.461| 0.140| 0.617| 0.244| 605| 1986| 0.305 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 92| 729| 0.126 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 91| 817| 0.111 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 89| 663| 0.134 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 75| 744| 0.101 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 22| 663| 0.033 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1483| 0.000 |
| `⏎⇥⁺⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⇥⁻⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `micro avg` | 0.957| 0.957| 0.689| 0.957| 0.801| 30968| 42975| 0.721 |
| `weighted avg` | 0.945| 0.957| 0.689| 0.949| 0.734| 30968| 42975| 0.721 |
| `macro avg` | 0.320| 0.283| 0.171| 0.295| 0.200| 30968| 42975| 0.721 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⇥⁺⇥⁺| ⏎⇥⁻⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1372 |20583 |197 |0 |0 |0 |0 |0 |0 |0 |0 |
|2234 |451 |8080 |0 |0 |0 |0 |0 |0 |0 |0 |
|2290 |0 |0 |683 |0 |0 |0 |0 |0 |0 |0 |
|1381 |164 |162 |0 |279 |0 |0 |0 |0 |0 |0 |
|1483 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|726 |82 |7 |0 |2 |0 |0 |0 |0 |0 |0 |
|637 |88 |4 |0 |0 |0 |0 |0 |0 |0 |0 |
|641 |2 |1 |0 |19 |0 |0 |0 |0 |0 |0 |
|669 |44 |31 |0 |0 |0 |0 |0 |0 |0 |0 |
|574 |89 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| www/js/controllers.js | 160 |
| www/lib/ng-cordova-oauth/src/oauth.js | 126 |
| www/js/services.js | 117 |
| src/js/controllers/authController.js | 71 |
| www/js/app.js | 68 |
| www/js/directives.js | 47 |
| www/lib/ng-cordova-oauth/src/utility.js | 30 |
| www/lib/ng-cordova-oauth/src/oauth.twitter.js | 26 |
| www/lib/ng-cordova-oauth/src/oauth.xing.js | 26 |
| gulpfile.js | 26 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 683}, "macro avg": {"f1-score": 0.2950016016440245, "precision": 0.3199850658211833, "recall": 0.28323422810430693, "support": 30968}, "micro avg": {"f1-score": 0.9566326530612245, "precision": 0.9566326530612245, "recall": 0.9566326530612245, "support": 30968}, "weighted avg": {"f1-score": 0.9490554466234143, "precision": 0.9449515961152428, "recall": 0.9566326530612245, "support": 30968}, "\u2205": {"f1-score": 0.9735827637584845, "precision": 0.9572152722875877, "recall": 0.9905197305101059, "support": 20780}, "\u23ce": {"f1-score": 0.6165745856353592, "precision": 0.93, "recall": 0.46115702479338844, "support": 605}, "\u23ce\u21e5\u207a\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 91}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 75}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 92}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 89}, "\u2423": {"f1-score": 0.9498618703344501, "precision": 0.9526055175666116, "recall": 0.947133981948189, "support": 8531}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1483}, "\u0027": {"f1-score": 0.37363238512035013, "precision": 1.0, "recall": 0.22973427514295325, "support": 2973}, "macro avg": {"f1-score": 0.20002691735309916, "precision": 0.3199850658211833, "recall": 0.1708307854158675, "support": 42975}, "micro avg": {"f1-score": 0.8012928877648999, "precision": 0.9566326530612245, "recall": 0.6893542757417103, "support": 42975}, "weighted avg": {"f1-score": 0.7335193872705204, "precision": 0.8441887401586787, "recall": 0.6893542757417103, "support": 42975}, "\u2205": {"f1-score": 0.9429847669224601, "precision": 0.9572152722875877, "recall": 0.9291711809317443, "support": 22152}, "\u23ce": {"f1-score": 0.24409448818897633, "precision": 0.93, "recall": 0.1404833836858006, "support": 1986}, "\u23ce\u21e5\u207a\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 663}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 817}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 744}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 729}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 663}, "\u2423": {"f1-score": 0.8396113680054035, "precision": 0.9526055175666116, "recall": 0.7505805852299118, "support": 10765}},
  "ppcr": 0.7206050029086678
}
```
</details>
