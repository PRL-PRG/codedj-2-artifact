# Train report for javascript / file:///tmp/top-repos-quality-repos-jffnnmlo/itmo-labs-1.git HEAD 95de8db54e9f5b3543d04c4b86027c9149d1f0a5

### Classification report

PPCR: 0.245

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.942| 0.994| 0.311| 0.967| 0.467| 1491| 4767| 0.313 |
| `␣` | 0.984| 1.000| 0.248| 0.992| 0.396| 619| 2501| 0.248 |
| `⏎` | 1.000| 0.786| 0.256| 0.880| 0.408| 210| 644| 0.326 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 27| 168| 0.161 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 15| 162| 0.093 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 78| 0.051 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 88| 0.023 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 856| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 394| 0.000 |
| `macro avg` | 0.325| 0.309| 0.091| 0.315| 0.141| 2368| 9658| 0.245 |
| `micro avg` | 0.957| 0.957| 0.235| 0.957| 0.377| 2368| 9658| 0.245 |
| `weighted avg` | 0.939| 0.957| 0.235| 0.946| 0.360| 2368| 9658| 0.245 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| '| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3276 |1482 |9 |0 |0 |0 |0 |0 |0 |0 |
|1882 |0 |619 |0 |0 |0 |0 |0 |0 |0 |
|856 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|434 |44 |1 |0 |165 |0 |0 |0 |0 |0 |
|394 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|141 |27 |0 |0 |0 |0 |0 |0 |0 |0 |
|147 |15 |0 |0 |0 |0 |0 |0 |0 |0 |
|86 |2 |0 |0 |0 |0 |0 |0 |0 |0 |
|74 |4 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/body.js | 37 |
| src/task.js | 17 |
| frmw-001/src/index.js | 15 |
| rxjs-task/works.js | 12 |
| rxjs-task/fetch-api.js | 5 |
| async-task/index.js | 4 |
| rxjs-task/index.js | 4 |
| evnt-001/index.js | 2 |
| func-008/index.js | 2 |
| webpack.config.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.3154482764229909, "precision": 0.32507243771164845, "recall": 0.3088531187122736, "support": 2368}, "micro avg": {"f1-score": 0.9569256756756757, "precision": 0.9569256756756757, "recall": 0.9569256756756757, "support": 2368}, "weighted avg": {"f1-score": 0.946244763854432, "precision": 0.9387712486031908, "recall": 0.9569256756756757, "support": 2368}, "\u2205": {"f1-score": 0.9670473083197388, "precision": 0.9415501905972046, "recall": 0.993963782696177, "support": 1491}, "\u23ce": {"f1-score": 0.88, "precision": 1.0, "recall": 0.7857142857142857, "support": 210}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 15}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 27}, "\u2423": {"f1-score": 0.9919871794871795, "precision": 0.9841017488076311, "recall": 1.0, "support": 619}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 856}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 394}, "macro avg": {"f1-score": 0.14120803515951888, "precision": 0.32507243771164845, "recall": 0.09051105891770124, "support": 9658}, "micro avg": {"f1-score": 0.3768501579910194, "precision": 0.9569256756756757, "recall": 0.23462414578587698, "support": 9658}, "weighted avg": {"f1-score": 0.3603403124468014, "precision": 0.7862505935333153, "recall": 0.23462414578587698, "support": 9658}, "\u2205": {"f1-score": 0.4674341586500551, "precision": 0.9415501905972046, "recall": 0.3108873505349276, "support": 4767}, "\u23ce": {"f1-score": 0.407911001236094, "precision": 1.0, "recall": 0.2562111801242236, "support": 644}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 88}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 162}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 78}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 168}, "\u2423": {"f1-score": 0.39552715654952075, "precision": 0.9841017488076311, "recall": 0.24750099960015995, "support": 2501}},
  "ppcr": 0.24518533857941602
}
```
</details>
