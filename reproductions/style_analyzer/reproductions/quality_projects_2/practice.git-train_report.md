# Train report for javascript / file:///tmp/top-repos-quality-repos-rneukp2u/practice.git HEAD 29be9fe98effacea67d6cd08f6a8fb35a6a4e58b

### Classification report

PPCR: 0.609

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.975| 0.996| 0.909| 0.985| 0.941| 10949| 12000| 0.912 |
| `␣` | 0.938| 0.923| 0.361| 0.931| 0.522| 2244| 5733| 0.391 |
| `"` | 1.000| 1.000| 0.264| 1.000| 0.417| 435| 1649| 0.264 |
| `⏎⏎` | 0.891| 0.931| 0.296| 0.911| 0.445| 175| 550| 0.318 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 130| 1270| 0.102 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 22| 67| 0.328 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 20| 766| 0.026 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 17| 777| 0.022 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 12| 77| 0.156 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 112| 0.000 |
| `weighted avg` | 0.955| 0.969| 0.590| 0.962| 0.661| 14004| 23001| 0.609 |
| `micro avg` | 0.969| 0.969| 0.590| 0.969| 0.734| 14004| 23001| 0.609 |
| `macro avg` | 0.380| 0.385| 0.183| 0.383| 0.232| 14004| 23001| 0.609 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1051 |10903 |46 |0 |0 |0 |0 |0 |0 |0 |0 |
|3489 |171 |2072 |0 |0 |0 |0 |1 |0 |0 |0 |
|1214 |0 |0 |435 |0 |0 |0 |0 |0 |0 |0 |
|1140 |41 |70 |0 |0 |0 |0 |19 |0 |0 |0 |
|760 |14 |3 |0 |0 |0 |0 |0 |0 |0 |0 |
|746 |13 |7 |0 |0 |0 |0 |0 |0 |0 |0 |
|375 |2 |10 |0 |0 |0 |0 |163 |0 |0 |0 |
|112 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|65 |12 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|45 |22 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| Web/tic-tac-toe/src/index.js | 51 |
| JavaScript/Bootcamp/OOP/person.js | 22 |
| JavaScript/Bootcamp/Projects/todo-old/scripts/uuid.js | 22 |
| JavaScript/React/base-syntax--assignment-problem/src/registerServiceWorker.js | 22 |
| JavaScript/React/lists-conditionals--assignment-problem/src/registerServiceWorker.js | 22 |
| JavaScript/Bootcamp/Projects/notes-old/scripts/uuid.js | 22 |
| JavaScript/Bootcamp/Projects/hangman-old/scripts/hangman.js | 17 |
| JavaScript/Bootcamp/Projects/hangman/src/hangman.js | 17 |
| JavaScript/React/lists-conditionals--assignment-problem/src/App.js | 12 |
| JavaScript/Bootcamp/Projects/notes/src/notes.js | 12 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 435}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.38269249720082704, "precision": 0.38045142830325374, "recall": 0.38505784331517545, "support": 14004}, "micro avg": {"f1-score": 0.9692230791202514, "precision": 0.9692230791202514, "recall": 0.9692230791202514, "support": 14004}, "weighted avg": {"f1-score": 0.9621017801389831, "precision": 0.9551764324445124, "recall": 0.9692230791202514, "support": 14004}, "\u2205": {"f1-score": 0.9854928368057125, "precision": 0.975398103417427, "recall": 0.9957987030779066, "support": 10949}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 130}, "\u23ce\u23ce": {"f1-score": 0.9106145251396648, "precision": 0.8907103825136612, "recall": 0.9314285714285714, "support": 175}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 17}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 20}, "\u2423": {"f1-score": 0.930817610062893, "precision": 0.9384057971014492, "recall": 0.9233511586452763, "support": 2244}},
  "cl_report_full": {"\"": {"f1-score": 0.4174664107485605, "precision": 1.0, "recall": 0.26379624014554276, "support": 1649}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 112}, "macro avg": {"f1-score": 0.23248685936371388, "precision": 0.38045142830325374, "recall": 0.1830159571258874, "support": 23001}, "micro avg": {"f1-score": 0.7335765437103094, "precision": 0.9692230791202514, "recall": 0.5901047780531281, "support": 23001}, "weighted avg": {"f1-score": 0.6614687515780058, "precision": 0.8357701137417611, "recall": 0.5901047780531281, "support": 23001}, "\u2205": {"f1-score": 0.940805936664078, "precision": 0.975398103417427, "recall": 0.9085833333333333, "support": 12000}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1270}, "\u23ce\u23ce": {"f1-score": 0.44474761255115963, "precision": 0.8907103825136612, "recall": 0.2963636363636364, "support": 550}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 77}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 777}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 67}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 766}, "\u2423": {"f1-score": 0.5218486336733409, "precision": 0.9384057971014492, "recall": 0.3614163614163614, "support": 5733}},
  "ppcr": 0.6088430937785314
}
```
</details>
