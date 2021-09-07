# Train report for javascript / file:///tmp/top-repos-quality-repos-9il6yuvw/in4331-web-data-management.git HEAD 565314e978861aea62b3d2bd6e53d48d5e4f6c20

### Classification report

PPCR: 0.692

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 0.988| 0.794| 0.994| 0.885| 6735| 8376| 0.804 |
| `␣` | 0.971| 0.992| 0.825| 0.982| 0.892| 4272| 5138| 0.831 |
| `⏎` | 0.956| 0.981| 0.683| 0.968| 0.796| 686| 986| 0.696 |
| `"` | 1.000| 1.000| 0.381| 1.000| 0.552| 535| 1404| 0.381 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 29| 327| 0.089 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 216| 0.009 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 444| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 443| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 235| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 135| 0.000 |
| `macro avg` | 0.393| 0.396| 0.268| 0.394| 0.313| 12259| 17704| 0.692 |
| `micro avg` | 0.987| 0.987| 0.684| 0.987| 0.808| 12259| 17704| 0.692 |
| `weighted avg` | 0.985| 0.987| 0.684| 0.986| 0.766| 12259| 17704| 0.692 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| '| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1641 |6654 |81 |0 |0 |0 |0 |0 |0 |0 |0 |
|866 |2 |4239 |0 |31 |0 |0 |0 |0 |0 |0 |
|869 |0 |0 |535 |0 |0 |0 |0 |0 |0 |0 |
|300 |0 |13 |0 |673 |0 |0 |0 |0 |0 |0 |
|444 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|443 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|298 |0 |29 |0 |0 |0 |0 |0 |0 |0 |0 |
|235 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|214 |0 |2 |0 |0 |0 |0 |0 |0 |0 |0 |
|135 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| postgres/saga/functions/order-checkout.js | 10 |
| saga/functions/order-checkout.js | 10 |
| saga/functions/order-remove.js | 10 |
| saga/functions/cancel-payment.js | 8 |
| postgres/stock/functions/stock-subtract.js | 8 |
| postgres/users/functions/credit-subtract.js | 8 |
| postgres/users/functions/credit-add.js | 8 |
| users/functions/credit-add.js | 6 |
| postgres/saga/functions/order-remove.js | 6 |
| users/functions/credit-subtract.js | 6 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 535}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.3943851623745688, "precision": 0.39270219819422103, "recall": 0.3961298117523186, "support": 12259}, "micro avg": {"f1-score": 0.9871115099110858, "precision": 0.9871115099110858, "recall": 0.9871115099110858, "support": 12259}, "weighted avg": {"f1-score": 0.9859190518639646, "precision": 0.984860435004691, "recall": 0.9871115099110858, "support": 12259}, "\u2205": {"f1-score": 0.9938018071839295, "precision": 0.9996995192307693, "recall": 0.9879732739420936, "support": 6735}, "\u23ce": {"f1-score": 0.9683453237410073, "precision": 0.9559659090909091, "recall": 0.9810495626822158, "support": 686}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 29}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9817044928207503, "precision": 0.9713565536205316, "recall": 0.9922752808988764, "support": 4272}},
  "cl_report_full": {"\"": {"f1-score": 0.5518308406395048, "precision": 1.0, "recall": 0.38105413105413105, "support": 1404}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 135}, "macro avg": {"f1-score": 0.31258250946586386, "precision": 0.39270219819422103, "recall": 0.2683051713676054, "support": 17704}, "micro avg": {"f1-score": 0.807729533090812, "precision": 0.9871115099110858, "recall": 0.6835178490736556, "support": 17704}, "weighted avg": {"f1-score": 0.765913460866795, "precision": 0.8874206694500029, "recall": 0.6835178490736556, "support": 17704}, "\u2205": {"f1-score": 0.8853113358169239, "precision": 0.9996995192307693, "recall": 0.7944126074498568, "support": 8376}, "\u23ce": {"f1-score": 0.7964497041420119, "precision": 0.9559659090909091, "recall": 0.6825557809330629, "support": 986}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 327}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 444}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 216}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 443}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 235}, "\u2423": {"f1-score": 0.8922332140601978, "precision": 0.9713565536205316, "recall": 0.8250291942390034, "support": 5138}},
  "ppcr": 0.6924423859014912
}
```
</details>
