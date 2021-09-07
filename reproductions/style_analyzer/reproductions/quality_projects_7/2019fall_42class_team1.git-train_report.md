# Train report for javascript / file:///tmp/top-repos-quality-repos-0iacxk2d/2019fall_42class_team1.git HEAD 0433a527ae869f44b4f342d0075f7a5ab82471b5

### Classification report

PPCR: 0.800

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.952| 0.995| 0.957| 0.973| 0.954| 23714| 24650| 0.962 |
| `␣` | 0.956| 0.837| 0.372| 0.892| 0.535| 4133| 9310| 0.444 |
| `'` | 0.925| 0.988| 0.745| 0.956| 0.825| 2419| 3208| 0.754 |
| `⏎` | 0.922| 0.878| 0.675| 0.900| 0.779| 1809| 2353| 0.769 |
| `"` | 0.973| 0.836| 0.495| 0.899| 0.656| 1186| 2006| 0.591 |
| `⏎␣⁺␣⁺` | 0.975| 0.742| 0.644| 0.843| 0.775| 1124| 1296| 0.867 |
| `⏎␣⁻␣⁻` | 0.969| 0.835| 0.720| 0.897| 0.826| 1057| 1227| 0.861 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 66| 317| 0.208 |
| `micro avg` | 0.950| 0.950| 0.761| 0.950| 0.845| 35508| 44367| 0.800 |
| `weighted avg` | 0.949| 0.950| 0.761| 0.948| 0.819| 35508| 44367| 0.800 |
| `macro avg` | 0.834| 0.764| 0.576| 0.795| 0.669| 35508| 44367| 0.800 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|936 |23596 |87 |0 |1 |0 |10 |20 |0 |
|5177 |584 |3460 |0 |75 |0 |6 |8 |0 |
|789 |0 |0 |2391 |0 |28 |0 |0 |0 |
|544 |144 |71 |0 |1589 |0 |5 |0 |0 |
|820 |0 |0 |194 |0 |992 |0 |0 |0 |
|172 |281 |1 |0 |8 |0 |834 |0 |0 |
|170 |171 |2 |0 |1 |0 |0 |883 |0 |
|251 |16 |0 |0 |50 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/market/client/order/Order.js | 108 |
| src/market/config/config.js | 72 |
| src/market/server/controllers/order.controller.js | 71 |
| src/market/server/controllers/product.controller.js | 67 |
| src/market/client/order/ProductOrderEdit.js | 61 |
| src/market/server/controllers/user.controller.js | 60 |
| src/market/client/order/api-order.js | 50 |
| src/market/client/cart/CartItems.js | 48 |
| src/market/client/order/Valid.js | 47 |
| src/market/client/product/EditProduct.js | 45 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.8993653671804169, "precision": 0.9725490196078431, "recall": 0.836424957841484, "support": 1186}, "\u0027": {"f1-score": 0.9556354916067146, "precision": 0.9249516441005803, "recall": 0.9884249689954526, "support": 2419}, "macro avg": {"f1-score": 0.7950100633911774, "precision": 0.8338991645508858, "recall": 0.7641000177121872, "support": 35508}, "micro avg": {"f1-score": 0.9503492170778416, "precision": 0.9503492170778416, "recall": 0.9503492170778416, "support": 35508}, "weighted avg": {"f1-score": 0.9479975236695823, "precision": 0.949036553752699, "recall": 0.9503492170778416, "support": 35508}, "\u2205": {"f1-score": 0.9729105677648124, "precision": 0.951758631816715, "recall": 0.9950240364341739, "support": 23714}, "\u23ce": {"f1-score": 0.8995188225304274, "precision": 0.9216937354988399, "recall": 0.8783858485351023, "support": 1809}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 66}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8428499242041436, "precision": 0.9754385964912281, "recall": 0.7419928825622776, "support": 1124}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8973577235772358, "precision": 0.969264544456641, "recall": 0.8353831598864712, "support": 1057}, "\u2423": {"f1-score": 0.8924426102656694, "precision": 0.9555371444352388, "recall": 0.8371642874425357, "support": 4133}},
  "cl_report_full": {"\"": {"f1-score": 0.6556510244547257, "precision": 0.9725490196078431, "recall": 0.4945164506480558, "support": 2006}, "\u0027": {"f1-score": 0.825479026411186, "precision": 0.9249516441005803, "recall": 0.7453241895261845, "support": 3208}, "macro avg": {"f1-score": 0.6689654866964441, "precision": 0.8338991645508858, "recall": 0.5758991814116184, "support": 44367}, "micro avg": {"f1-score": 0.8449452269170578, "precision": 0.9503492170778416, "recall": 0.7605878242838146, "support": 44367}, "weighted avg": {"f1-score": 0.8187725090496336, "precision": 0.9443343176981044, "recall": 0.7605878242838146, "support": 44367}, "\u2205": {"f1-score": 0.9544921321952995, "precision": 0.951758631816715, "recall": 0.9572413793103448, "support": 24650}, "\u23ce": {"f1-score": 0.7794947265145941, "precision": 0.9216937354988399, "recall": 0.6753081172970675, "support": 2353}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 317}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7754532775453277, "precision": 0.9754385964912281, "recall": 0.6435185185185185, "support": 1296}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8260056127221703, "precision": 0.969264544456641, "recall": 0.7196414017929911, "support": 1227}, "\u2423": {"f1-score": 0.53514809372825, "precision": 0.9555371444352388, "recall": 0.3716433941997852, "support": 9310}},
  "ppcr": 0.8003245655554804
}
```
</details>
