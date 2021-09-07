# Train report for javascript / file:///tmp/top-repos-quality-repos-k84oyq4k/sibi-activity-search-v3.git HEAD e1190faedef65d911c0c22cebe1cab1f542757dc

### Classification report

PPCR: 0.320

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.975| 1.000| 0.558| 0.987| 0.710| 2278| 4080| 0.558 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 33| 176| 0.188 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 22| 1759| 0.013 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 413| 0.005 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 176| 0.006 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 685| 0.000 |
| `macro avg` | 0.163| 0.167| 0.093| 0.165| 0.118| 2336| 7289| 0.320 |
| `weighted avg` | 0.951| 0.975| 0.313| 0.963| 0.397| 2336| 7289| 0.320 |
| `micro avg` | 0.975| 0.975| 0.313| 0.975| 0.473| 2336| 7289| 0.320 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|1802 |2278 |0 |0 |0 |0 |0 |
|1737 |22 |0 |0 |0 |0 |0 |
|685 |0 |0 |0 |0 |0 |0 |
|411 |2 |0 |0 |0 |0 |0 |
|143 |33 |0 |0 |0 |0 |0 |
|175 |1 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/components/Review.js | 13 |
| src/index.js | 12 |
| src/components/SearchResults.js | 9 |
| src/components/activity-display/ActivityTimes.js | 7 |
| src/components/ActivitySearchForm.js | 7 |
| src/components/Modal.js | 3 |
| src/components/PDFDocument.js | 2 |
| src/components/ActivitySearch.js | 2 |
| src/components/Loading.js | 1 |
| src/components/Nav.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.16457159370033234, "precision": 0.1625285388127854, "recall": 0.16666666666666666, "support": 2336}, "micro avg": {"f1-score": 0.9751712328767124, "precision": 0.9751712328767124, "recall": 0.9751712328767124, "support": 2336}, "weighted avg": {"f1-score": 0.9629129035514307, "precision": 0.9509589334302871, "recall": 0.9751712328767124, "support": 2336}, "\u2205": {"f1-score": 0.987429562201994, "precision": 0.9751712328767124, "recall": 1.0, "support": 2278}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 33}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 685}, "macro avg": {"f1-score": 0.11834995843724024, "precision": 0.1625285388127854, "recall": 0.09305555555555556, "support": 7289}, "micro avg": {"f1-score": 0.4733506493506493, "precision": 0.9751712328767124, "recall": 0.3125257236932364, "support": 7289}, "weighted avg": {"f1-score": 0.3974766061933929, "precision": 0.5458497228888718, "recall": 0.3125257236932364, "support": 7289}, "\u2205": {"f1-score": 0.7100997506234414, "precision": 0.9751712328767124, "recall": 0.5583333333333333, "support": 4080}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 413}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 176}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 176}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1759}},
  "ppcr": 0.32048291946769103
}
```
</details>
