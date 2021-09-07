# Train report for javascript / file:///tmp/top-repos-quality-repos-elcbw4bb/headstocks.git HEAD 70862f8df43c7af30db5fdb228fac612466661b2

### Classification report

PPCR: 0.760

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.956| 0.998| 0.962| 0.977| 0.959| 35688| 37024| 0.964 |
| `␣` | 0.971| 0.940| 0.627| 0.956| 0.762| 9696| 14554| 0.666 |
| `"` | 1.000| 1.000| 0.468| 1.000| 0.638| 2716| 5804| 0.468 |
| `⏎` | 0.908| 0.826| 0.375| 0.865| 0.531| 2533| 5577| 0.454 |
| `⏎␣⁺␣⁺` | 0.968| 0.674| 0.385| 0.794| 0.551| 1474| 2579| 0.572 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 402| 2288| 0.176 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 148| 868| 0.171 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 42| 188| 0.223 |
| `'` | 1.000| 1.000| 0.009| 1.000| 0.018| 4| 448| 0.009 |
| `weighted avg` | 0.948| 0.959| 0.729| 0.953| 0.789| 52703| 69330| 0.760 |
| `macro avg` | 0.645| 0.604| 0.314| 0.621| 0.384| 52703| 69330| 0.760 |
| `micro avg` | 0.959| 0.959| 0.729| 0.959| 0.828| 52703| 69330| 0.760 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| '| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1336 |35622 |64 |0 |0 |2 |0 |0 |0 |0 |
|4858 |476 |9119 |0 |70 |31 |0 |0 |0 |0 |
|3088 |0 |0 |2716 |0 |0 |0 |0 |0 |0 |
|3044 |347 |93 |0 |2093 |0 |0 |0 |0 |0 |
|1105 |396 |84 |0 |1 |993 |0 |0 |0 |0 |
|1886 |302 |31 |0 |69 |0 |0 |0 |0 |0 |
|720 |78 |0 |0 |70 |0 |0 |0 |0 |0 |
|444 |0 |0 |0 |0 |0 |0 |0 |4 |0 |
|146 |41 |0 |0 |1 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| server/routes/api/CompanyDetail.js | 103 |
| server/tests/api/Users.test.js | 100 |
| client/src/actions/CompanyDetail.js | 90 |
| client/config/webpack.config.js | 74 |
| server/routes/api/Users.js | 71 |
| client/src/components/NavbarDefault.js | 71 |
| client/src/components/Register.js | 58 |
| client/src/components/__tests__/Register.test.js | 55 |
| client/scripts/build.js | 51 |
| client/src/reducers/CompanyDetailReducer.js | 48 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 2716}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 4}, "macro avg": {"f1-score": 0.6213273364999762, "precision": 0.6448088697256258, "recall": 0.6042901739380855, "support": 52703}, "micro avg": {"f1-score": 0.9590915128171071, "precision": 0.9590915128171071, "recall": 0.9590915128171071, "support": 52703}, "weighted avg": {"f1-score": 0.9525293955062121, "precision": 0.9483342656593147, "recall": 0.9590915128171071, "support": 52703}, "\u2205": {"f1-score": 0.9766141192597669, "precision": 0.9559873329397242, "recall": 0.9981506388702085, "support": 35688}, "\u23ce": {"f1-score": 0.8654124457308249, "precision": 0.9084201388888888, "recall": 0.8262929332806949, "support": 2533}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 148}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7943999999999999, "precision": 0.9678362573099415, "recall": 0.6736770691994572, "support": 1474}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 402}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 42}, "\u2423": {"f1-score": 0.9555194635091947, "precision": 0.9710360983920775, "recall": 0.9404909240924092, "support": 9696}},
  "cl_report_full": {"\"": {"f1-score": 0.6375586854460094, "precision": 1.0, "recall": 0.46795313576843556, "support": 5804}, "\u0027": {"f1-score": 0.017699115044247787, "precision": 1.0, "recall": 0.008928571428571428, "support": 448}, "macro avg": {"f1-score": 0.3842247216930206, "precision": 0.6448088697256258, "recall": 0.3139890950947033, "support": 69330}, "micro avg": {"f1-score": 0.8284152647234764, "precision": 0.9590915128171071, "recall": 0.7290783210731285, "support": 69330}, "weighted avg": {"f1-score": 0.788755619469814, "precision": 0.913619547280316, "recall": 0.7290783210731285, "support": 69330}, "\u2205": {"f1-score": 0.9590501574993942, "precision": 0.9559873329397242, "recall": 0.9621326707000865, "support": 37024}, "\u23ce": {"f1-score": 0.5311508691790382, "precision": 0.9084201388888888, "recall": 0.3752913752913753, "support": 5577}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 868}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.5509015256588072, "precision": 0.9678362573099415, "recall": 0.3850329585110508, "support": 2579}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2288}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 188}, "\u2423": {"f1-score": 0.7616621424096888, "precision": 0.9710360983920775, "recall": 0.6265631441528102, "support": 14554}},
  "ppcr": 0.7601759699985576
}
```
</details>
