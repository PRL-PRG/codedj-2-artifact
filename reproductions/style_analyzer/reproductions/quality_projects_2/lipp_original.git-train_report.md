# Train report for javascript / file:///tmp/top-repos-quality-repos-mi28wn6_/lipp_original.git HEAD 0a004465cb7c5a7ab028ee2f71aa00f231a20f13

### Classification report

PPCR: 0.949

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `'` | 1.000| 1.000| 0.998| 1.000| 0.999| 13446| 13471| 0.998 |
| `∅` | 0.979| 0.993| 0.931| 0.986| 0.954| 13387| 14273| 0.938 |
| `␣` | 0.986| 0.975| 0.919| 0.980| 0.952| 9479| 10048| 0.943 |
| `⏎` | 0.997| 0.983| 0.924| 0.990| 0.959| 5360| 5704| 0.940 |
| `⏎␣⁻␣⁻` | 0.997| 0.998| 0.998| 0.997| 0.997| 1395| 1395| 1.000 |
| `⏎␣⁺␣⁺` | 0.996| 0.992| 0.959| 0.994| 0.977| 1379| 1426| 0.967 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 513| 0.006 |
| `weighted avg` | 0.990| 0.990| 0.940| 0.990| 0.959| 44449| 46830| 0.949 |
| `micro avg` | 0.990| 0.990| 0.940| 0.990| 0.964| 44449| 46830| 0.949 |
| `macro avg` | 0.851| 0.849| 0.819| 0.850| 0.834| 44449| 46830| 0.949 |

### Confusion matrix

|refusal|  ∅| '| ␣| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|886 |13294 |0 |85 |8 |0 |0 |0 |
|25 |0 |13446 |0 |0 |0 |0 |0 |
|569 |226 |0 |9239 |5 |5 |4 |0 |
|344 |55 |0 |36 |5269 |0 |0 |0 |
|47 |5 |0 |6 |0 |1368 |0 |0 |
|0 |3 |0 |0 |0 |0 |1392 |0 |
|510 |1 |0 |1 |1 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| app/static/libraries/glide@3.3.0/glide.js | 305 |
| app/static/libraries/summernote@0.8.11/plugin/specialchars/summernote-ext-specialchars.js | 47 |
| app/static/libraries/summernote@0.8.11/plugin/databasic/summernote-ext-databasic.js | 34 |
| app/static/libraries/timeline/timeline.js | 9 |
| app/static/libraries/summernote@0.8.11/plugin/hello/summernote-ext-hello.js | 3 |
| app/static/libraries/summernote@0.8.11/lang/summernote-ko-KR.js | 1 |
| app/static/libraries/summernote@0.8.11/lang/summernote-es-EU.js | 1 |
| app/static/libraries/summernote@0.8.11/lang/summernote-nl-NL.js | 1 |
| app/static/libraries/summernote@0.8.11/lang/summernote-vi-VN.js | 1 |
| app/static/libraries/summernote@0.8.11/lang/summernote-ca-ES.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 13446}, "macro avg": {"f1-score": 0.8497264597109762, "precision": 0.8508327658331397, "recall": 0.8486612701407102, "support": 44449}, "micro avg": {"f1-score": 0.9900785169520124, "precision": 0.9900785169520124, "recall": 0.9900785169520124, "support": 44449}, "weighted avg": {"f1-score": 0.9900427283742625, "precision": 0.9900661920054775, "recall": 0.9900785169520124, "support": 44449}, "\u2205": {"f1-score": 0.9857995624930481, "precision": 0.9786513545347467, "recall": 0.9930529618286398, "support": 13387}, "\u23ce": {"f1-score": 0.9901343606126092, "precision": 0.9973499905356805, "recall": 0.9830223880597015, "support": 5360}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.994186046511628, "precision": 0.9963583394027676, "recall": 0.9920232052211748, "support": 1379}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9974919383733428, "precision": 0.997134670487106, "recall": 0.9978494623655914, "support": 1395}, "\u2423": {"f1-score": 0.980473309986204, "precision": 0.9863350058716772, "recall": 0.9746808735098639, "support": 9479}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9990712189322732, "precision": 1.0, "recall": 0.9981441615321802, "support": 13471}, "macro avg": {"f1-score": 0.8341961295756449, "precision": 0.8508327658331397, "recall": 0.8185647941377127, "support": 46830}, "micro avg": {"f1-score": 0.9642524567534702, "precision": 0.9900785169520124, "recall": 0.9397394832372411, "support": 46830}, "weighted avg": {"f1-score": 0.9588012879363375, "precision": 0.9790877455820295, "recall": 0.9397394832372411, "support": 46830}, "\u2205": {"f1-score": 0.9544459202354884, "precision": 0.9786513545347467, "recall": 0.9314089539690324, "support": 14273}, "\u23ce": {"f1-score": 0.9591335214344225, "precision": 0.9973499905356805, "recall": 0.9237377279102384, "support": 5704}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 513}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.977491961414791, "precision": 0.9963583394027676, "recall": 0.9593267882187938, "support": 1426}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9974919383733428, "precision": 0.997134670487106, "recall": 0.9978494623655914, "support": 1395}, "\u2423": {"f1-score": 0.9517383466391965, "precision": 0.9863350058716772, "recall": 0.9194864649681529, "support": 10048}},
  "ppcr": 0.9491565235959855
}
```
</details>
