# Train report for javascript / file:///tmp/top-repos-quality-repos-iuygsz4o/radiopadre.git HEAD 3bf934eba69144d9707777a57da0e827625517a3

### Classification report

PPCR: 0.313

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.503| 1.000| 0.669| 1975| 3928| 0.503 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1322| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 616| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 220| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 231| 0.000 |
| `weighted avg` | 1.000| 1.000| 0.313| 1.000| 0.416| 1975| 6317| 0.313 |
| `micro avg` | 1.000| 1.000| 0.313| 1.000| 0.476| 1975| 6317| 0.313 |
| `macro avg` | 0.200| 0.200| 0.101| 0.200| 0.134| 1975| 6317| 0.313 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|1953 |1975 |0 |0 |0 |0 |
|1322 |0 |0 |0 |0 |0 |
|616 |0 |0 |0 |0 |0 |
|220 |0 |0 |0 |0 |0 |
|231 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.2, "precision": 0.2, "recall": 0.2, "support": 1975}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1975}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1975}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1975}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 231}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 220}, "macro avg": {"f1-score": 0.1338302558021345, "precision": 0.2, "recall": 0.1005600814663951, "support": 6317}, "micro avg": {"f1-score": 0.47636275928605887, "precision": 1.0, "recall": 0.31264840905493113, "support": 6317}, "weighted avg": {"f1-score": 0.4160877353101032, "precision": 0.6218141522874783, "recall": 0.3126484090549311, "support": 6317}, "\u2205": {"f1-score": 0.6691512790106725, "precision": 1.0, "recall": 0.5028004073319755, "support": 3928}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 616}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1322}},
  "ppcr": 0.31264840905493113
}
```
</details>
