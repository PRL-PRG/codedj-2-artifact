# Test report for javascript / file:///tmp/top-repos-quality-repos-iuygsz4o/radiopadre.git HEAD 3bf934eba69144d9707777a57da0e827625517a3

### Classification report

PPCR: 0.404

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 0.956| 0.484| 0.978| 0.653| 1421| 2805| 0.507 |
| `␣` | 0.820| 1.000| 0.326| 0.901| 0.466| 301| 924| 0.326 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 390| 0.010 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 116| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 40| 0.000 |
| `weighted avg` | 0.966| 0.962| 0.388| 0.962| 0.529| 1726| 4275| 0.404 |
| `micro avg` | 0.962| 0.962| 0.388| 0.962| 0.553| 1726| 4275| 0.404 |
| `macro avg` | 0.364| 0.391| 0.162| 0.376| 0.224| 1726| 4275| 0.404 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|1384 |1359 |62 |0 |0 |0 |
|623 |0 |301 |0 |0 |0 |
|386 |0 |4 |0 |0 |0 |
|40 |0 |0 |0 |0 |0 |
|116 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.3757790893034076, "precision": 0.3640326975476839, "recall": 0.3912737508796622, "support": 1726}, "micro avg": {"f1-score": 0.9617612977983777, "precision": 0.9617612977983777, "recall": 0.9617612977983777, "support": 1726}, "weighted avg": {"f1-score": 0.9620910267296878, "precision": 0.966320515532598, "recall": 0.9617612977983777, "support": 1726}, "\u2205": {"f1-score": 0.9776978417266187, "precision": 1.0, "recall": 0.956368754398311, "support": 1421}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u2423": {"f1-score": 0.9011976047904192, "precision": 0.8201634877384196, "recall": 1.0, "support": 301}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 116}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 40}, "macro avg": {"f1-score": 0.22380858838735027, "precision": 0.3640326975476839, "recall": 0.16204991087344028, "support": 4275}, "micro avg": {"f1-score": 0.5532411264789203, "precision": 0.9617612977983777, "recall": 0.3883040935672515, "support": 4275}, "weighted avg": {"f1-score": 0.5290749450678645, "precision": 0.8334107748936374, "recall": 0.3883040935672515, "support": 4275}, "\u2205": {"f1-score": 0.6527377521613833, "precision": 1.0, "recall": 0.4844919786096257, "support": 2805}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 390}, "\u2423": {"f1-score": 0.466305189775368, "precision": 0.8201634877384196, "recall": 0.32575757575757575, "support": 924}},
  "ppcr": 0.4037426900584795
}
```
</details>
