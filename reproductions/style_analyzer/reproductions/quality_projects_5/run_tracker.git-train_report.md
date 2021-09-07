# Train report for javascript / file:///tmp/top-repos-quality-repos-0os27k1t/run_tracker.git HEAD dfe4baa73200ff5f2068520f0e7d04879c50ab15

### Classification report

PPCR: 0.789

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.954| 0.993| 0.891| 0.973| 0.922| 10177| 11345| 0.897 |
| `␣` | 0.953| 0.903| 0.643| 0.928| 0.768| 3261| 4578| 0.712 |
| `"` | 1.000| 0.991| 0.980| 0.995| 0.990| 1096| 1108| 0.989 |
| `'` | 0.989| 1.000| 0.915| 0.994| 0.951| 895| 978| 0.915 |
| `⏎` | 0.968| 0.738| 0.356| 0.838| 0.520| 409| 849| 0.482 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 47| 296| 0.159 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 45| 289| 0.156 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 24| 304| 0.079 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 22| 290| 0.076 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 206| 0.019 |
| `micro avg` | 0.960| 0.960| 0.757| 0.960| 0.847| 15980| 20243| 0.789 |
| `macro avg` | 0.486| 0.463| 0.379| 0.473| 0.415| 15980| 20243| 0.789 |
| `weighted avg` | 0.951| 0.960| 0.757| 0.955| 0.812| 15980| 20243| 0.789 |

### Confusion matrix

|refusal|  ∅| ␣| '| "| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1168 |10106 |71 |0 |0 |0 |0 |0 |0 |0 |0 |
|1317 |306 |2945 |0 |0 |10 |0 |0 |0 |0 |0 |
|83 |0 |0 |895 |0 |0 |0 |0 |0 |0 |0 |
|12 |0 |0 |10 |1086 |0 |0 |0 |0 |0 |0 |
|440 |59 |48 |0 |0 |302 |0 |0 |0 |0 |0 |
|280 |15 |9 |0 |0 |0 |0 |0 |0 |0 |0 |
|244 |37 |8 |0 |0 |0 |0 |0 |0 |0 |0 |
|249 |47 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|268 |16 |6 |0 |0 |0 |0 |0 |0 |0 |0 |
|202 |2 |2 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/components/run_map/run_map.js | 88 |
| src/components/total_stats/total_stats.js | 34 |
| src/components/total_stats/modal/modal.js | 26 |
| src/components/api_test/index.js | 24 |
| src/components/account/sign_up/signup_form.js | 22 |
| src/components/total_stats/table.js | 21 |
| src/components/run_results/run_results.js | 19 |
| babel.config.js | 19 |
| src/components/run_results/results_chart.js | 18 |
| src/components/about_us/aboutus.js | 18 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.995417048579285, "precision": 1.0, "recall": 0.9908759124087592, "support": 1096}, "\u0027": {"f1-score": 0.9944444444444445, "precision": 0.988950276243094, "recall": 1.0, "support": 895}, "macro avg": {"f1-score": 0.47285145787258165, "precision": 0.4864758732177716, "recall": 0.46253829142495795, "support": 15980}, "micro avg": {"f1-score": 0.9595744680851064, "precision": 0.9595744680851064, "recall": 0.9595744680851064, "support": 15980}, "weighted avg": {"f1-score": 0.9545918251160925, "precision": 0.951169802449719, "recall": 0.9595744680851064, "support": 15980}, "\u2205": {"f1-score": 0.9733686491692752, "precision": 0.9544767661503589, "recall": 0.993023484327405, "support": 10177}, "\u23ce": {"f1-score": 0.8377253814147019, "precision": 0.967948717948718, "recall": 0.7383863080684596, "support": 409}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 24}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 47}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 45}, "\u2423": {"f1-score": 0.9275590551181102, "precision": 0.9533829718355454, "recall": 0.9030972094449555, "support": 3261}},
  "cl_report_full": {"\"": {"f1-score": 0.9899726526891522, "precision": 1.0, "recall": 0.98014440433213, "support": 1108}, "\u0027": {"f1-score": 0.9506107275624005, "precision": 0.988950276243094, "recall": 0.9151329243353783, "support": 978}, "macro avg": {"f1-score": 0.415058578193773, "precision": 0.4864758732177716, "recall": 0.3785072840369391, "support": 20243}, "micro avg": {"f1-score": 0.846644397206195, "precision": 0.9595744680851064, "recall": 0.7574964185150422, "support": 20243}, "weighted avg": {"f1-score": 0.8121334596820212, "precision": 0.8936475813240703, "recall": 0.7574964185150422, "support": 20243}, "\u2205": {"f1-score": 0.9215337619112753, "precision": 0.9544767661503589, "recall": 0.8907888937858087, "support": 11345}, "\u23ce": {"f1-score": 0.5202411714039621, "precision": 0.967948717948718, "recall": 0.35571260306242636, "support": 849}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 206}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 290}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 304}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 296}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 289}, "\u2423": {"f1-score": 0.7682274683709402, "precision": 0.9533829718355454, "recall": 0.6432940148536479, "support": 4578}},
  "ppcr": 0.7894086844835252
}
```
</details>
