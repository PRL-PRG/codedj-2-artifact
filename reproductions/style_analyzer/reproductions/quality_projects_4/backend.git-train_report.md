# Train report for javascript / file:///tmp/top-repos-quality-repos-uig6i8gh/backend.git HEAD ba9850bc7307ab31fcc2135df44e9408aa89737b

### Classification report

PPCR: 0.764

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.951| 0.996| 0.940| 0.973| 0.946| 8289| 8779| 0.944 |
| `␣` | 0.968| 0.895| 0.597| 0.930| 0.738| 2818| 4226| 0.667 |
| `'` | 1.000| 1.000| 0.994| 1.000| 0.997| 1737| 1747| 0.994 |
| `⏎␣⁺␣⁺` | 0.905| 0.749| 0.633| 0.819| 0.745| 545| 645| 0.845 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 47| 501| 0.094 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 29| 1262| 0.023 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 11| 94| 0.117 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 381| 0.003 |
| `micro avg` | 0.959| 0.959| 0.733| 0.959| 0.831| 13477| 17635| 0.764 |
| `weighted avg` | 0.953| 0.959| 0.733| 0.955| 0.774| 13477| 17635| 0.764 |
| `macro avg` | 0.478| 0.455| 0.396| 0.465| 0.428| 13477| 17635| 0.764 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|490 |8255 |34 |0 |0 |0 |0 |0 |0 |
|1408 |254 |2523 |0 |0 |41 |0 |0 |0 |
|10 |0 |0 |1737 |0 |0 |0 |0 |0 |
|1233 |19 |8 |0 |0 |2 |0 |0 |0 |
|100 |99 |38 |0 |0 |408 |0 |0 |0 |
|454 |43 |4 |0 |0 |0 |0 |0 |0 |
|380 |1 |0 |0 |0 |0 |0 |0 |0 |
|83 |11 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| routes/messages/commands/get_summary.js | 36 |
| middleware/webhooks.js | 28 |
| routes/messages/commands/pick_category.js | 21 |
| routes/messages/commands/get_started.js | 20 |
| routes/billing/index.js | 20 |
| models/migrations/20190812182330_initial_setup.js | 19 |
| routes/books/helpers/getRating.js | 18 |
| models/db/pages.js | 16 |
| models/migrations/20190903122821_categories.js | 15 |
| classes/Message.js | 15 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1737}, "macro avg": {"f1-score": 0.46528142145739104, "precision": 0.47790664494973406, "recall": 0.45497973229339356, "support": 13477}, "micro avg": {"f1-score": 0.9588929286933294, "precision": 0.9588929286933294, "recall": 0.9588929286933294, "support": 13477}, "weighted avg": {"f1-score": 0.954847023263684, "precision": 0.9526280096334518, "recall": 0.9588929286933294, "support": 13477}, "\u2205": {"f1-score": 0.9728360143774674, "precision": 0.9508177839207556, "recall": 0.9958981783086017, "support": 8289}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 29}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8192771084337348, "precision": 0.9046563192904656, "recall": 0.7486238532110092, "support": 545}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 47}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u2423": {"f1-score": 0.9301382488479262, "precision": 0.9677790563866513, "recall": 0.8953158268275373, "support": 2818}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9971297359357061, "precision": 1.0, "recall": 0.9942759015455066, "support": 1747}, "macro avg": {"f1-score": 0.4282082620455421, "precision": 0.47790664494973406, "recall": 0.39552057583636097, "support": 17635}, "micro avg": {"f1-score": 0.8307405502699924, "precision": 0.9588929286933294, "recall": 0.7328040827899064, "support": 17635}, "weighted avg": {"f1-score": 0.7736806726627291, "precision": 0.8374010174807288, "recall": 0.7328040827899064, "support": 17635}, "\u2205": {"f1-score": 0.945535765420079, "precision": 0.9508177839207556, "recall": 0.9403121084405969, "support": 8779}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1262}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 381}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7445255474452555, "precision": 0.9046563192904656, "recall": 0.6325581395348837, "support": 645}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 501}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 94}, "\u2423": {"f1-score": 0.7384750475632957, "precision": 0.9677790563866513, "recall": 0.5970184571699007, "support": 4226}},
  "ppcr": 0.7642188829033173
}
```
</details>
