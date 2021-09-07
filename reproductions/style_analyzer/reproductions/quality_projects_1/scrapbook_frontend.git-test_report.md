# Test report for javascript / file:///tmp/top-repos-quality-repos-vheepge6/scrapbook_frontend.git HEAD 79cc2b7cdd1bbed3ea47fc15af0bbeea04590f55

### Classification report

PPCR: 0.910

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `␣` | 0.900| 0.679| 0.667| 0.774| 0.766| 53| 54| 0.981 |
| `∅` | 0.507| 0.947| 0.923| 0.661| 0.655| 38| 39| 0.974 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 11| 11| 1.000 |
| `'` | 1.000| 0.900| 0.450| 0.947| 0.621| 10| 20| 0.500 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 4| 1.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 3| 1.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 3| 1.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `␣⇥` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `␣⇥⇥` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `␣⇥⇥⇥` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `␣⇥⇥⇥⇥` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `␣⇥⇥⇥⇥⇥` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `macro avg` | 0.160| 0.168| 0.136| 0.159| 0.136| 122| 134| 0.910 |
| `micro avg` | 0.664| 0.664| 0.604| 0.664| 0.633| 122| 134| 0.910 |
| `weighted avg` | 0.631| 0.664| 0.604| 0.620| 0.592| 122| 134| 0.910 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| '| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ␣⇥⇥⇥| ⏎␣⁻␣⁻| ␣⇥⇥| ⏎␣⁺␣⁺| ␣⇥⇥⇥⇥| ␣⇥| ⏎⏎| ␣⇥⇥⇥⇥⇥| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1 |36 |2 |0 |0 |0 |0 |0 |0 |
|1 |16 |36 |1 |0 |0 |0 |0 |0 |
|0 |10 |1 |0 |0 |0 |0 |0 |0 |
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|10 |0 |0 |0 |1 |9 |0 |0 |0 |
|0 |4 |0 |0 |0 |0 |0 |0 |0 |
|0 |2 |1 |0 |0 |0 |0 |0 |0 |
|0 |3 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.9473684210526316, "precision": 1.0, "recall": 0.9, "support": 10}, "macro avg": {"f1-score": 0.15880749521035498, "precision": 0.16046948356807514, "recall": 0.16844091360476662, "support": 122}, "micro avg": {"f1-score": 0.6639344262295082, "precision": 0.6639344262295082, "recall": 0.6639344262295082, "support": 122}, "weighted avg": {"f1-score": 0.6197283582478287, "precision": 0.6308820133918264, "recall": 0.6639344262295082, "support": 122}, "\u2205": {"f1-score": 0.6605504587155963, "precision": 0.5070422535211268, "recall": 0.9473684210526315, "support": 38}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u2423": {"f1-score": 0.7741935483870968, "precision": 0.9, "recall": 0.6792452830188679, "support": 53}, "\u2423\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423\u21e5\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423\u21e5\u21e5\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423\u21e5\u21e5\u21e5\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423\u21e5\u21e5\u21e5\u21e5\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.6206896551724138, "precision": 1.0, "recall": 0.45, "support": 20}, "macro avg": {"f1-score": 0.13607950376842526, "precision": 0.16046948356807514, "recall": 0.135982905982906, "support": 134}, "micro avg": {"f1-score": 0.6328125, "precision": 0.6639344262295082, "recall": 0.6044776119402985, "support": 134}, "weighted avg": {"f1-score": 0.5918117011819446, "precision": 0.6595122976665966, "recall": 0.6044776119402985, "support": 134}, "\u2205": {"f1-score": 0.6545454545454545, "precision": 0.5070422535211268, "recall": 0.9230769230769231, "support": 39}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u2423": {"f1-score": 0.7659574468085106, "precision": 0.9, "recall": 0.6666666666666666, "support": 54}, "\u2423\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423\u21e5\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423\u21e5\u21e5\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423\u21e5\u21e5\u21e5\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423\u21e5\u21e5\u21e5\u21e5\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "ppcr": 0.9104477611940298
}
```
</details>
