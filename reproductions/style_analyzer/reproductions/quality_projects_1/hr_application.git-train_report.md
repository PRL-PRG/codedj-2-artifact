# Train report for javascript / file:///tmp/top-repos-quality-repos-tm7_4cmd/hr_application.git HEAD 3908b3d409e2251a8858bc4e2632ea837c7aa854

### Classification report

PPCR: 0.678

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.935| 0.999| 0.916| 0.966| 0.925| 8278| 9028| 0.917 |
| `␣` | 0.955| 0.743| 0.317| 0.836| 0.476| 1192| 2791| 0.427 |
| `⏎␣⁻␣⁻` | 0.987| 0.641| 0.638| 0.777| 0.775| 343| 345| 0.994 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 108| 904| 0.119 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 63| 369| 0.171 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 232| 0.013 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 816| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 255| 0.000 |
| `macro avg` | 0.360| 0.298| 0.234| 0.322| 0.272| 9987| 14740| 0.678 |
| `micro avg` | 0.938| 0.938| 0.636| 0.938| 0.758| 9987| 14740| 0.678 |
| `weighted avg` | 0.923| 0.938| 0.636| 0.927| 0.675| 9987| 14740| 0.678 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|750 |8266 |10 |0 |0 |0 |2 |0 |0 |
|1599 |306 |886 |0 |0 |0 |0 |0 |0 |
|796 |84 |23 |0 |0 |0 |1 |0 |0 |
|816 |0 |0 |0 |0 |0 |0 |0 |0 |
|306 |55 |8 |0 |0 |0 |0 |0 |0 |
|2 |123 |0 |0 |0 |0 |220 |0 |0 |
|255 |0 |0 |0 |0 |0 |0 |0 |0 |
|229 |2 |1 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| DB/src/employee/employee.model.js | 74 |
| labgui/src/serviceWorker.js | 59 |
| labgui/src/Login.js | 48 |
| DB/src/employee/employee.controller.js | 41 |
| labgui/src/UploadScreen.js | 39 |
| DB/src/perf_reviews/perf_reviews.model.js | 36 |
| DB/src/report/report.model.js | 32 |
| DB/src/report/report.controller.js | 30 |
| labgui/src/Pastfiles.js | 30 |
| labgui/src/api/reviewRepository.js | 28 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.32240337126054985, "precision": 0.35959745462354653, "recall": 0.29790479775002987, "support": 9987}, "micro avg": {"f1-score": 0.9384199459297087, "precision": 0.9384199459297087, "recall": 0.9384199459297087, "support": 9987}, "weighted avg": {"f1-score": 0.927151617754601, "precision": 0.9232435466226503, "recall": 0.9384199459297087, "support": 9987}, "\u2205": {"f1-score": 0.9659927544700246, "precision": 0.9354911724762336, "recall": 0.998550374486591, "support": 8278}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 108}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 63}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7773851590106007, "precision": 0.9865470852017937, "recall": 0.641399416909621, "support": 343}, "\u2423": {"f1-score": 0.8358490566037735, "precision": 0.9547413793103449, "recall": 0.7432885906040269, "support": 1192}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 255}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 816}, "macro avg": {"f1-score": 0.27206958619941035, "precision": 0.35959745462354653, "recall": 0.23384075328051332, "support": 14740}, "micro avg": {"f1-score": 0.7580377724754317, "precision": 0.9384199459297087, "recall": 0.6358208955223881, "support": 14740}, "weighted avg": {"f1-score": 0.675164807607065, "precision": 0.7768423500112095, "recall": 0.6358208955223881, "support": 14740}, "\u2205": {"f1-score": 0.925436632333184, "precision": 0.9354911724762336, "recall": 0.9155959237926451, "support": 9028}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 904}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 232}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 369}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7746478873239436, "precision": 0.9865470852017937, "recall": 0.6376811594202898, "support": 345}, "\u2423": {"f1-score": 0.4764721699381554, "precision": 0.9547413793103449, "recall": 0.3174489430311716, "support": 2791}},
  "ppcr": 0.6775440976933514
}
```
</details>
