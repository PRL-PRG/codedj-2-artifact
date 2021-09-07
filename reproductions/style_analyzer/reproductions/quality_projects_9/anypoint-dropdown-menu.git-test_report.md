# Test report for javascript / file:///tmp/top-repos-quality-repos-f0r11jdy/anypoint-dropdown-menu.git HEAD e02e5909d6b3a21d0b93174448da9b7a4cbc63ce

### Classification report

PPCR: 0.867

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `␣` | 0.600| 0.600| 0.500| 0.600| 0.545| 15| 18| 0.833 |
| `∅` | 0.727| 0.571| 0.571| 0.640| 0.640| 14| 14| 1.000 |
| `'` | 1.000| 1.000| 0.625| 1.000| 0.769| 5| 8| 0.625 |
| `⏎` | 0.500| 0.667| 0.667| 0.571| 0.571| 3| 3| 1.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 2| 1.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `weighted avg` | 0.659| 0.615| 0.533| 0.633| 0.592| 39| 45| 0.867 |
| `macro avg` | 0.404| 0.405| 0.338| 0.402| 0.361| 39| 45| 0.867 |
| `micro avg` | 0.615| 0.615| 0.533| 0.615| 0.571| 39| 45| 0.867 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|0 |8 |6 |0 |0 |0 |0 |
|3 |3 |9 |0 |0 |3 |0 |
|3 |0 |0 |5 |0 |0 |0 |
|0 |0 |0 |0 |2 |0 |1 |
|0 |0 |0 |0 |0 |0 |0 |
|0 |0 |0 |0 |2 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 5}, "macro avg": {"f1-score": 0.40163265306122453, "precision": 0.40389610389610386, "recall": 0.40544217687074824, "support": 39}, "micro avg": {"f1-score": 0.6153846153846154, "precision": 0.6153846153846154, "recall": 0.6153846153846154, "support": 39}, "weighted avg": {"f1-score": 0.6326739926739927, "precision": 0.6585081585081585, "recall": 0.6153846153846154, "support": 39}, "\u2205": {"f1-score": 0.64, "precision": 0.7272727272727273, "recall": 0.5714285714285714, "support": 14}, "\u23ce": {"f1-score": 0.5714285714285715, "precision": 0.5, "recall": 0.6666666666666666, "support": 3}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.6, "precision": 0.6, "recall": 0.6, "support": 15}},
  "cl_report_full": {"\u0027": {"f1-score": 0.7692307692307693, "precision": 1.0, "recall": 0.625, "support": 8}, "macro avg": {"f1-score": 0.3608734123019838, "precision": 0.40389610389610386, "recall": 0.33758503401360546, "support": 45}, "micro avg": {"f1-score": 0.5714285714285715, "precision": 0.6153846153846154, "recall": 0.5333333333333333, "support": 45}, "weighted avg": {"f1-score": 0.5921403041403042, "precision": 0.6773737373737373, "recall": 0.5333333333333333, "support": 45}, "\u2205": {"f1-score": 0.64, "precision": 0.7272727272727273, "recall": 0.5714285714285714, "support": 14}, "\u23ce": {"f1-score": 0.5714285714285715, "precision": 0.5, "recall": 0.6666666666666666, "support": 3}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.5454545454545454, "precision": 0.6, "recall": 0.5, "support": 18}},
  "ppcr": 0.8666666666666667
}
```
</details>
