# Test report for javascript / file:///tmp/top-repos-quality-repos-nxt_vcuv/forum-app-project2.git HEAD e4a79e5ed9de3fff6485ce855892e5007678e3a0

### Classification report

PPCR: 0.766

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.830| 0.993| 0.909| 0.904| 0.868| 732| 800| 0.915 |
| `␣` | 0.935| 0.453| 0.281| 0.610| 0.432| 190| 306| 0.621 |
| `'` | 0.864| 1.000| 0.731| 0.927| 0.792| 38| 52| 0.731 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 28| 72| 0.389 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 17| 43| 0.395 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 36| 0.111 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 12| 0.250 |
| `micro avg` | 0.841| 0.841| 0.644| 0.841| 0.730| 1012| 1321| 0.766 |
| `weighted avg` | 0.808| 0.841| 0.644| 0.803| 0.657| 1012| 1321| 0.766 |
| `macro avg` | 0.375| 0.349| 0.274| 0.349| 0.299| 1012| 1321| 0.766 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|68 |727 |5 |0 |0 |0 |0 |0 |
|116 |98 |86 |6 |0 |0 |0 |0 |
|14 |0 |0 |38 |0 |0 |0 |0 |
|44 |28 |0 |0 |0 |0 |0 |0 |
|26 |16 |1 |0 |0 |0 |0 |0 |
|32 |4 |0 |0 |0 |0 |0 |0 |
|9 |3 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.9268292682926829, "precision": 0.8636363636363636, "recall": 1.0, "support": 38}, "macro avg": {"f1-score": 0.3487124574326086, "precision": 0.3754753783044432, "recall": 0.34940013969349604, "support": 1012}, "micro avg": {"f1-score": 0.8409090909090909, "precision": 0.8409090909090909, "recall": 0.8409090909090909, "support": 1012}, "weighted avg": {"f1-score": 0.8033612247093643, "precision": 0.8082213716949479, "recall": 0.8409090909090909, "support": 1012}, "\u2205": {"f1-score": 0.904228855721393, "precision": 0.8299086757990868, "recall": 0.9931693989071039, "support": 732}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 28}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 17}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u2423": {"f1-score": 0.6099290780141844, "precision": 0.9347826086956522, "recall": 0.45263157894736844, "support": 190}},
  "cl_report_full": {"\u0027": {"f1-score": 0.7916666666666666, "precision": 0.8636363636363636, "recall": 0.7307692307692307, "support": 52}, "macro avg": {"f1-score": 0.2987670338280789, "precision": 0.3754753783044432, "recall": 0.27436642605760253, "support": 1321}, "micro avg": {"f1-score": 0.7295327903986285, "precision": 0.8409090909090909, "recall": 0.6442089326267979, "support": 1321}, "weighted avg": {"f1-score": 0.6566550231526461, "precision": 0.753126048303732, "recall": 0.6442089326267979, "support": 1321}, "\u2205": {"f1-score": 0.8675417661097852, "precision": 0.8299086757990868, "recall": 0.90875, "support": 800}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 72}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 43}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 36}, "\u2423": {"f1-score": 0.43216080402010054, "precision": 0.9347826086956522, "recall": 0.28104575163398693, "support": 306}},
  "ppcr": 0.7660862982588947
}
```
</details>
