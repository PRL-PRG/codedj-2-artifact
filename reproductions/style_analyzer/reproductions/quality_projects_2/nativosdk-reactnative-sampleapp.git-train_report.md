# Train report for javascript / file:///tmp/top-repos-quality-repos-mvvat_io/nativosdk-reactnative-sampleapp.git HEAD 8f5c27718267b94d0a4ff69eb01dcf06c189fd85

### Classification report

PPCR: 0.367

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.987| 1.000| 0.622| 0.993| 0.763| 1462| 2352| 0.622 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 20| 1026| 0.019 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 306| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 213| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 136| 0.000 |
| `weighted avg` | 0.973| 0.987| 0.363| 0.980| 0.445| 1482| 4033| 0.367 |
| `micro avg` | 0.987| 0.987| 0.363| 0.987| 0.530| 1482| 4033| 0.367 |
| `macro avg` | 0.197| 0.200| 0.124| 0.199| 0.153| 1482| 4033| 0.367 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|890 |1462 |0 |0 |0 |0 |
|1006 |20 |0 |0 |0 |0 |
|306 |0 |0 |0 |0 |0 |
|213 |0 |0 |0 |0 |0 |
|136 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| screens/HomePage.js | 8 |
| screens/LandingViewPage.js | 4 |
| adTemplates/StandardDisplayAdTemplate.js | 2 |
| publisherTemplate/PublisherCard.js | 2 |
| screens/MOAPViewPage.js | 2 |
| adTemplates/NativeVideoAdTemplate.js | 1 |
| screens/FlatListPage.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.19864130434782606, "precision": 0.1973009446693657, "recall": 0.2, "support": 1482}, "micro avg": {"f1-score": 0.9865047233468286, "precision": 0.9865047233468286, "recall": 0.9865047233468286, "support": 1482}, "weighted avg": {"f1-score": 0.9798029249545267, "precision": 0.9731915691856028, "recall": 0.9865047233468286, "support": 1482}, "\u2205": {"f1-score": 0.9932065217391304, "precision": 0.9865047233468286, "recall": 1.0, "support": 1462}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 20}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 136}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 306}, "macro avg": {"f1-score": 0.1525299947835159, "precision": 0.1973009446693657, "recall": 0.12431972789115646, "support": 4033}, "micro avg": {"f1-score": 0.5301903898458749, "precision": 0.9865047233468286, "recall": 0.3625092982891148, "support": 4033}, "weighted avg": {"f1-score": 0.4447688417193521, "precision": 0.5753184005236154, "recall": 0.3625092982891148, "support": 4033}, "\u2205": {"f1-score": 0.7626499739175795, "precision": 0.9865047233468286, "recall": 0.6215986394557823, "support": 2352}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 213}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1026}},
  "ppcr": 0.3674683858170097
}
```
</details>
