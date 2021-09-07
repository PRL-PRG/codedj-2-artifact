# Train report for javascript / file:///tmp/top-repos-quality-repos-hxj75_rb/mindsprint.git HEAD bbd9c4eb0cf10008e2f9d06c7bbaa2b5d8975385

### Classification report

PPCR: 0.041

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `"` | 1.000| 1.000| 0.362| 1.000| 0.532| 179| 494| 0.362 |
| `∅` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 2788| 0.000 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 815| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 179| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 139| 0.000 |
| `macro avg` | 0.200| 0.200| 0.072| 0.200| 0.106| 179| 4415| 0.041 |
| `micro avg` | 1.000| 1.000| 0.041| 1.000| 0.078| 179| 4415| 0.041 |
| `weighted avg` | 1.000| 1.000| 0.041| 1.000| 0.060| 179| 4415| 0.041 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| '| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|2788 |0 |0 |0 |0 |0 |
|815 |0 |0 |0 |0 |0 |
|315 |0 |0 |179 |0 |0 |
|179 |0 |0 |0 |0 |0 |
|139 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 179}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.2, "precision": 0.2, "recall": 0.2, "support": 179}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 179}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 179}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\"": {"f1-score": 0.5319465081723626, "precision": 1.0, "recall": 0.3623481781376518, "support": 494}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 139}, "macro avg": {"f1-score": 0.10638930163447251, "precision": 0.2, "recall": 0.07246963562753037, "support": 4415}, "micro avg": {"f1-score": 0.0779277318241184, "precision": 1.0, "recall": 0.040543601359003395, "support": 4415}, "weighted avg": {"f1-score": 0.05952017554635268, "precision": 0.11189127972819932, "recall": 0.040543601359003395, "support": 4415}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2788}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 179}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 815}},
  "ppcr": 0.040543601359003395
}
```
</details>
