# Train report for javascript / file:///tmp/top-repos-quality-repos-mi98bflo/stockmarketnews.git HEAD cb31825adf600fdc5810932601f2e2113bb915c7

### Classification report

PPCR: 0.205

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.972| 1.000| 0.338| 0.986| 0.501| 210| 622| 0.338 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 260| 0.023 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 62| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 77| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 33| 0.000 |
| `micro avg` | 0.972| 0.972| 0.199| 0.972| 0.331| 216| 1054| 0.205 |
| `weighted avg` | 0.945| 0.972| 0.199| 0.959| 0.296| 216| 1054| 0.205 |
| `macro avg` | 0.194| 0.200| 0.068| 0.197| 0.100| 216| 1054| 0.205 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|412 |210 |0 |0 |0 |0 |
|254 |6 |0 |0 |0 |0 |
|62 |0 |0 |0 |0 |0 |
|77 |0 |0 |0 |0 |0 |
|33 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/pages/index.js | 3 |
| src/components/header.js | 1 |
| src/pages/404.js | 1 |
| src/pages/page-2.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.19718309859154928, "precision": 0.19444444444444445, "recall": 0.2, "support": 216}, "micro avg": {"f1-score": 0.9722222222222222, "precision": 0.9722222222222222, "recall": 0.9722222222222222, "support": 216}, "weighted avg": {"f1-score": 0.9585289514866979, "precision": 0.945216049382716, "recall": 0.9722222222222222, "support": 216}, "\u2205": {"f1-score": 0.9859154929577464, "precision": 0.9722222222222222, "recall": 1.0, "support": 210}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 62}, "macro avg": {"f1-score": 0.10023866348448687, "precision": 0.19444444444444445, "recall": 0.06752411575562701, "support": 1054}, "micro avg": {"f1-score": 0.33070866141732286, "precision": 0.9722222222222222, "recall": 0.19924098671726756, "support": 1054}, "weighted avg": {"f1-score": 0.29577062944663585, "precision": 0.573740248787687, "recall": 0.19924098671726756, "support": 1054}, "\u2205": {"f1-score": 0.5011933174224343, "precision": 0.9722222222222222, "recall": 0.33762057877813506, "support": 622}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 77}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 33}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 260}},
  "ppcr": 0.2049335863377609
}
```
</details>
