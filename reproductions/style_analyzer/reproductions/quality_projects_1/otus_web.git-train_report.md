# Train report for javascript / file:///tmp/top-repos-quality-repos-0sko0fhr/otus_web.git HEAD b90ad69e1b5c1828fa2ace165710422d113d1d17

### Classification report

PPCR: 0.722

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.934| 0.988| 0.836| 0.960| 0.882| 10685| 12635| 0.846 |
| `␣` | 0.963| 0.892| 0.650| 0.926| 0.776| 5030| 6911| 0.728 |
| `'` | 0.983| 1.000| 0.783| 0.991| 0.872| 1378| 1760| 0.783 |
| `⏎` | 0.926| 0.710| 0.259| 0.804| 0.405| 404| 1108| 0.365 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 46| 182| 0.253 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 42| 362| 0.116 |
| `"` | 1.000| 0.368| 0.280| 0.538| 0.438| 38| 50| 0.760 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 27| 555| 0.049 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 27| 365| 0.074 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 12| 201| 0.060 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 187| 0.027 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 194| 0.021 |
| `macro avg` | 0.400| 0.330| 0.234| 0.352| 0.281| 17698| 24510| 0.722 |
| `micro avg` | 0.945| 0.945| 0.683| 0.945| 0.793| 17698| 24510| 0.722 |
| `weighted avg` | 0.937| 0.945| 0.683| 0.940| 0.755| 17698| 24510| 0.722 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎⇥⁺| ⏎⇥⁻| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1950 |10562 |123 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1881 |533 |4489 |0 |8 |0 |0 |0 |0 |0 |0 |0 |0 |
|382 |0 |0 |1378 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|704 |97 |20 |0 |287 |0 |0 |0 |0 |0 |0 |0 |0 |
|528 |12 |2 |0 |13 |0 |0 |0 |0 |0 |0 |0 |0 |
|338 |23 |4 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|320 |32 |8 |0 |2 |0 |0 |0 |0 |0 |0 |0 |0 |
|182 |5 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|136 |35 |11 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|189 |7 |5 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|190 |4 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|12 |0 |0 |24 |0 |0 |0 |0 |0 |0 |0 |0 |14 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| ecommerce/static_in_env/js/addons/datatables-select.js | 317 |
| coursera/coursera_react2/homework/course-detail/src/App.js | 72 |
| ecommerce/static_in_env/js/modules/jquery.easing.js | 59 |
| ecommerce/static_in_env/js/modules/waves.js | 49 |
| ecommerce/static_in_env/js/modules/wow.js | 29 |
| coursera/coursera_react1/frontend/src/App.js | 28 |
| coursera/coursera_react2/homework/course-detail/src/serviceWorker.js | 28 |
| coursera/coursera_react1/frontend/src/serviceWorker.js | 28 |
| coursera/coursera_react2/homework/courses-list/src/serviceWorker.js | 28 |
| coursera/coursera_react2/homework/courses-list/src/App.js | 28 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.5384615384615384, "precision": 1.0, "recall": 0.3684210526315789, "support": 38}, "\u0027": {"f1-score": 0.9913669064748201, "precision": 0.9828815977175464, "recall": 1.0, "support": 1378}, "macro avg": {"f1-score": 0.35170675826069303, "precision": 0.4004536124611688, "recall": 0.3299792462997708, "support": 17698}, "micro avg": {"f1-score": 0.9453045541869137, "precision": 0.9453045541869137, "recall": 0.9453045541869137, "support": 17698}, "weighted avg": {"f1-score": 0.9398050245172661, "precision": 0.93728759227768, "recall": 0.9453045541869137, "support": 17698}, "\u2205": {"f1-score": 0.9604000909297568, "precision": 0.9338638373121132, "recall": 0.9884885353299018, "support": 10685}, "\u23ce": {"f1-score": 0.8039215686274509, "precision": 0.9258064516129032, "recall": 0.7103960396039604, "support": 404}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 27}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 27}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 46}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 42}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u2423": {"f1-score": 0.9263309946347502, "precision": 0.9628914628914629, "recall": 0.8924453280318091, "support": 5030}},
  "cl_report_full": {"\"": {"f1-score": 0.43750000000000006, "precision": 1.0, "recall": 0.28, "support": 50}, "\u0027": {"f1-score": 0.8716002530044277, "precision": 0.9828815977175464, "recall": 0.7829545454545455, "support": 1760}, "macro avg": {"f1-score": 0.2809879399558362, "precision": 0.4004536124611688, "recall": 0.23395466301702764, "support": 24510}, "micro avg": {"f1-score": 0.7927407126611068, "precision": 0.9453045541869137, "recall": 0.682578539371685, "support": 24510}, "weighted avg": {"f1-score": 0.7552920983315256, "precision": 0.8673838288393076, "recall": 0.682578539371685, "support": 24510}, "\u2205": {"f1-score": 0.8821883482981833, "precision": 0.9338638373121132, "recall": 0.8359319351009101, "support": 12635}, "\u23ce": {"f1-score": 0.40479548660084624, "precision": 0.9258064516129032, "recall": 0.25902527075812276, "support": 1108}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 201}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 194}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 555}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 365}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 182}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 362}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 187}, "\u2423": {"f1-score": 0.7757711915665773, "precision": 0.9628914628914629, "recall": 0.6495442048907538, "support": 6911}},
  "ppcr": 0.7220726234190127
}
```
</details>
