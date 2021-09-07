# Train report for javascript / file:///tmp/top-repos-quality-repos-n52d8we5/phantom-blue.git HEAD 3637b1547de171fa8ae28423799e996a3088951b

### Classification report

PPCR: 0.642

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.982| 0.993| 0.829| 0.988| 0.899| 9379| 11236| 0.835 |
| `␣` | 0.947| 0.976| 0.498| 0.961| 0.653| 2455| 4810| 0.510 |
| `'` | 1.000| 1.000| 0.947| 1.000| 0.973| 1295| 1368| 0.947 |
| `"` | 1.000| 1.000| 0.987| 1.000| 0.993| 379| 384| 0.987 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 88| 1555| 0.057 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 44| 786| 0.056 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 44| 772| 0.057 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 401| 0.020 |
| `micro avg` | 0.978| 0.978| 0.628| 0.978| 0.765| 13692| 21312| 0.642 |
| `weighted avg` | 0.965| 0.978| 0.628| 0.971| 0.702| 13692| 21312| 0.642 |
| `macro avg` | 0.491| 0.496| 0.408| 0.494| 0.440| 13692| 21312| 0.642 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1857 |9316 |63 |0 |0 |0 |0 |0 |0 |
|2355 |58 |2397 |0 |0 |0 |0 |0 |0 |
|1467 |44 |44 |0 |0 |0 |0 |0 |0 |
|73 |0 |0 |0 |1295 |0 |0 |0 |0 |
|742 |24 |20 |0 |0 |0 |0 |0 |0 |
|728 |44 |0 |0 |0 |0 |0 |0 |0 |
|393 |0 |8 |0 |0 |0 |0 |0 |0 |
|5 |0 |0 |0 |0 |0 |0 |0 |379 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| server/db/models/Location.js | 30 |
| script/seed.js | 26 |
| client/components/mapView/MapView.js | 24 |
| client/components/updateArtworkForm/UpdateArtworkForm.js | 17 |
| client/store/artworks.js | 15 |
| script/encrypt-heroku-auth-token.js | 13 |
| client/store/user.js | 13 |
| client/components/mainHome/MainHome.js | 11 |
| client/components/artwork/ArtworkOptions.js | 10 |
| client/components/artwork/LocationArtwork.js | 10 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 379}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1295}, "macro avg": {"f1-score": 0.4936185579990084, "precision": 0.49109516468769643, "recall": 0.4962072014243373, "support": 13692}, "micro avg": {"f1-score": 0.9777242185217645, "precision": 0.9777242185217645, "recall": 0.9777242185217645, "support": 13692}, "weighted avg": {"f1-score": 0.9711620470239689, "precision": 0.9647256071400073, "recall": 0.9777242185217645, "support": 13692}, "\u2205": {"f1-score": 0.9876490856082693, "precision": 0.982078853046595, "recall": 0.9932828659771831, "support": 9379}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 88}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 44}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 44}, "\u2423": {"f1-score": 0.9612993783837979, "precision": 0.9466824644549763, "recall": 0.9763747454175152, "support": 2455}},
  "cl_report_full": {"\"": {"f1-score": 0.9934469200524246, "precision": 1.0, "recall": 0.9869791666666666, "support": 384}, "\u0027": {"f1-score": 0.9725873075478784, "precision": 1.0, "recall": 0.9466374269005848, "support": 1368}, "macro avg": {"f1-score": 0.43976635438563555, "precision": 0.49109516468769643, "recall": 0.40763425942766446, "support": 21312}, "micro avg": {"f1-score": 0.7648840132556279, "precision": 0.9777242185217645, "recall": 0.6281437687687688, "support": 21312}, "weighted avg": {"f1-score": 0.7017383569551549, "precision": 0.8136346024239854, "recall": 0.6281437687687688, "support": 21312}, "\u2205": {"f1-score": 0.8991410095550623, "precision": 0.982078853046595, "recall": 0.829120683517266, "support": 11236}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1555}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 401}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 786}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 772}, "\u2423": {"f1-score": 0.6529555979297194, "precision": 0.9466824644549763, "recall": 0.49833679833679834, "support": 4810}},
  "ppcr": 0.642454954954955
}
```
</details>
