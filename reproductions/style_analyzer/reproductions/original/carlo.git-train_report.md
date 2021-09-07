# Train report for javascript / file:///tmp/top-repos-quality-repos-e6zsikgn/carlo.git HEAD 8f2cbfedf381818792017fe53651fe07f270bb96

### Classification report

PPCR: 0.697

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.984| 0.996| 0.936| 0.990| 0.959| 11474| 12206| 0.940 |
| `'` | 1.000| 1.000| 0.924| 1.000| 0.961| 1454| 1573| 0.924 |
| `␣` | 0.949| 0.850| 0.235| 0.897| 0.377| 1332| 4811| 0.277 |
| `⏎␣⁻␣⁻` | 0.920| 0.977| 0.555| 0.947| 0.692| 258| 454| 0.568 |
| `⏎` | 0.975| 0.983| 0.101| 0.979| 0.183| 118| 1148| 0.103 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 16| 541| 0.030 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 306| 0.010 |
| `weighted avg` | 0.980| 0.981| 0.684| 0.980| 0.740| 14655| 21039| 0.697 |
| `macro avg` | 0.690| 0.687| 0.393| 0.688| 0.453| 14655| 21039| 0.697 |
| `micro avg` | 0.981| 0.981| 0.684| 0.981| 0.806| 14655| 21039| 0.697 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|732 |11428 |46 |0 |0 |0 |0 |0 |
|3479 |178 |1132 |0 |0 |0 |22 |0 |
|119 |0 |0 |1454 |0 |0 |0 |0 |
|1030 |2 |0 |0 |116 |0 |0 |0 |
|525 |3 |13 |0 |0 |0 |0 |0 |
|196 |4 |2 |0 |0 |0 |252 |0 |
|303 |0 |0 |0 |3 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| rpc/test.js | 61 |
| lib/carlo.js | 59 |
| rpc/rpc.js | 26 |
| test/app.spec.js | 25 |
| lib/find_chrome.js | 24 |
| lib/color.js | 23 |
| examples/systeminfo/test.js | 7 |
| test/headful.js | 6 |
| examples/systeminfo/app.js | 5 |
| test/test.js | 5 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1454}, "macro avg": {"f1-score": 0.6875448074996022, "precision": 0.6896094961858498, "recall": 0.6865194027690389, "support": 14655}, "micro avg": {"f1-score": 0.9813715455475946, "precision": 0.9813715455475946, "recall": 0.9813715455475946, "support": 14655}, "weighted avg": {"f1-score": 0.9803110940591659, "precision": 0.9798343685341422, "recall": 0.9813715455475946, "support": 14655}, "\u2205": {"f1-score": 0.9899086144917493, "precision": 0.9839001291433491, "recall": 0.9959909360292836, "support": 11474}, "\u23ce": {"f1-score": 0.9789029535864979, "precision": 0.9747899159663865, "recall": 0.9830508474576272, "support": 118}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9473684210526316, "precision": 0.9197080291970803, "recall": 0.9767441860465116, "support": 258}, "\u2423": {"f1-score": 0.8966336633663367, "precision": 0.9488683989941324, "recall": 0.8498498498498499, "support": 1332}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9606871489924018, "precision": 1.0, "recall": 0.9243483788938335, "support": 1573}, "macro avg": {"f1-score": 0.4532394315271256, "precision": 0.6896094961858498, "recall": 0.39314496104576435, "support": 21039}, "micro avg": {"f1-score": 0.805849722642461, "precision": 0.9813715455475946, "recall": 0.6835876229858834, "support": 21039}, "weighted avg": {"f1-score": 0.7396429923734645, "precision": 0.9356004141199381, "recall": 0.6835876229858834, "support": 21039}, "\u2205": {"f1-score": 0.9594895260484447, "precision": 0.9839001291433491, "recall": 0.9362608553170572, "support": 12206}, "\u23ce": {"f1-score": 0.18310970797158643, "precision": 0.9747899159663865, "recall": 0.10104529616724739, "support": 1148}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 306}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 541}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.6923076923076923, "precision": 0.9197080291970803, "recall": 0.5550660792951542, "support": 454}, "\u2423": {"f1-score": 0.3770819453697535, "precision": 0.9488683989941324, "recall": 0.23529411764705882, "support": 4811}},
  "ppcr": 0.6965635248823613
}
```
</details>
