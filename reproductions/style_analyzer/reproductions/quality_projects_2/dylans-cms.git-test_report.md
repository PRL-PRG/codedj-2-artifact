# Test report for javascript / file:///tmp/top-repos-quality-repos-d5axd4fp/dylans-cms.git HEAD d71a957fde2255a444ee13e1adabd4b309fcbffe

### Classification report

PPCR: 0.587

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.948| 0.973| 0.805| 0.960| 0.870| 1131| 1368| 0.827 |
| `␣` | 0.775| 0.806| 0.242| 0.790| 0.369| 180| 599| 0.301 |
| `'` | 1.000| 1.000| 0.890| 1.000| 0.942| 153| 172| 0.890 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 17| 86| 0.198 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 12| 81| 0.148 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 140| 0.064 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 68| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 46| 0.000 |
| `weighted avg` | 0.908| 0.931| 0.546| 0.920| 0.615| 1502| 2560| 0.587 |
| `micro avg` | 0.931| 0.931| 0.546| 0.931| 0.689| 1502| 2560| 0.587 |
| `macro avg` | 0.340| 0.347| 0.242| 0.344| 0.273| 1502| 2560| 0.587 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| "| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|237 |1101 |30 |0 |0 |0 |0 |0 |0 |
|419 |35 |145 |0 |0 |0 |0 |0 |0 |
|19 |0 |0 |153 |0 |0 |0 |0 |0 |
|131 |7 |2 |0 |0 |0 |0 |0 |0 |
|69 |8 |9 |0 |0 |0 |0 |0 |0 |
|68 |0 |0 |0 |0 |0 |0 |0 |0 |
|69 |11 |1 |0 |0 |0 |0 |0 |0 |
|46 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 153}, "macro avg": {"f1-score": 0.3438130918528254, "precision": 0.34036317155558826, "recall": 0.34737879457707044, "support": 1502}, "micro avg": {"f1-score": 0.9314247669773634, "precision": 0.9314247669773635, "recall": 0.9314247669773635, "support": 1502}, "weighted avg": {"f1-score": 0.9196734124091792, "precision": 0.9082553655943164, "recall": 0.9314247669773635, "support": 1502}, "\u2205": {"f1-score": 0.96031399912778, "precision": 0.9475043029259896, "recall": 0.9734748010610079, "support": 1131}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 17}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u2423": {"f1-score": 0.7901907356948229, "precision": 0.7754010695187166, "recall": 0.8055555555555556, "support": 180}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 68}, "\u0027": {"f1-score": 0.9415384615384615, "precision": 1.0, "recall": 0.8895348837209303, "support": 172}, "macro avg": {"f1-score": 0.27260636697078783, "precision": 0.34036317155558826, "recall": 0.24205369524823434, "support": 2560}, "micro avg": {"f1-score": 0.688823239783358, "precision": 0.9314247669773635, "recall": 0.546484375, "support": 2560}, "weighted avg": {"f1-score": 0.6146860721716191, "precision": 0.7549418465017441, "recall": 0.546484375, "support": 2560}, "\u2205": {"f1-score": 0.8703557312252963, "precision": 0.9475043029259896, "recall": 0.8048245614035088, "support": 1368}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 140}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 46}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 86}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 81}, "\u2423": {"f1-score": 0.36895674300254455, "precision": 0.7754010695187166, "recall": 0.24207011686143573, "support": 599}},
  "ppcr": 0.58671875
}
```
</details>
