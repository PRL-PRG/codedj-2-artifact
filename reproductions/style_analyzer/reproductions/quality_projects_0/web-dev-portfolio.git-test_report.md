# Test report for javascript / file:///tmp/top-repos-quality-repos-d_85bwa5/web-dev-portfolio.git HEAD 072995f16f6bc081ce106652b0966b8431e233c6

### Classification report

PPCR: 0.579

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.985| 0.997| 0.808| 0.991| 0.888| 600| 740| 0.811 |
| `"` | 0.924| 1.000| 0.388| 0.961| 0.547| 73| 188| 0.388 |
| `␣` | 1.000| 0.849| 0.178| 0.918| 0.302| 53| 253| 0.209 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 54| 0.093 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 28| 0.000 |
| `macro avg` | 0.582| 0.569| 0.275| 0.574| 0.347| 731| 1263| 0.579 |
| `micro avg` | 0.979| 0.979| 0.567| 0.979| 0.718| 731| 1263| 0.579 |
| `weighted avg` | 0.973| 0.979| 0.567| 0.976| 0.662| 731| 1263| 0.579 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| '| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|140 |598 |0 |2 |0 |0 |
|200 |4 |45 |4 |0 |0 |
|115 |0 |0 |73 |0 |0 |
|49 |5 |0 |0 |0 |0 |
|28 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9605263157894737, "precision": 0.9240506329113924, "recall": 1.0, "support": 73}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.5739560316342993, "precision": 0.5818447229578962, "recall": 0.5691446540880503, "support": 731}, "micro avg": {"f1-score": 0.9794801641586868, "precision": 0.9794801641586868, "recall": 0.9794801641586868, "support": 731}, "weighted avg": {"f1-score": 0.9758191350291865, "precision": 0.9734055886858892, "recall": 0.9794801641586868, "support": 731}, "\u2205": {"f1-score": 0.9908864954432478, "precision": 0.985172981878089, "recall": 0.9966666666666667, "support": 600}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u2423": {"f1-score": 0.9183673469387755, "precision": 1.0, "recall": 0.8490566037735849, "support": 53}},
  "cl_report_full": {"\"": {"f1-score": 0.5468164794007491, "precision": 0.9240506329113924, "recall": 0.3882978723404255, "support": 188}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 28}, "macro avg": {"f1-score": 0.34734578742237887, "precision": 0.5818447229578962, "recall": 0.27485431861935095, "support": 1263}, "micro avg": {"f1-score": 0.7181544633901705, "precision": 0.9794801641586868, "recall": 0.566904196357878, "support": 1263}, "weighted avg": {"f1-score": 0.6621189073007416, "precision": 0.915082759760196, "recall": 0.566904196357878, "support": 1263}, "\u2205": {"f1-score": 0.8878990348923534, "precision": 0.985172981878089, "recall": 0.8081081081081081, "support": 740}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 54}, "\u2423": {"f1-score": 0.3020134228187919, "precision": 1.0, "recall": 0.17786561264822134, "support": 253}},
  "ppcr": 0.5787806809184481
}
```
</details>
