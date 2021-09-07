# Train report for javascript / file:///tmp/top-repos-quality-repos-xd9j98gp/crypto_basket_fe.git HEAD ef0ee505b80546a0087899dca26ce3ff83438de7

### Classification report

PPCR: 0.884

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.969| 0.989| 0.971| 0.979| 0.970| 49397| 50301| 0.982 |
| `␣` | 0.948| 0.959| 0.867| 0.953| 0.906| 19104| 21138| 0.904 |
| `"` | 0.991| 1.000| 0.855| 0.996| 0.918| 2751| 3217| 0.855 |
| `⏎␣⁻␣⁻` | 0.944| 0.700| 0.686| 0.804| 0.795| 2699| 2754| 0.980 |
| `⏎` | 0.897| 0.842| 0.445| 0.869| 0.595| 2654| 5022| 0.528 |
| `'` | 1.000| 0.987| 0.436| 0.994| 0.607| 1887| 4272| 0.442 |
| `⏎␣⁺␣⁺` | 0.932| 0.742| 0.431| 0.826| 0.590| 1607| 2765| 0.581 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 53| 1208| 0.044 |
| `micro avg` | 0.962| 0.962| 0.850| 0.962| 0.903| 80152| 90677| 0.884 |
| `weighted avg` | 0.961| 0.962| 0.850| 0.961| 0.886| 80152| 90677| 0.884 |
| `macro avg` | 0.835| 0.778| 0.587| 0.803| 0.673| 80152| 90677| 0.884 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|904 |48859 |371 |12 |0 |0 |88 |67 |0 |
|2034 |507 |18329 |226 |0 |0 |23 |19 |0 |
|2368 |189 |228 |2235 |0 |0 |1 |1 |0 |
|2385 |0 |0 |0 |1863 |24 |0 |0 |0 |
|466 |0 |0 |0 |0 |2751 |0 |0 |0 |
|55 |587 |221 |1 |0 |0 |1890 |0 |0 |
|1158 |237 |176 |0 |0 |0 |1 |1193 |0 |
|1155 |18 |18 |17 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| app/containers/UserDashboard/components/Modals/index.js | 90 |
| app/containers/UserDashboard/saga.js | 86 |
| app/containers/AdminDashboard/components/User/components/Activity/index.js | 75 |
| app/containers/AdminDashboard/components/Asset/index.js | 73 |
| app/containers/UserDashboard/components/Activity/index.js | 62 |
| app/containers/AdminDashboard/components/Activity/components/TradeView.js | 62 |
| app/containers/App/reducer.js | 58 |
| app/containers/AdminDashboard/components/User/index.js | 57 |
| app/containers/UserDashboard/UserDashboard.js | 53 |
| app/containers/AdminDashboard/components/Activity/components/TransferView.js | 53 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9956568946796959, "precision": 0.9913513513513513, "recall": 1.0, "support": 2751}, "\u0027": {"f1-score": 0.9936, "precision": 1.0, "recall": 0.9872813990461049, "support": 1887}, "macro avg": {"f1-score": 0.802637975234026, "precision": 0.8351571872679063, "recall": 0.7775730223292991, "support": 80152}, "micro avg": {"f1-score": 0.9621718734404631, "precision": 0.9621718734404631, "recall": 0.9621718734404631, "support": 80152}, "weighted avg": {"f1-score": 0.9607013353122035, "precision": 0.9610741059685457, "recall": 0.9621718734404631, "support": 80152}, "\u2205": {"f1-score": 0.9791971461210093, "precision": 0.9694823104549875, "recall": 0.9891086503228941, "support": 49397}, "\u23ce": {"f1-score": 0.8688046647230321, "precision": 0.8972300281011641, "recall": 0.8421250941974379, "support": 2654}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 53}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8264634568756494, "precision": 0.93203125, "recall": 0.7423771001866832, "support": 1607}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.803913228413441, "precision": 0.9435846230654019, "recall": 0.700259355316784, "support": 2699}, "\u2423": {"f1-score": 0.9534684110593805, "precision": 0.9475779351703458, "recall": 0.9594325795644891, "support": 19104}},
  "cl_report_full": {"\"": {"f1-score": 0.9182242990654206, "precision": 0.9913513513513513, "recall": 0.8551445446067765, "support": 3217}, "\u0027": {"f1-score": 0.6073349633251833, "precision": 1.0, "recall": 0.43609550561797755, "support": 4272}, "macro avg": {"f1-score": 0.672622208191352, "precision": 0.8351571872679063, "recall": 0.5865581318918209, "support": 90677}, "micro avg": {"f1-score": 0.9028911952888561, "precision": 0.9621718734404631, "recall": 0.8504913042998776, "support": 90677}, "weighted avg": {"f1-score": 0.8856701244616404, "precision": 0.9477441583689421, "recall": 0.8504913042998776, "support": 90677}, "\u2205": {"f1-score": 0.9704065621958728, "precision": 0.9694823104549875, "recall": 0.9713325778811555, "support": 50301}, "\u23ce": {"f1-score": 0.5949687208838014, "precision": 0.8972300281011641, "recall": 0.44504181600955794, "support": 5022}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1208}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.5898640296662546, "precision": 0.93203125, "recall": 0.4314647377938517, "support": 2765}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7946184570107211, "precision": 0.9435846230654019, "recall": 0.6862745098039216, "support": 2754}, "\u2423": {"f1-score": 0.9055606333835627, "precision": 0.9475779351703458, "recall": 0.8671113634213266, "support": 21138}},
  "ppcr": 0.88392866989424
}
```
</details>
