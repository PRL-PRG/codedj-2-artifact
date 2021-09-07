# Train report for javascript / file:///tmp/top-repos-quality-repos-31kju0mo/stoop.git HEAD 99b093ba96c1daa70c261ae929b13a7cef84ef2a

### Classification report

PPCR: 0.482

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.949| 1.000| 0.790| 0.974| 0.862| 3262| 4129| 0.790 |
| `␣` | 1.000| 0.718| 0.132| 0.836| 0.233| 358| 1945| 0.184 |
| `'` | 1.000| 1.000| 0.271| 1.000| 0.426| 182| 672| 0.271 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 31| 264| 0.117 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 23| 278| 0.083 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 19| 477| 0.040 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 74| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 193| 0.000 |
| `micro avg` | 0.955| 0.955| 0.461| 0.955| 0.622| 3875| 8032| 0.482 |
| `weighted avg` | 0.939| 0.955| 0.461| 0.944| 0.536| 3875| 8032| 0.482 |
| `macro avg` | 0.369| 0.340| 0.149| 0.351| 0.190| 3875| 8032| 0.482 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| "| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|867 |3262 |0 |0 |0 |0 |0 |0 |0 |
|1587 |101 |257 |0 |0 |0 |0 |0 |0 |
|490 |0 |0 |182 |0 |0 |0 |0 |0 |
|458 |19 |0 |0 |0 |0 |0 |0 |0 |
|255 |23 |0 |0 |0 |0 |0 |0 |0 |
|74 |0 |0 |0 |0 |0 |0 |0 |0 |
|233 |31 |0 |0 |0 |0 |0 |0 |0 |
|193 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| client/components/newCatForm.js | 16 |
| client/components/auth-form.js | 13 |
| server/db/models/user.js | 13 |
| client/routes.js | 12 |
| script/encrypt-heroku-auth-token.js | 12 |
| client/components/markers.js | 12 |
| client/components/navbar.js | 8 |
| server/auth/google.js | 8 |
| server/index.js | 7 |
| client/components/landingPage.js | 6 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 182}, "macro avg": {"f1-score": 0.3512243067339601, "precision": 0.3686699650756694, "recall": 0.3397346368715084, "support": 3875}, "micro avg": {"f1-score": 0.9550967741935484, "precision": 0.9550967741935484, "recall": 0.9550967741935484, "support": 3875}, "weighted avg": {"f1-score": 0.9441204081806852, "precision": 0.9385319764166886, "recall": 0.9550967741935484, "support": 3875}, "\u2205": {"f1-score": 0.9740220961481039, "precision": 0.9493597206053551, "recall": 1.0, "support": 3262}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 19}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 23}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 31}, "\u2423": {"f1-score": 0.8357723577235772, "precision": 1.0, "recall": 0.7178770949720671, "support": 358}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 74}, "\u0027": {"f1-score": 0.4262295081967213, "precision": 1.0, "recall": 0.2708333333333333, "support": 672}, "macro avg": {"f1-score": 0.19025578319247907, "precision": 0.3686699650756694, "recall": 0.14912360080889597, "support": 8032}, "micro avg": {"f1-score": 0.6216511295876375, "precision": 0.9550967741935484, "recall": 0.46078187250996017, "support": 8032}, "weighted avg": {"f1-score": 0.5355148475417439, "precision": 0.8138578543799193, "recall": 0.46078187250996017, "support": 8032}, "\u2205": {"f1-score": 0.8623925974884337, "precision": 0.9493597206053551, "recall": 0.7900217970452894, "support": 4129}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 477}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 193}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 278}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 264}, "\u2423": {"f1-score": 0.23342415985467757, "precision": 1.0, "recall": 0.13213367609254498, "support": 1945}},
  "ppcr": 0.48244521912350596
}
```
</details>
