# Train report for javascript / file:///tmp/top-repos-quality-repos-5n14cu3_/todo-web.git HEAD 2c05c6919a24b3e8ca27a0604c85a084fbfc86b4

### Classification report

PPCR: 0.467

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.924| 1.000| 0.635| 0.961| 0.753| 1639| 2583| 0.635 |
| `'` | 1.000| 1.000| 1.000| 1.000| 1.000| 676| 676| 1.000 |
| `␣` | 0.987| 0.687| 0.107| 0.810| 0.193| 227| 1458| 0.156 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 35| 192| 0.182 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 17| 349| 0.049 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 13| 184| 0.071 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 142| 0.000 |
| `macro avg` | 0.416| 0.384| 0.249| 0.396| 0.278| 2607| 5584| 0.467 |
| `micro avg` | 0.948| 0.948| 0.443| 0.948| 0.603| 2607| 5584| 0.467 |
| `weighted avg` | 0.926| 0.948| 0.443| 0.934| 0.520| 2607| 5584| 0.467 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|944 |1639 |0 |0 |0 |0 |0 |0 |
|1231 |71 |156 |0 |0 |0 |0 |0 |
|0 |0 |0 |676 |0 |0 |0 |0 |
|332 |17 |0 |0 |0 |0 |0 |0 |
|157 |33 |2 |0 |0 |0 |0 |0 |
|171 |13 |0 |0 |0 |0 |0 |0 |
|142 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/serviceWorker.js | 34 |
| src/TodoApp/index.js | 28 |
| src/TodoApp/TodoApp.test.js | 14 |
| cypress/integration/todo.spec.js | 12 |
| src/TodoApp/TodoList/TodoItem/TodoItem.test.js | 9 |
| src/TodoApp/Navbar/StatusTabs/index.js | 8 |
| src/TodoApp/TodoList/TodoItem/index.js | 7 |
| src/TodoApp/TodoList/index.js | 6 |
| src/TodoApp/Navbar/index.js | 4 |
| src/TodoApp/Navbar/ProgressBar/index.js | 4 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 676}, "macro avg": {"f1-score": 0.3958737795448564, "precision": 0.41596623656637793, "recall": 0.3838892385147892, "support": 2607}, "micro avg": {"f1-score": 0.9478327579593402, "precision": 0.9478327579593402, "recall": 0.9478327579593402, "support": 2607}, "weighted avg": {"f1-score": 0.933866414594556, "precision": 0.9264495780006033, "recall": 0.9478327579593402, "support": 2607}, "\u2205": {"f1-score": 0.9607268464243846, "precision": 0.9244218838127467, "recall": 1.0, "support": 1639}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 17}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 35}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 13}, "\u2423": {"f1-score": 0.8103896103896104, "precision": 0.9873417721518988, "recall": 0.6872246696035242, "support": 227}},
  "cl_report_full": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 676}, "macro avg": {"f1-score": 0.27794207992227793, "precision": 0.41596623656637793, "recall": 0.24878991042366963, "support": 5584}, "micro avg": {"f1-score": 0.6033451349041632, "precision": 0.9478327579593402, "recall": 0.44251432664756446, "support": 5584}, "weighted avg": {"f1-score": 0.5195680116005871, "precision": 0.8064695611901492, "recall": 0.44251432664756446, "support": 5584}, "\u2205": {"f1-score": 0.7525252525252526, "precision": 0.9244218838127467, "recall": 0.6345334881920248, "support": 2583}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 349}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 142}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 192}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 184}, "\u2423": {"f1-score": 0.19306930693069307, "precision": 0.9873417721518988, "recall": 0.10699588477366255, "support": 1458}},
  "ppcr": 0.4668696275071633
}
```
</details>
