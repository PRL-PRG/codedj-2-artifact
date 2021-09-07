# Test report for javascript / file:///tmp/top-repos-quality-repos-kquqhe21/skynet.git HEAD 4c1123d693048023ee6c8349e064d74f687c2afe

### Classification report

PPCR: 0.847

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.969| 0.960| 0.888| 0.964| 0.927| 99| 107| 0.925 |
| `␣` | 0.854| 0.953| 0.872| 0.901| 0.863| 43| 47| 0.915 |
| `'` | 1.000| 1.000| 0.833| 1.000| 0.909| 15| 18| 0.833 |
| `⏎␣⁺␣⁺` | 0.875| 0.875| 0.875| 0.875| 0.875| 8| 8| 1.000 |
| `⏎␣⁻␣⁻` | 0.875| 0.875| 0.875| 0.875| 0.875| 8| 8| 1.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 15| 0.267 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 6| 0.000 |
| `weighted avg` | 0.914| 0.932| 0.789| 0.922| 0.814| 177| 209| 0.847 |
| `micro avg` | 0.932| 0.932| 0.789| 0.932| 0.855| 177| 209| 0.847 |
| `macro avg` | 0.653| 0.666| 0.621| 0.659| 0.636| 177| 209| 0.847 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|8 |95 |4 |0 |0 |0 |0 |0 |
|4 |0 |41 |0 |0 |1 |1 |0 |
|3 |0 |0 |15 |0 |0 |0 |0 |
|11 |1 |3 |0 |0 |0 |0 |0 |
|0 |1 |0 |0 |0 |7 |0 |0 |
|0 |1 |0 |0 |0 |0 |7 |0 |
|6 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 15}, "macro avg": {"f1-score": 0.6593665580250061, "precision": 0.6533649173955297, "recall": 0.6661549045269977, "support": 177}, "micro avg": {"f1-score": 0.9322033898305084, "precision": 0.9322033898305084, "recall": 0.9322033898305084, "support": 177}, "weighted avg": {"f1-score": 0.9222004872869537, "precision": 0.913551154925247, "recall": 0.9322033898305084, "support": 177}, "\u2205": {"f1-score": 0.964467005076142, "precision": 0.9693877551020408, "recall": 0.9595959595959596, "support": 99}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.875, "precision": 0.875, "recall": 0.875, "support": 8}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.875, "precision": 0.875, "recall": 0.875, "support": 8}, "\u2423": {"f1-score": 0.9010989010989011, "precision": 0.8541666666666666, "recall": 0.9534883720930233, "support": 43}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9090909090909091, "precision": 1.0, "recall": 0.8333333333333334, "support": 18}, "macro avg": {"f1-score": 0.6355825817314906, "precision": 0.6533649173955297, "recall": 0.6205034608792811, "support": 209}, "micro avg": {"f1-score": 0.8549222797927462, "precision": 0.9322033898305084, "recall": 0.7894736842105263, "support": 209}, "weighted avg": {"f1-score": 0.8138889431750479, "precision": 0.8414847996614914, "recall": 0.7894736842105263, "support": 209}, "\u2205": {"f1-score": 0.926829268292683, "precision": 0.9693877551020408, "recall": 0.8878504672897196, "support": 107}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 15}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.875, "precision": 0.875, "recall": 0.875, "support": 8}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.875, "precision": 0.875, "recall": 0.875, "support": 8}, "\u2423": {"f1-score": 0.8631578947368421, "precision": 0.8541666666666666, "recall": 0.8723404255319149, "support": 47}},
  "ppcr": 0.84688995215311
}
```
</details>
