# Train report for javascript / file:///tmp/top-repos-quality-repos-_ojbtj4o/lisk-roulette.git HEAD 4f30d12e9c770f1a1d532b91e56271091ed3cd83

### Classification report

PPCR: 0.715

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.955| 1.000| 0.949| 0.977| 0.952| 13202| 13916| 0.949 |
| `␣` | 0.958| 0.712| 0.233| 0.817| 0.375| 1224| 3733| 0.328 |
| `⏎␣⁻␣⁻` | 0.996| 0.824| 0.571| 0.902| 0.726| 278| 401| 0.693 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 208| 417| 0.499 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 48| 825| 0.058 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 548| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 877| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 204| 0.000 |
| `macro avg` | 0.364| 0.317| 0.219| 0.337| 0.257| 14960| 20921| 0.715 |
| `weighted avg` | 0.940| 0.956| 0.684| 0.946| 0.714| 14960| 20921| 0.715 |
| `micro avg` | 0.956| 0.956| 0.684| 0.956| 0.797| 14960| 20921| 0.715 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|714 |13202 |0 |0 |0 |0 |0 |0 |0 |
|2509 |352 |871 |0 |0 |0 |0 |1 |0 |
|777 |15 |33 |0 |0 |0 |0 |0 |0 |
|548 |0 |0 |0 |0 |0 |0 |0 |0 |
|877 |0 |0 |0 |0 |0 |0 |0 |0 |
|209 |203 |5 |0 |0 |0 |0 |0 |0 |
|123 |49 |0 |0 |0 |0 |0 |229 |0 |
|204 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| client/src/components/field/index.js | 105 |
| client/src/components/table/index.js | 76 |
| client/src/serviceWorker.js | 70 |
| client/src/App.js | 61 |
| client/src/components/txviewer/table.js | 47 |
| client/src/components/login/index.js | 47 |
| client/src/transactions/1001_bet_roulette_transaction.js | 44 |
| client/src/transactions/101_faucet_transaction.js | 38 |
| client/src/components/roulette/index.js | 30 |
| client/src/components/txviewer/index.js | 27 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.33691981224042616, "precision": 0.36363263437969773, "recall": 0.31691778929797343, "support": 14960}, "micro avg": {"f1-score": 0.9560160427807487, "precision": 0.9560160427807487, "recall": 0.9560160427807487, "support": 14960}, "weighted avg": {"f1-score": 0.9458459907820456, "precision": 0.9398627065535619, "recall": 0.9560160427807487, "support": 14960}, "\u2205": {"f1-score": 0.9770935869444547, "precision": 0.9552130815425801, "recall": 1.0, "support": 13202}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 48}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 208}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9015748031496064, "precision": 0.9956521739130435, "recall": 0.8237410071942446, "support": 278}, "\u2423": {"f1-score": 0.8166901078293484, "precision": 0.9581958195819582, "recall": 0.7116013071895425, "support": 1224}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 877}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 548}, "macro avg": {"f1-score": 0.2566303429011888, "precision": 0.36363263437969773, "recall": 0.21913610951051748, "support": 20921}, "micro avg": {"f1-score": 0.7971907137482233, "precision": 0.9560160427807487, "recall": 0.6836193298599493, "support": 20921}, "weighted avg": {"f1-score": 0.7140746656470852, "precision": 0.8254360097024581, "recall": 0.6836193298599493, "support": 20921}, "\u2205": {"f1-score": 0.9519414500486716, "precision": 0.9552130815425801, "recall": 0.9486921529175051, "support": 13916}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 825}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 204}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 417}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7258320126782885, "precision": 0.9956521739130435, "recall": 0.571072319201995, "support": 401}, "\u2423": {"f1-score": 0.37526928048255065, "precision": 0.9581958195819582, "recall": 0.2333244039646397, "support": 3733}},
  "ppcr": 0.7150709813106448
}
```
</details>
