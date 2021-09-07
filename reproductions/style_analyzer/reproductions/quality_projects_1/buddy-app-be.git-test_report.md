# Test report for javascript / file:///tmp/top-repos-quality-repos-v8ugd2xp/buddy-app-be.git HEAD 66d503f27d1442029f021bba3b15a4d7573daed2

### Classification report

PPCR: 0.828

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.870| 0.949| 0.904| 0.908| 0.887| 820| 861| 0.952 |
| `␣` | 0.789| 0.693| 0.548| 0.738| 0.647| 238| 301| 0.791 |
| `⏎␣⁺␣⁺` | 0.803| 0.742| 0.742| 0.772| 0.772| 66| 66| 1.000 |
| `"` | 1.000| 1.000| 0.500| 1.000| 0.667| 50| 100| 0.500 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 30| 67| 0.448 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 32| 0.281 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 40| 0.025 |
| `macro avg` | 0.495| 0.483| 0.385| 0.488| 0.425| 1214| 1467| 0.828 |
| `micro avg` | 0.858| 0.858| 0.710| 0.858| 0.777| 1214| 1467| 0.828 |
| `weighted avg` | 0.827| 0.858| 0.710| 0.841| 0.733| 1214| 1467| 0.828 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|41 |778 |42 |0 |0 |0 |0 |0 |
|63 |61 |165 |0 |0 |12 |0 |0 |
|50 |0 |0 |50 |0 |0 |0 |0 |
|37 |30 |0 |0 |0 |0 |0 |0 |
|0 |17 |0 |0 |0 |49 |0 |0 |
|23 |7 |2 |0 |0 |0 |0 |0 |
|39 |1 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 50}, "macro avg": {"f1-score": 0.4882466495036777, "precision": 0.49471406539232887, "recall": 0.48349743445049853, "support": 1214}, "micro avg": {"f1-score": 0.8583196046128501, "precision": 0.8583196046128501, "recall": 0.8583196046128501, "support": 1214}, "weighted avg": {"f1-score": 0.8410581276502186, "precision": 0.8274406260246299, "recall": 0.8583196046128501, "support": 1214}, "\u2205": {"f1-score": 0.9078179696616103, "precision": 0.8702460850111857, "recall": 0.948780487804878, "support": 820}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 30}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7716535433070867, "precision": 0.8032786885245902, "recall": 0.7424242424242424, "support": 66}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u2423": {"f1-score": 0.738255033557047, "precision": 0.7894736842105263, "recall": 0.6932773109243697, "support": 238}},
  "cl_report_full": {"\"": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 100}, "macro avg": {"f1-score": 0.4245698171589788, "precision": 0.49471406539232887, "recall": 0.38488535206791424, "support": 1467}, "micro avg": {"f1-score": 0.7773218948153673, "precision": 0.8583196046128501, "recall": 0.7102931152010906, "support": 1467}, "weighted avg": {"f1-score": 0.7332866029844769, "precision": 0.777048296922033, "recall": 0.7102931152010906, "support": 1467}, "\u2205": {"f1-score": 0.8866096866096866, "precision": 0.8702460850111857, "recall": 0.9036004645760743, "support": 861}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 67}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 40}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7716535433070867, "precision": 0.8032786885245902, "recall": 0.7424242424242424, "support": 66}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 32}, "\u2423": {"f1-score": 0.6470588235294118, "precision": 0.7894736842105263, "recall": 0.5481727574750831, "support": 301}},
  "ppcr": 0.8275391956373551
}
```
</details>
