# Train report for javascript / file:///tmp/top-repos-quality-repos-fb5u__4v/regular-expert.git HEAD 81456c308df79e9137a6e1c4778e928c6c08c7b3

### Classification report

PPCR: 0.569

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.969| 0.989| 0.704| 0.979| 0.816| 4106| 5767| 0.712 |
| `␣` | 0.939| 0.936| 0.421| 0.937| 0.581| 931| 2069| 0.450 |
| `"` | 1.000| 1.000| 0.504| 1.000| 0.670| 380| 754| 0.504 |
| `⏎␣⁻␣⁻` | 1.000| 0.853| 0.460| 0.921| 0.630| 163| 302| 0.540 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 28| 338| 0.083 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 26| 447| 0.058 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 235| 0.021 |
| `weighted avg` | 0.957| 0.967| 0.550| 0.962| 0.666| 5639| 9912| 0.569 |
| `macro avg` | 0.558| 0.540| 0.299| 0.548| 0.385| 5639| 9912| 0.569 |
| `micro avg` | 0.967| 0.967| 0.550| 0.967| 0.701| 5639| 9912| 0.569 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1661 |4062 |44 |0 |0 |0 |0 |0 |
|1138 |60 |871 |0 |0 |0 |0 |0 |
|374 |0 |0 |380 |0 |0 |0 |0 |
|421 |19 |7 |0 |0 |0 |0 |0 |
|310 |23 |5 |0 |0 |0 |0 |0 |
|139 |23 |1 |0 |0 |0 |139 |0 |
|230 |5 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| controllers/realtor.js | 34 |
| controllers/crud.js | 14 |
| src/pages/SignUp.js | 13 |
| src/pages/CreateEditGroup.js | 12 |
| src/pages/CreateGroup.js | 12 |
| src/pages/ShareListing.js | 11 |
| src/components/Listing/index.js | 10 |
| server/routes/api.js | 10 |
| server/passport/localStrategy.js | 9 |
| src/pages/Group.js | 7 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 380}, "macro avg": {"f1-score": 0.5480891186023995, "precision": 0.5582237336893167, "recall": 0.5396568399290581, "support": 5639}, "micro avg": {"f1-score": 0.9668380918602589, "precision": 0.9668380918602589, "recall": 0.9668380918602589, "support": 5639}, "weighted avg": {"f1-score": 0.9615807084718612, "precision": 0.9568155200374969, "recall": 0.9668380918602589, "support": 5639}, "\u2205": {"f1-score": 0.9790310918293563, "precision": 0.9689885496183206, "recall": 0.9892839746712129, "support": 4106}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 26}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 28}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9205298013245032, "precision": 1.0, "recall": 0.852760736196319, "support": 163}, "\u2423": {"f1-score": 0.9370629370629371, "precision": 0.9385775862068966, "recall": 0.9355531686358755, "support": 931}},
  "cl_report_full": {"\"": {"f1-score": 0.6701940035273368, "precision": 1.0, "recall": 0.5039787798408488, "support": 754}, "macro avg": {"f1-score": 0.38536742261469425, "precision": 0.5582237336893167, "recall": 0.2985103353056645, "support": 9912}, "micro avg": {"f1-score": 0.701176773197865, "precision": 0.9668380918602589, "recall": 0.5500403551251009, "support": 9912}, "weighted avg": {"f1-score": 0.6661323109752011, "precision": 0.8662302251322561, "recall": 0.5500403551251009, "support": 9912}, "\u2205": {"f1-score": 0.8157445526659303, "precision": 0.9689885496183206, "recall": 0.704352349575169, "support": 5767}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 447}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 235}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 338}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.6303854875283447, "precision": 1.0, "recall": 0.4602649006622517, "support": 302}, "\u2423": {"f1-score": 0.581247914581248, "precision": 0.9385775862068966, "recall": 0.42097631706138233, "support": 2069}},
  "ppcr": 0.5689063761097659
}
```
</details>
