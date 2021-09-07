# Test report for javascript / file:///tmp/top-repos-quality-repos-mhpdq1pj/dubdub-sites.git HEAD f86014e831aec5000fe943db5b48cfde8fcbff53

### Classification report

PPCR: 0.664

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.959| 0.961| 0.770| 0.960| 0.854| 1105| 1380| 0.801 |
| `␣` | 0.802| 0.968| 0.603| 0.877| 0.689| 377| 605| 0.623 |
| `'` | 1.000| 0.905| 0.890| 0.950| 0.942| 169| 172| 0.983 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 32| 88| 0.364 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 16| 141| 0.113 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 16| 83| 0.193 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 68| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 46| 0.000 |
| `macro avg` | 0.345| 0.354| 0.283| 0.348| 0.311| 1715| 2583| 0.664 |
| `weighted avg` | 0.893| 0.921| 0.612| 0.905| 0.680| 1715| 2583| 0.664 |
| `micro avg` | 0.921| 0.921| 0.612| 0.921| 0.735| 1715| 2583| 0.664 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| "| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|275 |1062 |43 |0 |0 |0 |0 |0 |0 |
|228 |12 |365 |0 |0 |0 |0 |0 |0 |
|3 |0 |16 |153 |0 |0 |0 |0 |0 |
|125 |1 |15 |0 |0 |0 |0 |0 |0 |
|56 |17 |15 |0 |0 |0 |0 |0 |0 |
|68 |0 |0 |0 |0 |0 |0 |0 |0 |
|67 |15 |1 |0 |0 |0 |0 |0 |0 |
|46 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.9503105590062112, "precision": 1.0, "recall": 0.9053254437869822, "support": 169}, "macro avg": {"f1-score": 0.3484914254189674, "precision": 0.34519342446171714, "recall": 0.3543226472388588, "support": 1715}, "micro avg": {"f1-score": 0.9212827988338192, "precision": 0.9212827988338192, "recall": 0.9212827988338192, "support": 1715}, "weighted avg": {"f1-score": 0.9052032171859229, "precision": 0.8930086718609793, "recall": 0.9212827988338192, "support": 1715}, "\u2205": {"f1-score": 0.9602169981916817, "precision": 0.959349593495935, "recall": 0.9610859728506788, "support": 1105}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 32}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u2423": {"f1-score": 0.8774038461538461, "precision": 0.8021978021978022, "recall": 0.9681697612732095, "support": 377}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 68}, "\u0027": {"f1-score": 0.9415384615384615, "precision": 1.0, "recall": 0.8895348837209303, "support": 172}, "macro avg": {"f1-score": 0.3105323400113099, "precision": 0.34519342446171714, "recall": 0.2828007357795252, "support": 2583}, "micro avg": {"f1-score": 0.7352256863657515, "precision": 0.9212827988338192, "recall": 0.6116918312040264, "support": 2583}, "weighted avg": {"f1-score": 0.6802834522230199, "precision": 0.7670275297537982, "recall": 0.6116918312040264, "support": 2583}, "\u2205": {"f1-score": 0.8540410132689988, "precision": 0.959349593495935, "recall": 0.7695652173913043, "support": 1380}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 141}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 46}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 88}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 83}, "\u2423": {"f1-score": 0.6886792452830189, "precision": 0.8021978021978022, "recall": 0.6033057851239669, "support": 605}},
  "ppcr": 0.6639566395663956
}
```
</details>
