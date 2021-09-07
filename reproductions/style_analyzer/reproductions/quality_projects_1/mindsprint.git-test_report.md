# Test report for javascript / file:///tmp/top-repos-quality-repos-hxj75_rb/mindsprint.git HEAD bbd9c4eb0cf10008e2f9d06c7bbaa2b5d8975385

### Classification report

PPCR: 0.933

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.752| 1.000| 0.998| 0.859| 0.858| 522| 523| 0.998 |
| `␣` | 1.000| 0.083| 0.075| 0.153| 0.139| 145| 161| 0.901 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 39| 39| 1.000 |
| `"` | 0.512| 1.000| 0.550| 0.677| 0.530| 22| 40| 0.550 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 21| 40| 0.525 |
| `macro avg` | 0.453| 0.417| 0.325| 0.338| 0.305| 749| 803| 0.933 |
| `micro avg` | 0.742| 0.742| 0.692| 0.742| 0.716| 749| 803| 0.933 |
| `weighted avg` | 0.733| 0.742| 0.692| 0.648| 0.613| 749| 803| 0.933 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| '| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|1 |522 |0 |0 |0 |0 |
|16 |133 |12 |0 |0 |0 |
|18 |0 |0 |22 |0 |0 |
|0 |39 |0 |0 |0 |0 |
|19 |0 |0 |21 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.676923076923077, "precision": 0.5116279069767442, "recall": 1.0, "support": 22}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 21}, "macro avg": {"f1-score": 0.33766839010804817, "precision": 0.45275785805240937, "recall": 0.41655172413793107, "support": 749}, "micro avg": {"f1-score": 0.7423230974632844, "precision": 0.7423230974632844, "recall": 0.7423230974632844, "support": 749}, "weighted avg": {"f1-score": 0.6478269512310543, "precision": 0.7328225047108362, "recall": 0.7423230974632844, "support": 749}, "\u2205": {"f1-score": 0.8585526315789473, "precision": 0.7521613832853026, "recall": 1.0, "support": 522}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 39}, "\u2423": {"f1-score": 0.15286624203821655, "precision": 1.0, "recall": 0.08275862068965517, "support": 145}},
  "cl_report_full": {"\"": {"f1-score": 0.5301204819277109, "precision": 0.5116279069767442, "recall": 0.55, "support": 40}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 40}, "macro avg": {"f1-score": 0.30533919415747257, "precision": 0.45275785805240937, "recall": 0.3245244231203164, "support": 803}, "micro avg": {"f1-score": 0.7164948453608248, "precision": 0.7423230974632844, "recall": 0.6924034869240349, "support": 803}, "weighted avg": {"f1-score": 0.612944142928413, "precision": 0.7158723782531545, "recall": 0.6924034869240349, "support": 803}, "\u2205": {"f1-score": 0.85784716516023, "precision": 0.7521613832853026, "recall": 0.9980879541108987, "support": 523}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 39}, "\u2423": {"f1-score": 0.13872832369942198, "precision": 1.0, "recall": 0.07453416149068323, "support": 161}},
  "ppcr": 0.9327521793275217
}
```
</details>
