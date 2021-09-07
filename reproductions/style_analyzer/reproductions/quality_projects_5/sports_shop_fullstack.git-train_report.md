# Train report for javascript / file:///tmp/top-repos-quality-repos-53piskwu/sports_shop_fullstack.git HEAD aaaf7d51e2998a0522951db75fa1e01d14e9e063

### Classification report

PPCR: 0.359

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.959| 1.000| 0.575| 0.979| 0.719| 19516| 33923| 0.575 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 557| 15240| 0.037 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 285| 3859| 0.074 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 3620| 0.000 |
| `micro avg` | 0.959| 0.959| 0.345| 0.959| 0.507| 20358| 56642| 0.359 |
| `macro avg` | 0.240| 0.250| 0.144| 0.245| 0.180| 20358| 56642| 0.359 |
| `weighted avg` | 0.919| 0.959| 0.345| 0.938| 0.431| 20358| 56642| 0.359 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|14407 |19516 |0 |0 |0 |
|14683 |557 |0 |0 |0 |
|3620 |0 |0 |0 |0 |
|3574 |285 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| finished-application/frontend/components/CreateItem.js | 29 |
| stepped-solutions/19/frontend/components/CreateItem.js | 28 |
| stepped-solutions/20/frontend/components/UpdateItem.js | 23 |
| finished-application/frontend/components/UpdateItem.js | 23 |
| finished-application/frontend/components/Signup.js | 21 |
| stepped-solutions/64/frontend/components/Signup.js | 21 |
| stepped-solutions/18/frontend/components/CreateItem.js | 21 |
| stepped-solutions/28/frontend/components/Signup.js | 21 |
| stepped-solutions/31/frontend/components/Reset.js | 18 |
| finished-application/frontend/components/Reset.js | 18 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.24472087074283994, "precision": 0.2396600844876707, "recall": 0.25, "support": 20358}, "micro avg": {"f1-score": 0.9586403379506828, "precision": 0.9586403379506828, "recall": 0.9586403379506828, "support": 20358}, "weighted avg": {"f1-score": 0.9383971929300057, "precision": 0.9189912975461992, "recall": 0.9586403379506828, "support": 20358}, "\u2205": {"f1-score": 0.9788834829713597, "precision": 0.9586403379506828, "recall": 1.0, "support": 19516}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 285}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 557}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3620}, "macro avg": {"f1-score": 0.1797682430316317, "precision": 0.2396600844876707, "recall": 0.14382572296082305, "support": 56642}, "micro avg": {"f1-score": 0.506909090909091, "precision": 0.9586403379506828, "recall": 0.3445499805797818, "support": 56642}, "weighted avg": {"f1-score": 0.4306541512207932, "precision": 0.5741314957858306, "recall": 0.3445499805797818, "support": 56642}, "\u2205": {"f1-score": 0.7190729721265268, "precision": 0.9586403379506828, "recall": 0.5753028918432922, "support": 33923}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3859}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 15240}},
  "ppcr": 0.3594152748843614
}
```
</details>
