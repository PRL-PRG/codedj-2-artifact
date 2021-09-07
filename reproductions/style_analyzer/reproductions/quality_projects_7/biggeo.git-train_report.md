# Train report for javascript / file:///tmp/top-repos-quality-repos-ffrmb_v3/biggeo.git HEAD 890b0158a5ccafcbf14ae97a329d1fca7c57c89e

### Classification report

PPCR: 0.647

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.980| 0.998| 0.941| 0.989| 0.960| 9564| 10145| 0.943 |
| `␣` | 0.986| 0.909| 0.328| 0.946| 0.492| 2030| 5627| 0.361 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.946| 0.970| 0.829| 0.958| 0.883| 304| 356| 0.854 |
| `"` | 1.000| 1.000| 0.271| 1.000| 0.426| 222| 820| 0.271 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 18| 994| 0.018 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 238| 0.021 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 393| 0.008 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 196| 0.000 |
| `micro avg` | 0.980| 0.980| 0.634| 0.980| 0.770| 12146| 18769| 0.647 |
| `weighted avg` | 0.978| 0.980| 0.634| 0.979| 0.702| 12146| 18769| 0.647 |
| `macro avg` | 0.489| 0.485| 0.296| 0.487| 0.345| 12146| 18769| 0.647 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| '| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|581 |9543 |21 |0 |0 |0 |0 |0 |0 |
|3597 |171 |1845 |0 |0 |0 |14 |0 |0 |
|598 |0 |0 |222 |0 |0 |0 |0 |0 |
|976 |14 |2 |0 |0 |0 |2 |0 |0 |
|390 |2 |1 |0 |0 |0 |0 |0 |0 |
|52 |7 |2 |0 |0 |0 |295 |0 |0 |
|196 |0 |0 |0 |0 |0 |0 |0 |0 |
|233 |3 |1 |0 |0 |0 |1 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| nodejs-express/pg_mapper/public/javascripts/roundslider.js | 121 |
| nodejs-express/pg_mapper/public/javascripts/leaflet.geometryutil.js | 69 |
| nodejs-express/pg_mapper/public/javascripts/leaflet-sidebar.js | 22 |
| nodejs-express/pg_mapper/public/javascripts/leaflet.hotline.js | 21 |
| nodejs-express/pg_mapper/bin/www | 4 |
| nodejs-express/pg_mapper/app.js | 3 |
| nodejs-express/pg_mapper/routes/users.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 222}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.4865210124089846, "precision": 0.4888579838624757, "recall": 0.4846332497391859, "support": 12146}, "micro avg": {"f1-score": 0.9801580767330809, "precision": 0.9801580767330809, "recall": 0.9801580767330809, "support": 12146}, "weighted avg": {"f1-score": 0.9788301051352585, "precision": 0.9781588016528471, "recall": 0.9801580767330809, "support": 12146}, "\u2205": {"f1-score": 0.988707003729797, "precision": 0.9797741273100616, "recall": 0.9978042659974906, "support": 9564}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 18}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9577922077922078, "precision": 0.9455128205128205, "recall": 0.9703947368421053, "support": 304}, "\u2423": {"f1-score": 0.9456688877498719, "precision": 0.9855769230769231, "recall": 0.9088669950738916, "support": 2030}},
  "cl_report_full": {"\"": {"f1-score": 0.42610364683301344, "precision": 1.0, "recall": 0.2707317073170732, "support": 820}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 196}, "macro avg": {"f1-score": 0.34515271844116385, "precision": 0.4888579838624757, "recall": 0.29599090447414567, "support": 18769}, "micro avg": {"f1-score": 0.7701762898269449, "precision": 0.9801580767330809, "recall": 0.6342905855399862, "support": 18769}, "weighted avg": {"f1-score": 0.7016917602297449, "precision": 0.8866882855675308, "recall": 0.6342905855399862, "support": 18769}, "\u2205": {"f1-score": 0.9598189590143325, "precision": 0.9797741273100616, "recall": 0.9406604238541153, "support": 10145}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 994}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 238}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 393}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.8832335329341318, "precision": 0.9455128205128205, "recall": 0.8286516853932584, "support": 356}, "\u2423": {"f1-score": 0.49206560874783306, "precision": 0.9855769230769231, "recall": 0.32788341922871866, "support": 5627}},
  "ppcr": 0.6471309073472215
}
```
</details>
