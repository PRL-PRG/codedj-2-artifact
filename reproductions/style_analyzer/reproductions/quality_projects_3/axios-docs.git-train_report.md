# Train report for javascript / file:///tmp/top-repos-quality-repos-f_nv0o4o/axios-docs.git HEAD 16aa2ce7fa42e7c46407b78966b7521d8e588a72

### Classification report

PPCR: 0.813

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.965| 0.995| 0.974| 0.980| 0.969| 16497| 16848| 0.979 |
| `␣` | 0.967| 0.922| 0.575| 0.944| 0.722| 4094| 6561| 0.624 |
| `'` | 1.000| 1.000| 0.994| 1.000| 0.997| 3233| 3254| 0.994 |
| `⏎␣⁻␣⁻` | 0.992| 0.956| 0.717| 0.973| 0.833| 900| 1199| 0.751 |
| `⏎⏎` | 0.927| 0.715| 0.394| 0.807| 0.553| 445| 808| 0.551 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 161| 1316| 0.122 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 33| 1206| 0.027 |
| `macro avg` | 0.693| 0.655| 0.522| 0.672| 0.582| 25363| 31192| 0.813 |
| `weighted avg` | 0.962| 0.970| 0.789| 0.966| 0.826| 25363| 31192| 0.813 |
| `micro avg` | 0.970| 0.970| 0.789| 0.970| 0.870| 25363| 31192| 0.813 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|351 |16415 |82 |0 |0 |0 |0 |0 |
|2467 |313 |3775 |0 |0 |0 |6 |0 |
|21 |0 |0 |3233 |0 |0 |0 |0 |
|1155 |109 |26 |0 |0 |0 |1 |25 |
|1173 |25 |8 |0 |0 |0 |0 |0 |
|299 |31 |9 |0 |0 |0 |860 |0 |
|363 |125 |2 |0 |0 |0 |0 |318 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| lib/adapters/http.js | 98 |
| lib/utils.js | 75 |
| lib/adapters/xhr.js | 47 |
| karma.conf.js | 38 |
| examples/server.js | 35 |
| lib/helpers/buildURL.js | 32 |
| test/specs/requests.spec.js | 29 |
| lib/core/dispatchRequest.js | 26 |
| lib/helpers/isURLSameOrigin.js | 26 |
| lib/defaults.js | 23 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 3233}, "macro avg": {"f1-score": 0.672041620787229, "precision": 0.6930084860587947, "recall": 0.6553246843853752, "support": 25363}, "micro avg": {"f1-score": 0.9699562354611048, "precision": 0.9699562354611048, "recall": 0.9699562354611048, "support": 25363}, "weighted avg": {"f1-score": 0.9657253927984806, "precision": 0.9624851432629676, "recall": 0.9699562354611048, "support": 25363}, "\u2205": {"f1-score": 0.9795613904221989, "precision": 0.9645669291338582, "recall": 0.9950293992847185, "support": 16497}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 161}, "\u23ce\u23ce": {"f1-score": 0.8071065989847717, "precision": 0.9271137026239067, "recall": 0.7146067415730337, "support": 445}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 33}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9734012450481041, "precision": 0.9919261822376009, "recall": 0.9555555555555556, "support": 900}, "\u2423": {"f1-score": 0.9442221110555278, "precision": 0.9674525884161969, "recall": 0.9220810942843185, "support": 4094}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9967627562817943, "precision": 1.0, "recall": 0.9935464044253227, "support": 3254}, "macro avg": {"f1-score": 0.5818359398801017, "precision": 0.6930084860587947, "recall": 0.5220063394677853, "support": 31192}, "micro avg": {"f1-score": 0.8699849703828132, "precision": 0.9699562354611048, "recall": 0.7886958194408823, "support": 31192}, "weighted avg": {"f1-score": 0.8256954014710408, "precision": 0.8909626641083903, "recall": 0.7886958194408823, "support": 31192}, "\u2205": {"f1-score": 0.9694088466308391, "precision": 0.9645669291338582, "recall": 0.9742996201329535, "support": 16848}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1316}, "\u23ce\u23ce": {"f1-score": 0.5525629887054736, "precision": 0.9271137026239067, "recall": 0.3935643564356436, "support": 808}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1206}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8325266214908034, "precision": 0.9919261822376009, "recall": 0.7172643869891576, "support": 1199}, "\u2423": {"f1-score": 0.7215903660518016, "precision": 0.9674525884161969, "recall": 0.575369608291419, "support": 6561}},
  "ppcr": 0.8131251602975121
}
```
</details>
