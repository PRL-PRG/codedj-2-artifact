# Test report for javascript / file:///tmp/top-repos-quality-repos-tirec064/nodeciudemy.git HEAD 20606fce8a2305e2424235578ed09985e6642270

### Classification report

PPCR: 0.572

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.864| 1.000| 0.840| 0.927| 0.852| 293| 349| 0.840 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 35| 166| 0.211 |
| `'` | 1.000| 0.909| 0.500| 0.952| 0.667| 22| 40| 0.550 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 16| 0.375 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 18| 0.111 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 24| 0.042 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 15| 0.000 |
| `weighted avg` | 0.767| 0.872| 0.498| 0.815| 0.516| 359| 628| 0.572 |
| `micro avg` | 0.872| 0.872| 0.498| 0.872| 0.634| 359| 628| 0.572 |
| `macro avg` | 0.266| 0.273| 0.191| 0.269| 0.217| 359| 628| 0.572 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|56 |293 |0 |0 |0 |0 |0 |0 |
|131 |35 |0 |0 |0 |0 |0 |0 |
|18 |2 |0 |20 |0 |0 |0 |0 |
|23 |1 |0 |0 |0 |0 |0 |0 |
|16 |2 |0 |0 |0 |0 |0 |0 |
|10 |6 |0 |0 |0 |0 |0 |0 |
|15 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.9523809523809523, "precision": 1.0, "recall": 0.9090909090909091, "support": 22}, "macro avg": {"f1-score": 0.2685137346077671, "precision": 0.26632954066582387, "recall": 0.27272727272727276, "support": 359}, "micro avg": {"f1-score": 0.871866295264624, "precision": 0.871866295264624, "recall": 0.871866295264624, "support": 359}, "weighted avg": {"f1-score": 0.8151154083155775, "precision": 0.7666904955587874, "recall": 0.871866295264624, "support": 359}, "\u2205": {"f1-score": 0.9272151898734177, "precision": 0.8643067846607669, "recall": 1.0, "support": 293}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 35}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 40}, "macro avg": {"f1-score": 0.2169158361018826, "precision": 0.26632954066582387, "recall": 0.1913630781825624, "support": 628}, "micro avg": {"f1-score": 0.6342451874366768, "precision": 0.871866295264624, "recall": 0.4984076433121019, "support": 628}, "weighted avg": {"f1-score": 0.5158047573199032, "precision": 0.5440176239595663, "recall": 0.4984076433121019, "support": 628}, "\u2205": {"f1-score": 0.8517441860465116, "precision": 0.8643067846607669, "recall": 0.839541547277937, "support": 349}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 24}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 15}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 18}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 166}},
  "ppcr": 0.571656050955414
}
```
</details>
