# Train report for javascript / file:///tmp/top-repos-quality-repos-c6etyd31/be-nc-news.git HEAD 2d0d7d666013cfbc44da40e51266ea7f439066f0

### Classification report

PPCR: 0.937

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.940| 1.000| 0.997| 0.969| 0.968| 7417| 7436| 0.997 |
| `␣` | 0.993| 0.915| 0.785| 0.953| 0.877| 2789| 3253| 0.857 |
| `⏎` | 0.970| 0.919| 0.907| 0.944| 0.938| 2143| 2172| 0.987 |
| `'` | 1.000| 1.000| 0.989| 1.000| 0.995| 1940| 1961| 0.989 |
| `⏎␣⁺␣⁺` | 0.983| 0.833| 0.765| 0.901| 0.860| 681| 741| 0.919 |
| `⏎␣⁻␣⁻` | 1.000| 0.938| 0.905| 0.968| 0.950| 568| 589| 0.964 |
| `"` | 1.000| 1.000| 0.526| 1.000| 0.689| 515| 979| 0.526 |
| `macro avg` | 0.984| 0.944| 0.839| 0.962| 0.897| 16053| 17131| 0.937 |
| `weighted avg` | 0.966| 0.965| 0.904| 0.965| 0.929| 16053| 17131| 0.937 |
| `micro avg` | 0.965| 0.965| 0.904| 0.965| 0.934| 16053| 17131| 0.937 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|19 |7416 |1 |0 |0 |0 |0 |0 |
|464 |185 |2552 |42 |0 |0 |10 |0 |
|29 |173 |0 |1970 |0 |0 |0 |0 |
|21 |0 |0 |0 |1940 |0 |0 |0 |
|464 |0 |0 |0 |0 |515 |0 |0 |
|60 |103 |11 |0 |0 |0 |567 |0 |
|21 |12 |5 |18 |0 |0 |0 |533 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| __tests__/app.test.js | 203 |
| models/comments.model.js | 47 |
| models/articles.model.js | 45 |
| __tests__/utils.test.js | 28 |
| controllers/comments.controller.js | 23 |
| db/utils/utils.js | 21 |
| controllers/articles.controller.js | 20 |
| errors/index.js | 19 |
| db/seeds/seed.js | 15 |
| routes/articles.router.js | 12 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 515}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1940}, "macro avg": {"f1-score": 0.9622046333393967, "precision": 0.9837911520524438, "recall": 0.9435914185151791, "support": 16053}, "micro avg": {"f1-score": 0.9651155547249736, "precision": 0.9651155547249736, "recall": 0.9651155547249736, "support": 16053}, "weighted avg": {"f1-score": 0.9646955261095072, "precision": 0.9664674210767669, "recall": 0.9651155547249736, "support": 16053}, "\u2205": {"f1-score": 0.9690317522540179, "precision": 0.9400430979845354, "recall": 0.9998651745988945, "support": 7417}, "\u23ce": {"f1-score": 0.9441648693985143, "precision": 0.9704433497536946, "recall": 0.919272048530098, "support": 2143}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9014308426073131, "precision": 0.9826689774696707, "recall": 0.8325991189427313, "support": 681}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9682107175295186, "precision": 1.0, "recall": 0.9383802816901409, "support": 568}, "\u2423": {"f1-score": 0.9525942515864128, "precision": 0.9933826391592059, "recall": 0.9150233058443886, "support": 2789}},
  "cl_report_full": {"\"": {"f1-score": 0.6894243641231593, "precision": 1.0, "recall": 0.526046986721144, "support": 979}, "\u0027": {"f1-score": 0.9946167649320686, "precision": 1.0, "recall": 0.9892911779704232, "support": 1961}, "macro avg": {"f1-score": 0.8966683651865158, "precision": 0.9837911520524438, "recall": 0.8391798714051637, "support": 17131}, "micro avg": {"f1-score": 0.9337632594021216, "precision": 0.9651155547249736, "recall": 0.9043838655069757, "support": 17131}, "weighted avg": {"f1-score": 0.9285930188126904, "precision": 0.9682210536318928, "recall": 0.9043838655069757, "support": 17131}, "\u2205": {"f1-score": 0.9678303425774877, "precision": 0.9400430979845354, "recall": 0.9973103819257665, "support": 7436}, "\u23ce": {"f1-score": 0.9376487386958591, "precision": 0.9704433497536946, "recall": 0.9069981583793738, "support": 2172}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8603945371775418, "precision": 0.9826689774696707, "recall": 0.7651821862348178, "support": 741}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9500891265597148, "precision": 1.0, "recall": 0.9049235993208828, "support": 589}, "\u2423": {"f1-score": 0.87667468223978, "precision": 0.9933826391592059, "recall": 0.784506609283738, "support": 3253}},
  "ppcr": 0.9370731422567276
}
```
</details>
