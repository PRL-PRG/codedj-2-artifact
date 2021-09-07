# Train report for javascript / file:///tmp/top-repos-quality-repos-0z55eys2/jirascope.git HEAD 657724f9c2556e89f886483e51e27277cbb17a64

### Classification report

PPCR: 0.536

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.993| 1.000| 0.862| 0.997| 0.923| 5574| 6462| 0.863 |
| `⏎␣⁺␣⁺` | 0.951| 0.965| 0.812| 0.958| 0.876| 260| 309| 0.841 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 32| 2517| 0.013 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 314| 0.025 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 556| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 545| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 247| 0.000 |
| `micro avg` | 0.991| 0.991| 0.532| 0.991| 0.692| 5874| 10950| 0.536 |
| `weighted avg` | 0.985| 0.991| 0.532| 0.988| 0.570| 5874| 10950| 0.536 |
| `macro avg` | 0.278| 0.281| 0.239| 0.279| 0.257| 5874| 10950| 0.536 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|888 |5573 |0 |0 |0 |0 |1 |0 |
|2485 |23 |0 |0 |0 |0 |9 |0 |
|556 |0 |0 |0 |0 |0 |0 |0 |
|545 |0 |0 |0 |0 |0 |0 |0 |
|306 |5 |0 |0 |0 |0 |3 |0 |
|49 |9 |0 |0 |0 |0 |251 |0 |
|247 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| lib/index.js | 17 |
| lib/cli/dot.js | 7 |
| lib/cli/dots.js | 7 |
| lib/cli/index.js | 6 |
| lib/common/jiraClient.js | 4 |
| lib/cli/highest.js | 2 |
| lib/cli/warnings.js | 2 |
| lib/cli/roots.js | 2 |
| lib/cli/trackers.js | 2 |
| lib/common/fileDataStore.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.27923107945140513, "precision": 0.2777374586198116, "recall": 0.2807436015724498, "support": 5874}, "micro avg": {"f1-score": 0.9914879128362274, "precision": 0.9914879128362274, "recall": 0.9914879128362274, "support": 5874}, "weighted avg": {"f1-score": 0.9881077848593689, "precision": 0.9847521966040262, "recall": 0.9914879128362274, "support": 5874}, "\u2205": {"f1-score": 0.9966022889842633, "precision": 0.9934046345811052, "recall": 0.9998205956225332, "support": 5574}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9580152671755725, "precision": 0.9507575757575758, "recall": 0.9653846153846154, "support": 260}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 32}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 556}, "macro avg": {"f1-score": 0.2570549031911745, "precision": 0.2777374586198116, "recall": 0.23924631828193504, "support": 10950}, "micro avg": {"f1-score": 0.692344270090347, "precision": 0.9914879128362274, "recall": 0.5318721461187215, "support": 10950}, "weighted avg": {"f1-score": 0.5695922468963502, "precision": 0.613074414572803, "recall": 0.5318721461187215, "support": 10950}, "\u2205": {"f1-score": 0.9232935719019217, "precision": 0.9934046345811052, "recall": 0.8624264933457134, "support": 6462}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 545}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 247}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8760907504363001, "precision": 0.9507575757575758, "recall": 0.8122977346278317, "support": 309}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 314}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2517}},
  "ppcr": 0.5364383561643835
}
```
</details>
