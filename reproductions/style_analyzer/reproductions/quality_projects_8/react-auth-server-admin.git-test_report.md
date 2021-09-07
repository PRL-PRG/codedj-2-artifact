# Test report for javascript / file:///tmp/top-repos-quality-repos-ndxw65pu/react-auth-server-admin.git HEAD 703a07fe863fb0eadad6137990df5bf34b5779cb

### Classification report

PPCR: 0.658

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.785| 0.989| 0.767| 0.875| 0.776| 269| 347| 0.775 |
| `␣` | 0.628| 0.462| 0.290| 0.533| 0.397| 106| 169| 0.627 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 23| 43| 0.535 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 10| 0.900 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 42| 0.190 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 18| 0.111 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 5| 0.000 |
| `weighted avg` | 0.666| 0.755| 0.497| 0.700| 0.530| 417| 634| 0.658 |
| `micro avg` | 0.755| 0.755| 0.497| 0.755| 0.599| 417| 634| 0.658 |
| `macro avg` | 0.177| 0.181| 0.132| 0.176| 0.147| 417| 634| 0.658 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|78 |266 |3 |0 |0 |0 |0 |0 |
|63 |57 |49 |0 |0 |0 |0 |0 |
|34 |0 |8 |0 |0 |0 |0 |0 |
|20 |7 |16 |0 |0 |0 |0 |0 |
|5 |0 |0 |0 |0 |0 |0 |0 |
|1 |9 |0 |0 |0 |0 |0 |0 |
|16 |0 |2 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "macro avg": {"f1-score": 0.17595108695652173, "precision": 0.1766082368958475, "recall": 0.18138896682331485, "support": 417}, "micro avg": {"f1-score": 0.7553956834532374, "precision": 0.7553956834532374, "recall": 0.7553956834532374, "support": 417}, "weighted avg": {"f1-score": 0.6998357835470754, "precision": 0.6658596880154147, "recall": 0.7553956834532374, "support": 417}, "\u2205": {"f1-score": 0.875, "precision": 0.7846607669616519, "recall": 0.9888475836431226, "support": 269}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 23}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u2423": {"f1-score": 0.532608695652174, "precision": 0.6282051282051282, "recall": 0.46226415094339623, "support": 106}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 42}, "macro avg": {"f1-score": 0.14653391721060893, "precision": 0.1766082368958475, "recall": 0.13206392919871085, "support": 634}, "micro avg": {"f1-score": 0.5994291151284491, "precision": 0.7553956834532374, "recall": 0.4968454258675079, "support": 634}, "weighted avg": {"f1-score": 0.5302124170272459, "precision": 0.5969147520541954, "recall": 0.4968454258675079, "support": 634}, "\u2205": {"f1-score": 0.7755102040816327, "precision": 0.7846607669616519, "recall": 0.7665706051873199, "support": 347}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 43}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 18}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u2423": {"f1-score": 0.39676113360323884, "precision": 0.6282051282051282, "recall": 0.28994082840236685, "support": 169}},
  "ppcr": 0.6577287066246057
}
```
</details>
