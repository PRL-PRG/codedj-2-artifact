# Test report for javascript / file:///tmp/top-repos-quality-repos-oknrmilz/telescope.git HEAD e82daf606bf4ac64c445b5bd58f80784dd16f97a

### Classification report

PPCR: 0.787

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.939| 0.981| 0.885| 0.960| 0.911| 267| 296| 0.902 |
| `␣` | 0.971| 0.953| 0.706| 0.962| 0.818| 214| 289| 0.740 |
| `'` | 1.000| 0.962| 0.898| 0.981| 0.946| 183| 196| 0.934 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 1.000| 1.000| 1.000| 1.000| 1.000| 24| 24| 1.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 1.000| 1.000| 1.000| 1.000| 1.000| 24| 24| 1.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 28| 0.036 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 49| 0.000 |
| `weighted avg` | 0.967| 0.968| 0.762| 0.967| 0.816| 713| 906| 0.787 |
| `macro avg` | 0.701| 0.699| 0.641| 0.700| 0.668| 713| 906| 0.787 |
| `micro avg` | 0.968| 0.968| 0.762| 0.968| 0.852| 713| 906| 0.787 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|29 |262 |5 |0 |0 |0 |0 |0 |
|75 |10 |204 |0 |0 |0 |0 |0 |
|13 |7 |0 |176 |0 |0 |0 |0 |
|49 |0 |0 |0 |0 |0 |0 |0 |
|0 |0 |0 |0 |0 |24 |0 |0 |
|0 |0 |0 |0 |0 |0 |24 |0 |
|27 |0 |1 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.9805013927576601, "precision": 1.0, "recall": 0.9617486338797814, "support": 183}, "macro avg": {"f1-score": 0.7003532147725737, "precision": 0.7014995245409992, "recall": 0.6994704385938378, "support": 713}, "micro avg": {"f1-score": 0.967741935483871, "precision": 0.967741935483871, "recall": 0.967741935483871, "support": 713}, "weighted avg": {"f1-score": 0.9671781787072886, "precision": 0.9672046242376061, "recall": 0.967741935483871, "support": 713}, "\u2205": {"f1-score": 0.9597069597069596, "precision": 0.9390681003584229, "recall": 0.9812734082397003, "support": 267}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 24}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 24}, "\u2423": {"f1-score": 0.9622641509433962, "precision": 0.9714285714285714, "recall": 0.9532710280373832, "support": 214}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9462365591397849, "precision": 1.0, "recall": 0.8979591836734694, "support": 196}, "macro avg": {"f1-score": 0.6678823110724219, "precision": 0.7014995245409992, "recall": 0.6412823816785401, "support": 906}, "micro avg": {"f1-score": 0.8523780111179741, "precision": 0.967741935483871, "recall": 0.7615894039735099, "support": 906}, "weighted avg": {"f1-score": 0.8162307348060621, "precision": 0.8859900826147356, "recall": 0.7615894039735099, "support": 906}, "\u2205": {"f1-score": 0.9113043478260869, "precision": 0.9390681003584229, "recall": 0.8851351351351351, "support": 296}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 49}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 28}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 24}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 24}, "\u2423": {"f1-score": 0.8176352705410822, "precision": 0.9714285714285714, "recall": 0.7058823529411765, "support": 289}},
  "ppcr": 0.7869757174392936
}
```
</details>
