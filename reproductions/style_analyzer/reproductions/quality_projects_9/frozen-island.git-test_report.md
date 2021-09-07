# Test report for javascript / file:///tmp/top-repos-quality-repos-ru2mv3a9/frozen-island.git HEAD f227b26797ba5667def2ad30a8b84359967d9564

### Classification report

PPCR: 0.692

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.852| 0.990| 0.874| 0.916| 0.863| 496| 562| 0.883 |
| `␣` | 0.965| 0.838| 0.583| 0.897| 0.727| 197| 283| 0.696 |
| `'` | 0.931| 0.931| 0.466| 0.931| 0.621| 58| 116| 0.500 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 22| 36| 0.611 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 16| 95| 0.168 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 38| 0.184 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 8| 0.625 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 26| 0.154 |
| `weighted avg` | 0.828| 0.882| 0.610| 0.851| 0.655| 805| 1164| 0.692 |
| `macro avg` | 0.344| 0.345| 0.240| 0.343| 0.276| 805| 1164| 0.692 |
| `micro avg` | 0.882| 0.882| 0.610| 0.882| 0.721| 805| 1164| 0.692 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|66 |491 |5 |0 |0 |0 |0 |0 |0 |
|86 |32 |165 |0 |0 |0 |0 |0 |0 |
|58 |4 |0 |54 |0 |0 |0 |0 |0 |
|79 |16 |0 |0 |0 |0 |0 |0 |0 |
|31 |6 |1 |0 |0 |0 |0 |0 |0 |
|14 |22 |0 |0 |0 |0 |0 |0 |0 |
|22 |4 |0 |0 |0 |0 |0 |0 |0 |
|3 |1 |0 |4 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u0027": {"f1-score": 0.9310344827586207, "precision": 0.9310344827586207, "recall": 0.9310344827586207, "support": 58}, "macro avg": {"f1-score": 0.34297729866410076, "precision": 0.34354716487699133, "recall": 0.34481466117174747, "support": 805}, "micro avg": {"f1-score": 0.8819875776397516, "precision": 0.8819875776397516, "recall": 0.8819875776397516, "support": 805}, "weighted avg": {"f1-score": 0.8509513262743801, "precision": 0.8284388507500635, "recall": 0.8819875776397516, "support": 805}, "\u2205": {"f1-score": 0.9160447761194029, "precision": 0.8524305555555556, "recall": 0.9899193548387096, "support": 496}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u2423": {"f1-score": 0.8967391304347827, "precision": 0.9649122807017544, "recall": 0.8375634517766497, "support": 197}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u0027": {"f1-score": 0.6206896551724138, "precision": 0.9310344827586207, "recall": 0.46551724137931033, "support": 116}, "macro avg": {"f1-score": 0.2763099126017459, "precision": 0.34354716487699133, "recall": 0.24027769888303838, "support": 1164}, "micro avg": {"f1-score": 0.7211782630777044, "precision": 0.8819875776397516, "recall": 0.6099656357388317, "support": 1164}, "weighted avg": {"f1-score": 0.6552099862735046, "precision": 0.738948580464621, "recall": 0.6099656357388317, "support": 1164}, "\u2205": {"f1-score": 0.8629173989455184, "precision": 0.8524305555555556, "recall": 0.8736654804270463, "support": 562}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 95}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 26}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 38}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 36}, "\u2423": {"f1-score": 0.7268722466960353, "precision": 0.9649122807017544, "recall": 0.5830388692579506, "support": 283}},
  "ppcr": 0.6915807560137457
}
```
</details>
