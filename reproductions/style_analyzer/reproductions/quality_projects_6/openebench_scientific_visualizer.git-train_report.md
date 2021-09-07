# Train report for javascript / file:///tmp/top-repos-quality-repos-w5v778i1/openebench_scientific_visualizer.git HEAD 859040441e881d748992b66079df96fdd3cf31e6

### Classification report

PPCR: 0.511

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.986| 0.995| 0.787| 0.990| 0.875| 5671| 7167| 0.791 |
| `␣` | 0.962| 0.930| 0.299| 0.945| 0.456| 922| 2866| 0.322 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 12| 627| 0.019 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 138| 0.029 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 147| 0.007 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1574| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 262| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 152| 0.000 |
| `micro avg` | 0.983| 0.983| 0.502| 0.983| 0.665| 6610| 12933| 0.511 |
| `weighted avg` | 0.980| 0.983| 0.502| 0.982| 0.586| 6610| 12933| 0.511 |
| `macro avg` | 0.244| 0.241| 0.136| 0.242| 0.166| 6610| 12933| 0.511 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| '| ⏎⏎| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1496 |5640 |31 |0 |0 |0 |0 |0 |0 |
|1944 |65 |857 |0 |0 |0 |0 |0 |0 |
|1574 |0 |0 |0 |0 |0 |0 |0 |0 |
|615 |9 |3 |0 |0 |0 |0 |0 |0 |
|262 |0 |0 |0 |0 |0 |0 |0 |0 |
|152 |0 |0 |0 |0 |0 |0 |0 |0 |
|134 |4 |0 |0 |0 |0 |0 |0 |0 |
|146 |1 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/app.js | 21 |
| src/scatter_plot.js | 21 |
| src/legend.js | 14 |
| src/diagonals.js | 13 |
| src/squares.js | 12 |
| src/classification.js | 8 |
| src/clusters.js | 8 |
| src/table.js | 6 |
| src/chart_coordinates.js | 5 |
| src/selection_list.js | 3 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.2419670974480958, "precision": 0.24350337809130138, "recall": 0.2405043345697236, "support": 6610}, "micro avg": {"f1-score": 0.9829046898638427, "precision": 0.9829046898638427, "recall": 0.9829046898638427, "support": 6610}, "weighted avg": {"f1-score": 0.9815257783754333, "precision": 0.9802541773774609, "recall": 0.9829046898638427, "support": 6610}, "\u2205": {"f1-score": 0.9903424056189639, "precision": 0.9861863962231159, "recall": 0.9945335919590901, "support": 5671}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u2423": {"f1-score": 0.9453943739658026, "precision": 0.9618406285072951, "recall": 0.9295010845986985, "support": 922}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1574}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 262}, "macro avg": {"f1-score": 0.16644796028942982, "precision": 0.24350337809130138, "recall": 0.13574539636628336, "support": 12933}, "micro avg": {"f1-score": 0.6648928004912245, "precision": 0.9829046898638427, "recall": 0.5023583082038197, "support": 12933}, "weighted avg": {"f1-score": 0.5861964939054849, "precision": 0.7596561619912611, "recall": 0.5023583082038197, "support": 12933}, "\u2205": {"f1-score": 0.8753686171038336, "precision": 0.9861863962231159, "recall": 0.7869401423189619, "support": 7167}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 627}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 152}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 147}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 138}, "\u2423": {"f1-score": 0.45621506521160504, "precision": 0.9618406285072951, "recall": 0.299023028611305, "support": 2866}},
  "ppcr": 0.5110956467950205
}
```
</details>
