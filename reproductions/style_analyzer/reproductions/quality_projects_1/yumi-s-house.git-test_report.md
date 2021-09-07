# Test report for javascript / file:///tmp/top-repos-quality-repos-n68j2a10/yumi-s-house.git HEAD ed27ccb20ee1d10d35ed0264d74445b43188343c

### Classification report

PPCR: 0.268

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.322| 1.000| 0.487| 130| 404| 0.322 |
| `'` | 1.000| 1.000| 0.500| 1.000| 0.667| 58| 116| 0.500 |
| `␣` | 1.000| 1.000| 0.184| 1.000| 0.311| 39| 212| 0.184 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 48| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 26| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 21| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 21| 0.000 |
| `macro avg` | 0.429| 0.429| 0.144| 0.429| 0.209| 227| 848| 0.268 |
| `micro avg` | 1.000| 1.000| 0.268| 1.000| 0.422| 227| 848| 0.268 |
| `weighted avg` | 1.000| 1.000| 0.268| 1.000| 0.401| 227| 848| 0.268 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|274 |130 |0 |0 |0 |0 |0 |0 |
|173 |0 |39 |0 |0 |0 |0 |0 |
|58 |0 |0 |58 |0 |0 |0 |0 |
|48 |0 |0 |0 |0 |0 |0 |0 |
|21 |0 |0 |0 |0 |0 |0 |0 |
|21 |0 |0 |0 |0 |0 |0 |0 |
|26 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 58}, "macro avg": {"f1-score": 0.42857142857142855, "precision": 0.42857142857142855, "recall": 0.42857142857142855, "support": 227}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 227}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 227}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 130}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 39}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 116}, "macro avg": {"f1-score": 0.2091878606494301, "precision": 0.42857142857142855, "recall": 0.14367777748125216, "support": 848}, "micro avg": {"f1-score": 0.4223255813953488, "precision": 1.0, "recall": 0.267688679245283, "support": 848}, "weighted avg": {"f1-score": 0.4008466170650589, "precision": 0.8632075471698113, "recall": 0.267688679245283, "support": 848}, "\u2205": {"f1-score": 0.48689138576779023, "precision": 1.0, "recall": 0.3217821782178218, "support": 404}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 48}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 26}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 21}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 21}, "\u2423": {"f1-score": 0.3107569721115538, "precision": 1.0, "recall": 0.18396226415094338, "support": 212}},
  "ppcr": 0.267688679245283
}
```
</details>
