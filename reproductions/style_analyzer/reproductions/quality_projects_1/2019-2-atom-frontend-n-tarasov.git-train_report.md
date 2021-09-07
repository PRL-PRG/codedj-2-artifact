# Train report for javascript / file:///tmp/top-repos-quality-repos-gil9c1pi/2019-2-atom-frontend-n-tarasov.git HEAD 8fa9ccf47a3d6aea525cda63e7dfde81b5c2d673

### Classification report

PPCR: 0.429

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.979| 0.987| 0.505| 0.983| 0.666| 4797| 9374| 0.512 |
| `␣` | 0.953| 0.995| 0.558| 0.974| 0.704| 2146| 3827| 0.561 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 58| 432| 0.134 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 44| 989| 0.044 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 34| 478| 0.071 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1027| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 226| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 147| 0.000 |
| `macro avg` | 0.241| 0.248| 0.133| 0.245| 0.171| 7079| 16500| 0.429 |
| `micro avg` | 0.970| 0.970| 0.416| 0.970| 0.583| 7079| 16500| 0.429 |
| `weighted avg` | 0.952| 0.970| 0.416| 0.961| 0.542| 7079| 16500| 0.429 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⇥⁺| ⏎⇥⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|4577 |4734 |63 |0 |0 |0 |0 |0 |0 |
|1681 |10 |2136 |0 |0 |0 |0 |0 |0 |
|1027 |0 |0 |0 |0 |0 |0 |0 |0 |
|945 |29 |15 |0 |0 |0 |0 |0 |0 |
|444 |20 |14 |0 |0 |0 |0 |0 |0 |
|374 |44 |14 |0 |0 |0 |0 |0 |0 |
|226 |0 |0 |0 |0 |0 |0 |0 |0 |
|147 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| exam/src/components/CityProfile.js | 30 |
| src/components/Chat/MessageField/MessageField.js | 19 |
| exam/src/serviceWorker.js | 15 |
| exam/src/actions/index.js | 14 |
| src/utils/serviceWorker.js | 13 |
| src/serviceWorker.js | 13 |
| src/App.js | 13 |
| exam/src/App.js | 12 |
| src/components/Chat/MessageField/DropMenu/DropMenu.js | 9 |
| src/components/Chat/FormInput/Microphone/Microphone.js | 9 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.2445417030877992, "precision": 0.2414283242999205, "recall": 0.24777586993735018, "support": 7079}, "micro avg": {"f1-score": 0.9704760559401046, "precision": 0.9704760559401046, "recall": 0.9704760559401046, "support": 7079}, "weighted avg": {"f1-score": 0.9610981110401042, "precision": 0.9520257906855878, "recall": 0.9704760559401046, "support": 7079}, "\u2205": {"f1-score": 0.9827693585219015, "precision": 0.9787058093859831, "recall": 0.9868667917448405, "support": 4797}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 44}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 34}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 58}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9735642661804922, "precision": 0.9527207850133809, "recall": 0.9953401677539608, "support": 2146}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 147}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1027}, "macro avg": {"f1-score": 0.17126869374579917, "precision": 0.2414283242999205, "recall": 0.13289417537870707, "support": 16500}, "micro avg": {"f1-score": 0.5827219135671573, "precision": 0.9704760559401046, "recall": 0.4163636363636364, "support": 16500}, "weighted avg": {"f1-score": 0.5417709296436974, "precision": 0.776997012207904, "recall": 0.4163636363636364, "support": 16500}, "\u2205": {"f1-score": 0.6662444585180494, "precision": 0.9787058093859831, "recall": 0.5050138681459355, "support": 9374}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 989}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 478}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 432}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 226}, "\u2423": {"f1-score": 0.703905091448344, "precision": 0.9527207850133809, "recall": 0.5581395348837209, "support": 3827}},
  "ppcr": 0.42903030303030304
}
```
</details>
