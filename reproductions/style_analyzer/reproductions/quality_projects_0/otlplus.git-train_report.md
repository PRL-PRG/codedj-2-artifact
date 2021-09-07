# Train report for javascript / file:///tmp/top-repos-quality-repos-9q1ttk53/otlplus.git HEAD 4c42150d13d7ef51c08baf54a71117ab40fee93e

### Classification report

PPCR: 0.821

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.972| 0.996| 0.902| 0.984| 0.935| 34089| 37649| 0.905 |
| `␣` | 0.979| 0.957| 0.754| 0.968| 0.852| 13536| 17189| 0.787 |
| `'` | 1.000| 1.000| 0.979| 1.000| 0.989| 5962| 6090| 0.979 |
| `⏎␣⁺␣⁺` | 0.947| 0.831| 0.602| 0.885| 0.736| 1462| 2019| 0.724 |
| `⏎␣⁻␣⁻` | 0.965| 0.888| 0.684| 0.925| 0.801| 1431| 1857| 0.771 |
| `"` | 1.000| 1.000| 1.000| 1.000| 1.000| 176| 176| 1.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 128| 3092| 0.041 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 80| 103| 0.777 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 37| 986| 0.038 |
| `⏎⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 11| 193| 0.057 |
| `macro avg` | 0.586| 0.567| 0.492| 0.576| 0.531| 56912| 69354| 0.821 |
| `micro avg` | 0.976| 0.976| 0.801| 0.976| 0.880| 56912| 69354| 0.821 |
| `weighted avg` | 0.971| 0.976| 0.801| 0.973| 0.851| 56912| 69354| 0.821 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| ⏎⏎⏎| "| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3560 |33948 |136 |0 |0 |5 |0 |0 |0 |0 |0 |
|3653 |525 |12956 |0 |0 |55 |0 |0 |0 |0 |0 |
|128 |0 |0 |5962 |0 |0 |0 |0 |0 |0 |0 |
|2964 |71 |49 |0 |0 |8 |0 |0 |0 |0 |0 |
|557 |177 |70 |0 |0 |1215 |0 |0 |0 |0 |0 |
|426 |142 |18 |0 |0 |0 |1271 |0 |0 |0 |0 |
|949 |36 |1 |0 |0 |0 |0 |0 |0 |0 |0 |
|182 |10 |1 |0 |0 |0 |0 |0 |0 |0 |0 |
|0 |0 |0 |0 |0 |0 |0 |0 |0 |176 |0 |
|23 |34 |0 |0 |0 |0 |46 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| react/src/common/lectureFunctions.js | 76 |
| react/src/components/sections/timetable/TimetableSubSection.js | 67 |
| react/src/pages/CreditPage.js | 66 |
| react/src/components/sections/timetable/LectureListSection.js | 54 |
| react/src/components/sections/timetable/SummarySubSection.js | 48 |
| react/src/components/tabs/LectureListTabs.js | 46 |
| react/src/components/tabs/CourseListTabs.js | 41 |
| react/src/registerServiceWorker.js | 38 |
| react/src/components/tabs/TimetableTabs.js | 38 |
| react/src/components/sections/dictionary/CourseDetailSection.js | 36 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 176}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 5962}, "macro avg": {"f1-score": 0.5761883838424449, "precision": 0.5862811888103334, "recall": 0.5672258497049946, "support": 56912}, "micro avg": {"f1-score": 0.9756817542873207, "precision": 0.9756817542873207, "recall": 0.9756817542873207, "support": 56912}, "weighted avg": {"f1-score": 0.9732150012172942, "precision": 0.9712628164131498, "recall": 0.9756817542873207, "support": 56912}, "\u2205": {"f1-score": 0.9835438637153783, "precision": 0.9715250550897175, "recall": 0.9958637683710287, "support": 34089}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 128}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 37}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8852459016393442, "precision": 0.9469992205767732, "recall": 0.8310533515731874, "support": 1462}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9250363901018922, "precision": 0.9650721336370539, "recall": 0.8881900768693222, "support": 1431}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 80}, "\u2423": {"f1-score": 0.9680576829678335, "precision": 0.9792154787997884, "recall": 0.9571513002364066, "support": 13536}},
  "cl_report_full": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 176}, "\u0027": {"f1-score": 0.9893793561234651, "precision": 1.0, "recall": 0.9789819376026273, "support": 6090}, "macro avg": {"f1-score": 0.5313296846452861, "precision": 0.5862811888103334, "recall": 0.4920637374769384, "support": 69354}, "micro avg": {"f1-score": 0.8795400186907005, "precision": 0.9756817542873207, "recall": 0.8006459612999971, "support": 69354}, "weighted avg": {"f1-score": 0.8511344438036457, "precision": 0.9138450852534945, "recall": 0.8006459612999971, "support": 69354}, "\u2205": {"f1-score": 0.935309675997355, "precision": 0.9715250550897175, "recall": 0.9016972562352253, "support": 37649}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3092}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 986}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 193}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7359176256814052, "precision": 0.9469992205767732, "recall": 0.6017830609212481, "support": 2019}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8008821676118463, "precision": 0.9650721336370539, "recall": 0.6844372644049542, "support": 1857}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 103}, "\u2423": {"f1-score": 0.8518080210387902, "precision": 0.9792154787997884, "recall": 0.753737855605329, "support": 17189}},
  "ppcr": 0.8206015514606223
}
```
</details>
