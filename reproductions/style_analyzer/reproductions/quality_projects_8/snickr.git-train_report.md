# Train report for javascript / file:///tmp/top-repos-quality-repos-dzthos0r/snickr.git HEAD 9d4a1df888a47d69d8a6b991e075b4b7e8df646b

### Classification report

PPCR: 0.775

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.955| 0.991| 0.907| 0.973| 0.930| 16042| 17533| 0.915 |
| `␣` | 0.957| 0.914| 0.588| 0.935| 0.728| 5271| 8200| 0.643 |
| `'` | 0.998| 0.998| 0.901| 0.998| 0.947| 2506| 2776| 0.903 |
| `⏎` | 0.910| 0.953| 0.702| 0.931| 0.793| 1776| 2409| 0.737 |
| `⏎␣⁺␣⁺` | 1.000| 0.635| 0.210| 0.777| 0.346| 458| 1389| 0.330 |
| `"` | 0.991| 0.991| 0.884| 0.991| 0.934| 431| 483| 0.892 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 212| 1331| 0.159 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 72| 344| 0.209 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 94| 0.043 |
| `weighted avg` | 0.948| 0.957| 0.742| 0.952| 0.803| 26772| 34559| 0.775 |
| `micro avg` | 0.957| 0.957| 0.742| 0.957| 0.836| 26772| 34559| 0.775 |
| `macro avg` | 0.646| 0.609| 0.466| 0.623| 0.520| 26772| 34559| 0.775 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| ⏎⏎| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1491 |15902 |132 |0 |8 |0 |0 |0 |0 |0 |
|2929 |346 |4820 |0 |105 |0 |0 |0 |0 |0 |
|270 |0 |0 |2502 |0 |0 |0 |4 |0 |0 |
|633 |67 |17 |0 |1692 |0 |0 |0 |0 |0 |
|931 |126 |40 |0 |1 |291 |0 |0 |0 |0 |
|1119 |201 |10 |0 |1 |0 |0 |0 |0 |0 |
|52 |0 |0 |4 |0 |0 |0 |427 |0 |0 |
|272 |5 |15 |0 |52 |0 |0 |0 |0 |0 |
|90 |2 |2 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| client-react/src/components/pages/Workspace.js | 63 |
| client-react/src/components/dialogs/NotificationDialog.js | 52 |
| client-react/src/components/subcomponents/DetailDrawer.js | 51 |
| client-react/src/components/pages/Signup.js | 47 |
| client-react/src/components/subcomponents/WorkspaceManager.js | 42 |
| client-react/src/routes.js | 40 |
| client-react/src/components/dialogs/UserProfileDialog.js | 39 |
| client-react/src/components/subcomponents/DIYTopBar.js | 37 |
| client-react/src/components/dialogs/InvitationDialog.js | 35 |
| client-react/src/components/dialogs/ChannelDialog.js | 34 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9907192575406032, "precision": 0.9907192575406032, "recall": 0.9907192575406032, "support": 431}, "\u0027": {"f1-score": 0.9984038308060654, "precision": 0.9984038308060654, "recall": 0.9984038308060654, "support": 2506}, "macro avg": {"f1-score": 0.6228068989628807, "precision": 0.6457256779527921, "recall": 0.6092119296495807, "support": 26772}, "micro avg": {"f1-score": 0.9574929030330196, "precision": 0.9574929030330196, "recall": 0.9574929030330196, "support": 26772}, "weighted avg": {"f1-score": 0.9515493474210467, "precision": 0.9476546892198209, "recall": 0.9574929030330196, "support": 26772}, "\u2205": {"f1-score": 0.9728671499801169, "precision": 0.95513244038681, "recall": 0.9912729086148859, "support": 16042}, "\u23ce": {"f1-score": 0.930949105914718, "precision": 0.9101667563206025, "recall": 0.9527027027027027, "support": 1776}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 72}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7770360480640854, "precision": 1.0, "recall": 0.6353711790393013, "support": 458}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 212}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u2423": {"f1-score": 0.9352866983603375, "precision": 0.9571088165210484, "recall": 0.9144374881426675, "support": 5271}},
  "cl_report_full": {"\"": {"f1-score": 0.9343544857768052, "precision": 0.9907192575406032, "recall": 0.8840579710144928, "support": 483}, "\u0027": {"f1-score": 0.9473684210526316, "precision": 0.9984038308060654, "recall": 0.9012968299711815, "support": 2776}, "macro avg": {"f1-score": 0.5199751819147814, "precision": 0.6457256779527921, "recall": 0.46577827373139385, "support": 34559}, "micro avg": {"f1-score": 0.8359231057703282, "precision": 0.9574929030330196, "recall": 0.7417459995948957, "support": 34559}, "weighted avg": {"f1-score": 0.8032022371715332, "precision": 0.9093523401852086, "recall": 0.7417459995948957, "support": 34559}, "\u2205": {"f1-score": 0.930431221110526, "precision": 0.95513244038681, "recall": 0.906975417783608, "support": 17533}, "\u23ce": {"f1-score": 0.7928772258669166, "precision": 0.9101667563206025, "recall": 0.7023661270236613, "support": 2409}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 344}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.3464285714285714, "precision": 1.0, "recall": 0.20950323974082075, "support": 1389}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1331}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 94}, "\u2423": {"f1-score": 0.7283167119975822, "precision": 0.9571088165210484, "recall": 0.5878048780487805, "support": 8200}},
  "ppcr": 0.7746751931479499
}
```
</details>
