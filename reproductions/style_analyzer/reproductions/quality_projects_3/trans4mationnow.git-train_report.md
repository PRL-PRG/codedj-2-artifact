# Train report for javascript / file:///tmp/top-repos-quality-repos-aqrws9qf/trans4mationnow.git HEAD fa4ae9a3dd29627f02bcbfb8de842c31ce548e61

### Classification report

PPCR: 0.076

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `␣` | 0.935| 0.998| 0.207| 0.965| 0.339| 503| 2426| 0.207 |
| `'` | 1.000| 1.000| 0.250| 1.000| 0.400| 382| 1528| 0.250 |
| `∅` | 0.994| 1.000| 0.019| 0.997| 0.038| 158| 8225| 0.019 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 13| 141| 0.092 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 12| 136| 0.088 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 212| 0.028 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 164| 0.018 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 792| 0.001 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 292| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 234| 0.000 |
| `macro avg` | 0.293| 0.300| 0.048| 0.296| 0.078| 1078| 14150| 0.076 |
| `weighted avg` | 0.936| 0.967| 0.074| 0.951| 0.123| 1078| 14150| 0.076 |
| `micro avg` | 0.967| 0.967| 0.074| 0.967| 0.137| 1078| 14150| 0.076 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| ⏎␣⁺␣⁺| ⏎⇥⁺| ⏎⇥⁻| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|8067 |158 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1923 |1 |502 |0 |0 |0 |0 |0 |0 |0 |0 |
|1146 |0 |0 |382 |0 |0 |0 |0 |0 |0 |0 |
|791 |0 |1 |0 |0 |0 |0 |0 |0 |0 |0 |
|292 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|234 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|128 |0 |13 |0 |0 |0 |0 |0 |0 |0 |0 |
|124 |0 |12 |0 |0 |0 |0 |0 |0 |0 |0 |
|206 |0 |6 |0 |0 |0 |0 |0 |0 |0 |0 |
|161 |0 |3 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| client/src/components/Resources/index.js | 24 |
| routes/api/profile.js | 2 |
| routes/api/users.js | 2 |
| client/src/components/Blog/index.js | 1 |
| client/src/components/Survey/index.js | 1 |
| client/src/components/GEMsUniversity/GemForm.js | 1 |
| models/index.js | 1 |
| client/src/components/Jumbotron/index.js | 1 |
| client/src/components/Home/Testimonial.js | 1 |
| client/src/components/Resources/ResourceItem.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 382}, "macro avg": {"f1-score": 0.29622300412521235, "precision": 0.2928533783071572, "recall": 0.2998011928429424, "support": 1078}, "micro avg": {"f1-score": 0.9666048237476809, "precision": 0.9666048237476809, "recall": 0.9666048237476809, "support": 1078}, "weighted avg": {"f1-score": 0.9509184033631982, "precision": 0.9361987979645224, "recall": 0.9666048237476809, "support": 1078}, "\u2205": {"f1-score": 0.9968454258675079, "precision": 0.9937106918238994, "recall": 1.0, "support": 158}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 13}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u2423": {"f1-score": 0.9653846153846153, "precision": 0.9348230912476723, "recall": 0.9980119284294234, "support": 503}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 292}, "\u0027": {"f1-score": 0.4, "precision": 1.0, "recall": 0.25, "support": 1528}, "macro avg": {"f1-score": 0.07765366041226011, "precision": 0.2928533783071572, "recall": 0.047613470583371134, "support": 14150}, "micro avg": {"f1-score": 0.1368531652219595, "precision": 0.9666048237476809, "recall": 0.07363957597173144, "support": 14150}, "weighted avg": {"f1-score": 0.12319766650111261, "precision": 0.8458764141073092, "recall": 0.07363957597173144, "support": 14150}, "\u2205": {"f1-score": 0.037690839694656496, "precision": 0.9937106918238994, "recall": 0.019209726443768997, "support": 8225}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 792}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 141}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 136}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 164}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 234}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 212}, "\u2423": {"f1-score": 0.3388457644279446, "precision": 0.9348230912476723, "recall": 0.20692497938994228, "support": 2426}},
  "ppcr": 0.07618374558303886
}
```
</details>
