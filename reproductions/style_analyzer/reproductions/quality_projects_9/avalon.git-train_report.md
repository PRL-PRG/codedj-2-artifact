# Train report for javascript / file:///tmp/top-repos-quality-repos-j193rcti/avalon.git HEAD f8e2d458993647a98853a84222b480012ae16e39

### Classification report

PPCR: 0.710

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.970| 0.997| 0.946| 0.983| 0.958| 8184| 8619| 0.950 |
| `␣` | 0.971| 0.978| 0.497| 0.975| 0.658| 2651| 5216| 0.508 |
| `⏎` | 0.979| 0.805| 0.307| 0.884| 0.467| 533| 1399| 0.381 |
| `⏎␣⁻␣⁻` | 0.908| 0.721| 0.659| 0.804| 0.764| 517| 566| 0.913 |
| `'` | 1.000| 1.000| 0.628| 1.000| 0.771| 312| 497| 0.628 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 25| 609| 0.041 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 18| 336| 0.054 |
| `weighted avg` | 0.965| 0.969| 0.688| 0.966| 0.763| 12240| 17242| 0.710 |
| `macro avg` | 0.690| 0.643| 0.434| 0.664| 0.517| 12240| 17242| 0.710 |
| `micro avg` | 0.969| 0.969| 0.688| 0.969| 0.805| 12240| 17242| 0.710 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|435 |8157 |27 |0 |0 |0 |0 |0 |
|2565 |55 |2594 |0 |0 |0 |2 |0 |
|866 |35 |33 |429 |0 |0 |36 |0 |
|185 |0 |0 |0 |312 |0 |0 |0 |
|584 |23 |2 |0 |0 |0 |0 |0 |
|49 |127 |15 |2 |0 |0 |373 |0 |
|318 |10 |1 |7 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/phina/display/dragon/Dragon.js | 51 |
| src/phina/display/scene/MainScene.js | 49 |
| api/Node/server.js | 45 |
| src/phina/display/scene/BaseScene.js | 29 |
| src/store/mutations.js | 27 |
| src/store/getters.js | 25 |
| src/phina/display/scene/InputScene.js | 24 |
| src/store/actions.js | 18 |
| src/mixins/controller.js | 9 |
| src/phina/display/ComboEffect.js | 9 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 312}, "macro avg": {"f1-score": 0.6636352859967145, "precision": 0.689723699039728, "recall": 0.6430782325188195, "support": 12240}, "micro avg": {"f1-score": 0.9693627450980392, "precision": 0.9693627450980392, "recall": 0.9693627450980392, "support": 12240}, "weighted avg": {"f1-score": 0.966479221882705, "precision": 0.9654813612052803, "recall": 0.9693627450980392, "support": 12240}, "\u2205": {"f1-score": 0.983304201072871, "precision": 0.9702628761746164, "recall": 0.9967008797653959, "support": 8184}, "\u23ce": {"f1-score": 0.8836251287332648, "precision": 0.9794520547945206, "recall": 0.8048780487804879, "support": 533}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 18}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 25}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8038793103448276, "precision": 0.9075425790754258, "recall": 0.7214700193423598, "support": 517}, "\u2423": {"f1-score": 0.974638361826038, "precision": 0.9708083832335329, "recall": 0.978498679743493, "support": 2651}},
  "cl_report_full": {"\u0027": {"f1-score": 0.7713226205191595, "precision": 1.0, "recall": 0.6277665995975855, "support": 497}, "macro avg": {"f1-score": 0.5168342754851134, "precision": 0.689723699039728, "recall": 0.43387689293797177, "support": 17242}, "micro avg": {"f1-score": 0.8048979038057119, "precision": 0.9693627450980392, "recall": 0.6881452267718362, "support": 17242}, "weighted avg": {"f1-score": 0.7631435015538428, "precision": 0.9167935727357239, "recall": 0.6881452267718362, "support": 17242}, "\u2205": {"f1-score": 0.9581816046047222, "precision": 0.9702628761746164, "recall": 0.9463974939088061, "support": 8619}, "\u23ce": {"f1-score": 0.46706586826347307, "precision": 0.9794520547945206, "recall": 0.30664760543245173, "support": 1399}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 336}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 609}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7635619242579325, "precision": 0.9075425790754258, "recall": 0.6590106007067138, "support": 566}, "\u2423": {"f1-score": 0.657707910750507, "precision": 0.9708083832335329, "recall": 0.4973159509202454, "support": 5216}},
  "ppcr": 0.7098944438000232
}
```
</details>
