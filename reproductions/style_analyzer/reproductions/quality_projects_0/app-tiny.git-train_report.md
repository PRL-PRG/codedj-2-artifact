# Train report for javascript / file:///tmp/top-repos-quality-repos-1802uxw5/app-tiny.git HEAD 4418dd28c63a28aaae194e214daa4f247884fec7

### Classification report

PPCR: 0.596

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.978| 0.999| 0.934| 0.988| 0.956| 5049| 5397| 0.936 |
| `'` | 1.000| 1.000| 0.941| 1.000| 0.970| 640| 680| 0.941 |
| `␣` | 1.000| 0.898| 0.121| 0.946| 0.216| 452| 3352| 0.135 |
| `⏎␣⁻␣⁻` | 0.973| 0.861| 0.711| 0.914| 0.822| 338| 409| 0.826 |
| `⏎` | 0.971| 0.930| 0.195| 0.950| 0.325| 143| 681| 0.210 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 15| 441| 0.034 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 186| 0.011 |
| `macro avg` | 0.703| 0.670| 0.415| 0.685| 0.470| 6639| 11146| 0.596 |
| `micro avg` | 0.981| 0.981| 0.584| 0.981| 0.732| 6639| 11146| 0.596 |
| `weighted avg` | 0.979| 0.981| 0.584| 0.979| 0.637| 6639| 11146| 0.596 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|348 |5043 |0 |0 |0 |0 |6 |0 |
|2900 |40 |406 |0 |4 |0 |2 |0 |
|40 |0 |0 |640 |0 |0 |0 |0 |
|538 |10 |0 |0 |133 |0 |0 |0 |
|426 |15 |0 |0 |0 |0 |0 |0 |
|71 |47 |0 |0 |0 |0 |291 |0 |
|184 |2 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| lib/tiny/new-pedido.js | 11 |
| lib/ecomplus/save-products-db.js | 11 |
| lib/ecomplus/create-variations.js | 10 |
| routes/ecom/webhook.js | 10 |
| lib/tiny/new-produto.js | 9 |
| lib/ecomplus/save-orders-db.js | 9 |
| lib/map-stores.js | 8 |
| routes/api/products/tiny/create.js | 8 |
| routes/api/products/ecom/create.js | 8 |
| routes/api/orders/create.js | 8 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 640}, "macro avg": {"f1-score": 0.6854695610931439, "precision": 0.7031344559080289, "recall": 0.6697226299997292, "support": 6639}, "micro avg": {"f1-score": 0.9810212381382738, "precision": 0.9810212381382738, "recall": 0.9810212381382738, "support": 6639}, "weighted avg": {"f1-score": 0.9793746267408394, "precision": 0.9786366582033968, "recall": 0.9810212381382738, "support": 6639}, "\u2205": {"f1-score": 0.9882422104644326, "precision": 0.9778941244909831, "recall": 0.9988116458704694, "support": 5049}, "\u23ce": {"f1-score": 0.9500000000000001, "precision": 0.9708029197080292, "recall": 0.9300699300699301, "support": 143}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 15}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.913657770800628, "precision": 0.9732441471571907, "recall": 0.8609467455621301, "support": 338}, "\u2423": {"f1-score": 0.9463869463869463, "precision": 1.0, "recall": 0.8982300884955752, "support": 452}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9696969696969697, "precision": 1.0, "recall": 0.9411764705882353, "support": 680}, "macro avg": {"f1-score": 0.4698061777273487, "precision": 0.7031344559080289, "recall": 0.41478552340788183, "support": 11146}, "micro avg": {"f1-score": 0.7324149564239528, "precision": 0.9810212381382738, "recall": 0.5843351875112148, "support": 11146}, "weighted avg": {"f1-score": 0.6369109178608964, "precision": 0.9302770710915391, "recall": 0.5843351875112148, "support": 11146}, "\u2205": {"f1-score": 0.9556566230812963, "precision": 0.9778941244909831, "recall": 0.934408004446915, "support": 5397}, "\u23ce": {"f1-score": 0.3251833740831296, "precision": 0.9708029197080292, "recall": 0.19530102790014683, "support": 681}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 186}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 441}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8220338983050847, "precision": 0.9732441471571907, "recall": 0.7114914425427873, "support": 409}, "\u2423": {"f1-score": 0.2160723789249601, "precision": 1.0, "recall": 0.12112171837708831, "support": 3352}},
  "ppcr": 0.595639691369101
}
```
</details>
