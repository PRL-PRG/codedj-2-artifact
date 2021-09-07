# Train report for javascript / file:///tmp/top-repos-quality-repos-jf6o__r_/fit-conditions.git HEAD c707db68924d22a738ff5b722d94c31413a86899

### Classification report

PPCR: 0.407

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.969| 0.997| 0.585| 0.983| 0.729| 2331| 3972| 0.587 |
| `⏎` | 0.989| 0.966| 0.578| 0.977| 0.729| 381| 637| 0.598 |
| `␣` | 0.944| 0.922| 0.228| 0.933| 0.368| 294| 1187| 0.248 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 33| 122| 0.270 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 129| 0.078 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 86| 0.047 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 83| 0.036 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1168| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 116| 0.000 |
| `macro avg` | 0.323| 0.320| 0.155| 0.321| 0.203| 3056| 7500| 0.407 |
| `weighted avg` | 0.953| 0.969| 0.395| 0.961| 0.506| 3056| 7500| 0.407 |
| `micro avg` | 0.969| 0.969| 0.395| 0.969| 0.561| 3056| 7500| 0.407 |

### Confusion matrix

|refusal|  ∅| "| ␣| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| '| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1641 |2323 |0 |7 |1 |0 |0 |0 |0 |0 |
|1168 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|893 |23 |0 |271 |0 |0 |0 |0 |0 |0 |
|256 |12 |0 |1 |368 |0 |0 |0 |0 |0 |
|119 |4 |0 |6 |0 |0 |0 |0 |0 |0 |
|89 |32 |0 |0 |1 |0 |0 |0 |0 |0 |
|116 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|82 |2 |0 |2 |0 |0 |0 |0 |0 |0 |
|80 |1 |0 |0 |2 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| client/src/registerServiceWorker.js | 14 |
| client/src/pages/Profile.js | 13 |
| client/src/pages/Signup.js | 12 |
| client/src/pages/Home.js | 11 |
| client/src/index.js | 9 |
| client/src/components/Navbar/Navbar.js | 7 |
| client/src/pages/Indoor.js | 7 |
| config/auth.js | 5 |
| scripts/seedDB.js | 5 |
| client/src/pages/Login.js | 4 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.3214393897468583, "precision": 0.3225140288523018, "recall": 0.3204684410158726, "support": 3056}, "micro avg": {"f1-score": 0.9692408376963351, "precision": 0.9692408376963351, "recall": 0.9692408376963351, "support": 3056}, "weighted avg": {"f1-score": 0.9611373696468739, "precision": 0.9533869531536909, "recall": 0.9692408376963351, "support": 3056}, "\u2205": {"f1-score": 0.9826565143824026, "precision": 0.9691280767626199, "recall": 0.9965679965679966, "support": 2331}, "\u23ce": {"f1-score": 0.9774236387782205, "precision": 0.989247311827957, "recall": 0.9658792650918635, "support": 381}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 33}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u2423": {"f1-score": 0.9328743545611016, "precision": 0.9442508710801394, "recall": 0.9217687074829932, "support": 294}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1168}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 116}, "macro avg": {"f1-score": 0.20295698652623828, "precision": 0.3225140288523018, "recall": 0.15453984100719576, "support": 7500}, "micro avg": {"f1-score": 0.5611974232663889, "precision": 0.9692408376963351, "recall": 0.39493333333333336, "support": 7500}, "weighted avg": {"f1-score": 0.5064768768459514, "precision": 0.7467137390010214, "recall": 0.39493333333333336, "support": 7500}, "\u2205": {"f1-score": 0.7294708745485947, "precision": 0.9691280767626199, "recall": 0.5848439073514602, "support": 3972}, "\u23ce": {"f1-score": 0.7294350842418237, "precision": 0.989247311827957, "recall": 0.5777080062794349, "support": 637}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 129}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 86}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 122}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 83}, "\u2423": {"f1-score": 0.3677069199457259, "precision": 0.9442508710801394, "recall": 0.22830665543386688, "support": 1187}},
  "ppcr": 0.40746666666666664
}
```
</details>
