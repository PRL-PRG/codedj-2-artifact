# Train report for javascript / file:///tmp/top-repos-quality-repos-xw0hilfj/codeforlearning.git HEAD 3a54642474b1ea0613ba74afacf8ef2754c10f66

### Classification report

PPCR: 0.732

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.961| 0.994| 0.962| 0.977| 0.961| 84666| 87500| 0.968 |
| `␣` | 0.978| 0.947| 0.685| 0.962| 0.806| 33834| 46786| 0.723 |
| `'` | 0.987| 1.000| 0.760| 0.993| 0.859| 11160| 14675| 0.760 |
| `⏎` | 0.933| 0.785| 0.209| 0.853| 0.342| 3979| 14928| 0.267 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 515| 7295| 0.071 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 505| 7621| 0.066 |
| `"` | 0.992| 0.620| 0.230| 0.763| 0.374| 400| 1076| 0.372 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 202| 4062| 0.050 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 21| 396| 0.053 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 12| 229| 0.052 |
| `⏎⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 94| 0.021 |
| `⏎⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 95| 0.000 |
| `macro avg` | 0.404| 0.362| 0.237| 0.379| 0.278| 135296| 184757| 0.732 |
| `weighted avg` | 0.957| 0.966| 0.708| 0.961| 0.757| 135296| 184757| 0.732 |
| `micro avg` | 0.966| 0.966| 0.708| 0.966| 0.817| 135296| 184757| 0.732 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎⏎␣⁻␣⁻| ⏎⏎␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2834 |84149 |517 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|12952 |1746 |32056 |32 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|10949 |731 |123 |3125 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3515 |0 |0 |0 |11158 |0 |0 |0 |2 |0 |0 |0 |0 |
|7116 |424 |75 |6 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|6780 |513 |0 |2 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3860 |12 |6 |184 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|676 |0 |0 |0 |152 |0 |0 |0 |248 |0 |0 |0 |0 |
|375 |21 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|217 |6 |6 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|92 |1 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|95 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| VUE/vue-blog/.nuxt/utils.js | 81 |
| VUE/vue-blog2/.nuxt/utils.js | 81 |
| sourcecodes/redux/examples/todomvc/src/reducers/todos.spec.js | 59 |
| sourcecodes/redux/src/combineReducers.js | 55 |
| sourcecodes/redux/test/createStore.spec.js | 47 |
| wxapp/jza/js/main.js | 44 |
| wxapp/lottery/pages/index/index.js | 43 |
| VUE/mp-vue/todolist/src/utils/map.js | 42 |
| VUE/vue-blog2/.nuxt/server.js | 40 |
| sourcecodes/redux/test/compose.spec.js | 36 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.7630769230769231, "precision": 0.992, "recall": 0.62, "support": 400}, "\u0027": {"f1-score": 0.9931464174454828, "precision": 0.9865605658709107, "recall": 0.999820788530466, "support": 11160}, "macro avg": {"f1-score": 0.37903731120947537, "precision": 0.40417009173071355, "recall": 0.3622114134940912, "support": 135296}, "micro avg": {"f1-score": 0.9662961210974456, "precision": 0.9662961210974456, "recall": 0.9662961210974456, "support": 135296}, "weighted avg": {"f1-score": 0.9612849422467251, "precision": 0.9573835248348578, "recall": 0.9662961210974456, "support": 135296}, "\u2205": {"f1-score": 0.976948841637207, "precision": 0.9605721265253473, "recall": 0.9938936527059268, "support": 84666}, "\u23ce": {"f1-score": 0.8528930131004366, "precision": 0.9331143624962676, "recall": 0.7853732093490827, "support": 3979}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 202}, "\u23ce\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 505}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 515}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 21}, "\u2423": {"f1-score": 0.9623825392536551, "precision": 0.9777940458760371, "recall": 0.9474493113436189, "support": 33834}},
  "cl_report_full": {"\"": {"f1-score": 0.3740573152337858, "precision": 0.992, "recall": 0.23048327137546468, "support": 1076}, "\u0027": {"f1-score": 0.8588031556667308, "precision": 0.9865605658709107, "recall": 0.7603407155025553, "support": 14675}, "macro avg": {"f1-score": 0.2784740227896111, "precision": 0.40417009173071355, "recall": 0.23725226904352203, "support": 184757}, "micro avg": {"f1-score": 0.8169646902231816, "precision": 0.9662961210974456, "recall": 0.7076105370838452, "support": 184757}, "weighted avg": {"f1-score": 0.7572466605552979, "precision": 0.8620611549701719, "recall": 0.7076105370838452, "support": 184757}, "\u2205": {"f1-score": 0.9611371592719713, "precision": 0.9605721265253473, "recall": 0.9617028571428572, "support": 87500}, "\u23ce": {"f1-score": 0.3419598402363626, "precision": 0.9331143624962676, "recall": 0.20933815648445875, "support": 14928}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4062}, "\u23ce\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 95}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 94}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7621}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 229}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7295}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 396}, "\u2423": {"f1-score": 0.8057308030664824, "precision": 0.9777940458760371, "recall": 0.6851622280169282, "support": 46786}},
  "ppcr": 0.7322916046482677
}
```
</details>
