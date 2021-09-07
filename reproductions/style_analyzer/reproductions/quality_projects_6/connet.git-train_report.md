# Train report for javascript / file:///tmp/top-repos-quality-repos-oeexcjg8/connet.git HEAD 7df0655579d699f9cf686acb28b953bad3b4b2bc

### Classification report

PPCR: 0.748

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.944| 0.997| 0.918| 0.970| 0.931| 12161| 13207| 0.921 |
| `'` | 1.000| 1.000| 0.938| 1.000| 0.968| 2466| 2630| 0.938 |
| `␣` | 0.978| 0.838| 0.388| 0.902| 0.556| 2242| 4834| 0.464 |
| `⏎⇥⁻` | 0.985| 0.667| 0.566| 0.796| 0.719| 583| 687| 0.849 |
| `⏎⇥⁺` | 0.921| 0.576| 0.250| 0.709| 0.393| 283| 653| 0.433 |
| `⏎` | 0.936| 0.756| 0.105| 0.836| 0.188| 250| 1805| 0.139 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 24| 259| 0.093 |
| `micro avg` | 0.956| 0.956| 0.715| 0.956| 0.818| 18009| 24075| 0.748 |
| `weighted avg` | 0.955| 0.956| 0.715| 0.952| 0.773| 18009| 24075| 0.748 |
| `macro avg` | 0.823| 0.691| 0.452| 0.745| 0.536| 18009| 24075| 0.748 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⇥⁻| ⏎⇥⁺| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1046 |12124 |33 |0 |0 |0 |4 |0 |
|2592 |356 |1878 |0 |0 |0 |8 |0 |
|164 |0 |0 |2466 |0 |0 |0 |0 |
|1555 |52 |1 |0 |189 |6 |2 |0 |
|104 |192 |2 |0 |0 |389 |0 |0 |
|370 |114 |6 |0 |0 |0 |163 |0 |
|235 |11 |0 |0 |13 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| front/src/components/NavBar/NavBar.js | 101 |
| front/src/components/Room/Room.js | 68 |
| front/src/App.js | 65 |
| back/api/profileApi.js | 60 |
| back/api/roomApi.js | 56 |
| front/src/components/Event/Event.js | 48 |
| front/src/serviceWorker.js | 41 |
| back/api/eventApi.js | 28 |
| front/src/components/UpdateProfile/UpdateProfile.js | 27 |
| front/src/components/Banner/Banner.js | 27 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 2466}, "macro avg": {"f1-score": 0.7446375417812675, "precision": 0.823294003036654, "recall": 0.6905446571871713, "support": 18009}, "micro avg": {"f1-score": 0.9555777666722195, "precision": 0.9555777666722195, "recall": 0.9555777666722195, "support": 18009}, "weighted avg": {"f1-score": 0.9524784418727952, "precision": 0.9552139155833683, "recall": 0.9555777666722195, "support": 18009}, "\u2205": {"f1-score": 0.9695321871251499, "precision": 0.9435753755156043, "recall": 0.9969574870487624, "support": 12161}, "\u23ce": {"f1-score": 0.8362831858407079, "precision": 0.9356435643564357, "recall": 0.756, "support": 250}, "\u23ce\u21e5\u207a": {"f1-score": 0.7086956521739131, "precision": 0.9209039548022598, "recall": 0.5759717314487632, "support": 283}, "\u23ce\u21e5\u207b": {"f1-score": 0.7955010224948874, "precision": 0.9848101265822785, "recall": 0.6672384219554031, "support": 583}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 24}, "\u2423": {"f1-score": 0.9024507448342144, "precision": 0.978125, "recall": 0.8376449598572703, "support": 2242}},
  "cl_report_full": {"\u0027": {"f1-score": 0.967817896389325, "precision": 1.0, "recall": 0.9376425855513308, "support": 2630}, "macro avg": {"f1-score": 0.53638492735634, "precision": 0.823294003036654, "recall": 0.4520992904874772, "support": 24075}, "micro avg": {"f1-score": 0.8178405094572759, "precision": 0.9555777666722195, "recall": 0.7148078920041537, "support": 24075}, "weighted avg": {"f1-score": 0.7731926969537598, "precision": 0.94649253198529, "recall": 0.7148078920041537, "support": 24075}, "\u2205": {"f1-score": 0.9306109917101628, "precision": 0.9435753755156043, "recall": 0.917998031347013, "support": 13207}, "\u23ce": {"f1-score": 0.1883408071748879, "precision": 0.9356435643564357, "recall": 0.10470914127423822, "support": 1805}, "\u23ce\u21e5\u207a": {"f1-score": 0.3927710843373494, "precision": 0.9209039548022598, "recall": 0.24961715160796324, "support": 653}, "\u23ce\u21e5\u207b": {"f1-score": 0.7190388170055454, "precision": 0.9848101265822785, "recall": 0.5662299854439592, "support": 687}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 259}, "\u2423": {"f1-score": 0.5561148948771099, "precision": 0.978125, "recall": 0.3884981381878362, "support": 4834}},
  "ppcr": 0.7480373831775701
}
```
</details>
