# Train report for javascript / file:///tmp/top-repos-quality-repos-esnobzj6/sysdevops.git HEAD a0e8c515986a6b12b9f5b6e85b0a129045dfcdba

### Classification report

PPCR: 0.777

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.924| 0.991| 0.961| 0.956| 0.942| 5723| 5904| 0.969 |
| `'` | 1.000| 1.000| 0.944| 1.000| 0.971| 1407| 1491| 0.944 |
| `␣` | 0.931| 0.736| 0.360| 0.822| 0.519| 1122| 2294| 0.489 |
| `⏎␣⁻␣⁻` | 1.000| 0.745| 0.710| 0.854| 0.830| 322| 338| 0.953 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 42| 245| 0.171 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 33| 531| 0.062 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 27| 357| 0.076 |
| `weighted avg` | 0.929| 0.939| 0.730| 0.931| 0.760| 8676| 11160| 0.777 |
| `macro avg` | 0.551| 0.496| 0.425| 0.519| 0.466| 8676| 11160| 0.777 |
| `micro avg` | 0.939| 0.939| 0.730| 0.939| 0.821| 8676| 11160| 0.777 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|181 |5674 |49 |0 |0 |0 |0 |0 |
|1172 |296 |826 |0 |0 |0 |0 |0 |
|84 |0 |0 |1407 |0 |0 |0 |0 |
|498 |27 |6 |0 |0 |0 |0 |0 |
|330 |21 |6 |0 |0 |0 |0 |0 |
|16 |82 |0 |0 |0 |0 |240 |0 |
|203 |42 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| test/route-install.js | 67 |
| lib/routes/install.js | 62 |
| test/completion.js | 44 |
| test/route-clear-config.js | 38 |
| lib/routes/home.js | 35 |
| test/route-home.js | 33 |
| lib/completion/completer.js | 31 |
| lib/routes/clear-config.js | 27 |
| test/router.js | 25 |
| gulpfile.js | 24 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1407}, "macro avg": {"f1-score": 0.5189740918070573, "precision": 0.5507188832462871, "recall": 0.4961378650163101, "support": 8676}, "micro avg": {"f1-score": 0.9390272014753342, "precision": 0.9390272014753342, "recall": 0.9390272014753342, "support": 8676}, "weighted avg": {"f1-score": 0.9311049629280326, "precision": 0.9290877352176894, "recall": 0.9390272014753342, "support": 8676}, "\u2205": {"f1-score": 0.9564264643910662, "precision": 0.9238033213936828, "recall": 0.9914380569631313, "support": 5723}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 33}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 42}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 27}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8540925266903914, "precision": 1.0, "recall": 0.7453416149068323, "support": 322}, "\u2423": {"f1-score": 0.8222996515679443, "precision": 0.9312288613303269, "recall": 0.7361853832442068, "support": 1122}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9710144927536231, "precision": 1.0, "recall": 0.9436619718309859, "support": 1491}, "macro avg": {"f1-score": 0.46612190239241735, "precision": 0.5507188832462871, "recall": 0.424976321575535, "support": 11160}, "micro avg": {"f1-score": 0.8214357733413995, "precision": 0.9390272014753342, "recall": 0.7300179211469534, "support": 11160}, "weighted avg": {"f1-score": 0.760010860091311, "precision": 0.8440299119534115, "recall": 0.7300179211469534, "support": 11160}, "\u2205": {"f1-score": 0.9420554540926449, "precision": 0.9238033213936828, "recall": 0.9610433604336044, "support": 5904}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 531}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 245}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 357}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8304498269896193, "precision": 1.0, "recall": 0.7100591715976331, "support": 338}, "\u2423": {"f1-score": 0.5193335429110342, "precision": 0.9312288613303269, "recall": 0.36006974716652135, "support": 2294}},
  "ppcr": 0.7774193548387097
}
```
</details>
