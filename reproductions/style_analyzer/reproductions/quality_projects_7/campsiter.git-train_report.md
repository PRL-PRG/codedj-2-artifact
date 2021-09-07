# Train report for javascript / file:///tmp/top-repos-quality-repos-87w84oth/campsiter.git HEAD 2b0ee376508a4a1f9985c220c4b9798c45293cb7

### Classification report

PPCR: 0.506

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.981| 0.989| 0.671| 0.985| 0.797| 8882| 13085| 0.679 |
| `␣` | 0.946| 0.962| 0.430| 0.954| 0.591| 2765| 6190| 0.447 |
| `"` | 1.000| 1.000| 0.853| 1.000| 0.921| 792| 928| 0.853 |
| `'` | 1.000| 1.000| 0.186| 1.000| 0.313| 266| 1431| 0.186 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 67| 870| 0.077 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 34| 848| 0.040 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 1710| 0.005 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 287| 0.010 |
| `micro avg` | 0.975| 0.975| 0.493| 0.975| 0.655| 12818| 25349| 0.506 |
| `weighted avg` | 0.967| 0.975| 0.493| 0.971| 0.607| 12818| 25349| 0.506 |
| `macro avg` | 0.491| 0.494| 0.267| 0.492| 0.328| 12818| 25349| 0.506 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁻␣⁻| "| ⏎␣⁺␣⁺| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|4203 |8781 |101 |0 |0 |0 |0 |0 |0 |
|3425 |106 |2659 |0 |0 |0 |0 |0 |0 |
|1701 |8 |1 |0 |0 |0 |0 |0 |0 |
|1165 |0 |0 |0 |266 |0 |0 |0 |0 |
|803 |20 |47 |0 |0 |0 |0 |0 |0 |
|136 |0 |0 |0 |0 |0 |792 |0 |0 |
|814 |32 |2 |0 |0 |0 |0 |0 |0 |
|284 |3 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/client/App.js | 42 |
| src/client/components/campgroundPage.js | 20 |
| src/server/controllers/campgroundController.js | 14 |
| src/server/controllers/userController.js | 14 |
| src/client/components/campgroundsHome.js | 11 |
| src/server/middleware.js | 11 |
| src/client/hooks/useGetAndDeleteComments.js | 10 |
| src/client/components/editUser.js | 10 |
| src/client/components/loadingButton.js | 10 |
| src/client/components/userPicDisplay.js | 9 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 792}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 266}, "macro avg": {"f1-score": 0.492345003289295, "precision": 0.4909225829539355, "recall": 0.4937865425044373, "support": 12818}, "micro avg": {"f1-score": 0.9750351068809486, "precision": 0.9750351068809486, "recall": 0.9750351068809486, "support": 12818}, "weighted avg": {"f1-score": 0.9707483245885603, "precision": 0.9665082050095616, "recall": 0.9750351068809486, "support": 12818}, "\u2205": {"f1-score": 0.9848586810228802, "precision": 0.9811173184357542, "recall": 0.9886286872326052, "support": 8882}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 34}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 67}, "\u2423": {"f1-score": 0.9539013452914797, "precision": 0.9462633451957295, "recall": 0.9616636528028933, "support": 2765}},
  "cl_report_full": {"\"": {"f1-score": 0.9209302325581395, "precision": 1.0, "recall": 0.853448275862069, "support": 928}, "\u0027": {"f1-score": 0.3134944018856806, "precision": 1.0, "recall": 0.18588399720475193, "support": 1431}, "macro avg": {"f1-score": 0.3277897860598834, "precision": 0.4909225829539355, "recall": 0.26749622927935646, "support": 25349}, "micro avg": {"f1-score": 0.6549113108182462, "precision": 0.9750351068809486, "recall": 0.49303720067852774, "support": 25349}, "weighted avg": {"f1-score": 0.6071104705935019, "precision": 0.8305767572090974, "recall": 0.49303720067852774, "support": 25349}, "\u2205": {"f1-score": 0.797004765146358, "precision": 0.9811173184357542, "recall": 0.6710737485670615, "support": 13085}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1710}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 287}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 848}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 870}, "\u2423": {"f1-score": 0.5908888888888889, "precision": 0.9462633451957295, "recall": 0.4295638126009693, "support": 6190}},
  "ppcr": 0.5056609728194406
}
```
</details>
