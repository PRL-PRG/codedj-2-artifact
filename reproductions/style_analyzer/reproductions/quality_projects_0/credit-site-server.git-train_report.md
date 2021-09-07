# Train report for javascript / file:///tmp/top-repos-quality-repos-dghgzwi4/credit-site-server.git HEAD 36e265a6d4834ae9a8f251c72da47d3bd7a8b58b

### Classification report

PPCR: 0.606

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.959| 0.961| 0.635| 0.960| 0.764| 5401| 8175| 0.661 |
| `␣` | 0.920| 0.908| 0.602| 0.914| 0.728| 2870| 4327| 0.663 |
| `⏎` | 0.911| 1.000| 0.551| 0.954| 0.687| 565| 1025| 0.551 |
| `'` | 1.000| 1.000| 0.506| 1.000| 0.672| 562| 1110| 0.506 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.972| 0.983| 0.909| 0.977| 0.939| 528| 571| 0.925 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 19| 744| 0.026 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 17| 488| 0.035 |
| `macro avg` | 0.680| 0.693| 0.458| 0.686| 0.542| 9962| 16440| 0.606 |
| `micro avg` | 0.948| 0.948| 0.574| 0.948| 0.715| 9962| 16440| 0.606 |
| `weighted avg` | 0.945| 0.948| 0.574| 0.946| 0.692| 9962| 16440| 0.606 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|2774 |5190 |210 |0 |0 |0 |1 |0 |
|1457 |214 |2607 |0 |35 |0 |14 |0 |
|548 |0 |0 |562 |0 |0 |0 |0 |
|460 |0 |0 |0 |565 |0 |0 |0 |
|725 |6 |13 |0 |0 |0 |0 |0 |
|43 |3 |0 |0 |6 |0 |519 |0 |
|471 |0 |3 |0 |14 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/controllers/auth.js | 34 |
| src/controllers/api/managers.js | 25 |
| frontend/src/api/clients.js | 22 |
| frontend/src/api/managers.js | 20 |
| frontend/src/api/profile.js | 19 |
| frontend/src/shared/Table/Table.js | 19 |
| frontend/src/services/notification.js | 18 |
| frontend/src/components/ErrorPages/styles.js | 17 |
| src/validator/index.js | 17 |
| src/controllers/api/clients.js | 15 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 562}, "macro avg": {"f1-score": 0.6864443031478106, "precision": 0.6803184608316453, "recall": 0.6931785821883362, "support": 9962}, "micro avg": {"f1-score": 0.9479020277052801, "precision": 0.9479020277052801, "recall": 0.9479020277052801, "support": 9962}, "weighted avg": {"f1-score": 0.946095030766762, "precision": 0.9445483131530741, "recall": 0.9479020277052801, "support": 9962}, "\u2205": {"f1-score": 0.9598668392824117, "precision": 0.958802881950859, "recall": 0.9609331605258286, "support": 5401}, "\u23ce": {"f1-score": 0.9535864978902954, "precision": 0.9112903225806451, "recall": 1.0, "support": 565}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 17}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 19}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9774011299435029, "precision": 0.9719101123595506, "recall": 0.9829545454545454, "support": 528}, "\u2423": {"f1-score": 0.9142556549184641, "precision": 0.9202259089304624, "recall": 0.9083623693379791, "support": 2870}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6722488038277512, "precision": 1.0, "recall": 0.5063063063063064, "support": 1110}, "macro avg": {"f1-score": 0.5415238618737686, "precision": 0.6803184608316453, "recall": 0.4576879797462951, "support": 16440}, "micro avg": {"f1-score": 0.7153245966214682, "precision": 0.9479020277052801, "recall": 0.5743917274939173, "support": 16440}, "weighted avg": {"f1-score": 0.6923729418751563, "precision": 0.8770720390932389, "recall": 0.5743917274939173, "support": 16440}, "\u2205": {"f1-score": 0.7639093317633207, "precision": 0.958802881950859, "recall": 0.634862385321101, "support": 8175}, "\u23ce": {"f1-score": 0.6869300911854104, "precision": 0.9112903225806451, "recall": 0.551219512195122, "support": 1025}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 488}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 744}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9393665158371041, "precision": 0.9719101123595506, "recall": 0.9089316987740805, "support": 571}, "\u2423": {"f1-score": 0.7282122905027933, "precision": 0.9202259089304624, "recall": 0.6024959556274555, "support": 4327}},
  "ppcr": 0.6059610705596107
}
```
</details>
