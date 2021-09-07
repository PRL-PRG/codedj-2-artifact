# Train report for javascript / file:///tmp/top-repos-quality-repos-0vofpx0m/contact-list-app.git HEAD d2d9c26d20f6e8242f8ada0803628a01ea89e263

### Classification report

PPCR: 0.288

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.970| 1.000| 0.487| 0.985| 0.649| 615| 1262| 0.487 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 553| 0.014 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 105| 0.057 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 106| 0.047 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 174| 0.000 |
| `micro avg` | 0.970| 0.970| 0.280| 0.970| 0.434| 634| 2200| 0.288 |
| `weighted avg` | 0.941| 0.970| 0.280| 0.955| 0.372| 634| 2200| 0.288 |
| `macro avg` | 0.194| 0.200| 0.097| 0.197| 0.130| 634| 2200| 0.288 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|647 |615 |0 |0 |0 |0 |
|545 |8 |0 |0 |0 |0 |
|174 |0 |0 |0 |0 |0 |
|101 |5 |0 |0 |0 |0 |
|99 |6 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/utils/ContactsAPI.js | 6 |
| src/registerServiceWorker.js | 5 |
| src/App.js | 4 |
| src/CreateContact.js | 2 |
| src/index.js | 1 |
| src/ImageInput.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.19695756605284226, "precision": 0.19400630914826497, "recall": 0.2, "support": 634}, "micro avg": {"f1-score": 0.9700315457413249, "precision": 0.9700315457413249, "recall": 0.9700315457413249, "support": 634}, "weighted avg": {"f1-score": 0.9552752612184384, "precision": 0.9409611997333042, "recall": 0.9700315457413249, "support": 634}, "\u2205": {"f1-score": 0.9847878302642114, "precision": 0.9700315457413249, "recall": 1.0, "support": 615}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 174}, "macro avg": {"f1-score": 0.12974683544303794, "precision": 0.19400630914826497, "recall": 0.09746434231378764, "support": 2200}, "micro avg": {"f1-score": 0.434015525758645, "precision": 0.9700315457413249, "recall": 0.27954545454545454, "support": 2200}, "weighted avg": {"f1-score": 0.3721375143843498, "precision": 0.5564453685116145, "recall": 0.27954545454545454, "support": 2200}, "\u2205": {"f1-score": 0.6487341772151898, "precision": 0.9700315457413249, "recall": 0.4873217115689382, "support": 1262}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 106}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 105}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 553}},
  "ppcr": 0.2881818181818182
}
```
</details>
