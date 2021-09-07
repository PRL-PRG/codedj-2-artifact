# Train report for javascript / file:///tmp/top-repos-quality-repos-qpbpda06/trust.git HEAD 38a9e1976cc5284eef67673e9d9cdf0ca1711c80

### Classification report

PPCR: 0.658

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.946| 1.000| 0.945| 0.972| 0.946| 5845| 6187| 0.945 |
| `␣` | 1.000| 0.689| 0.247| 0.816| 0.396| 541| 1511| 0.358 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 83| 462| 0.180 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 39| 121| 0.322 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 37| 156| 0.237 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 154| 0.026 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1020| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 345| 0.000 |
| `weighted avg` | 0.927| 0.949| 0.625| 0.935| 0.648| 6549| 9956| 0.658 |
| `micro avg` | 0.949| 0.949| 0.625| 0.949| 0.753| 6549| 9956| 0.658 |
| `macro avg` | 0.243| 0.211| 0.149| 0.224| 0.168| 6549| 9956| 0.658 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| '| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|342 |5845 |0 |0 |0 |0 |0 |0 |0 |
|970 |168 |373 |0 |0 |0 |0 |0 |0 |
|1020 |0 |0 |0 |0 |0 |0 |0 |0 |
|379 |83 |0 |0 |0 |0 |0 |0 |0 |
|345 |0 |0 |0 |0 |0 |0 |0 |0 |
|119 |37 |0 |0 |0 |0 |0 |0 |0 |
|150 |4 |0 |0 |0 |0 |0 |0 |0 |
|82 |39 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| js/student.js | 135 |
| server.js | 119 |
| js/teacher.js | 68 |
| js/quickPlay.js | 9 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.22358217666858343, "precision": 0.2433006800518135, "recall": 0.2111829944547135, "support": 6549}, "micro avg": {"f1-score": 0.94945793250878, "precision": 0.94945793250878, "recall": 0.94945793250878, "support": 6549}, "weighted avg": {"f1-score": 0.9353515409764042, "precision": 0.9272774162807754, "recall": 0.94945793250878, "support": 6549}, "\u2205": {"f1-score": 0.9724648531736128, "precision": 0.9464054404145078, "recall": 1.0, "support": 5845}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 83}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 39}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 37}, "\u2423": {"f1-score": 0.8161925601750546, "precision": 1.0, "recall": 0.6894639556377079, "support": 541}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1020}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 345}, "macro avg": {"f1-score": 0.16769117553868101, "precision": 0.2433006800518135, "recall": 0.14894739904778886, "support": 9956}, "micro avg": {"f1-score": 0.7534686458648894, "precision": 0.94945793250878, "recall": 0.6245480112494978, "support": 9956}, "weighted avg": {"f1-score": 0.6477004087457986, "precision": 0.7398965909847891, "recall": 0.6245480112494978, "support": 9956}, "\u2205": {"f1-score": 0.9455633745854566, "precision": 0.9464054404145078, "recall": 0.9447228058833037, "support": 6187}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 462}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 121}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 154}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 156}, "\u2423": {"f1-score": 0.39596602972399153, "precision": 1.0, "recall": 0.2468563864990073, "support": 1511}},
  "ppcr": 0.6577942948975493
}
```
</details>
