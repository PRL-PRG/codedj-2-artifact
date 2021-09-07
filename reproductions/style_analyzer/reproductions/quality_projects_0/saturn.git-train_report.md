# Train report for javascript / file:///tmp/top-repos-quality-repos-j608t2um/saturn.git HEAD 610b59f2b2758406cd58b8b7b5bd7106b86e720d

### Classification report

PPCR: 0.654

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `␣` | 0.936| 0.993| 0.606| 0.964| 0.736| 3124| 5116| 0.611 |
| `∅` | 0.992| 0.943| 0.592| 0.967| 0.741| 3018| 4811| 0.627 |
| `'` | 1.000| 1.000| 0.994| 1.000| 0.997| 2738| 2754| 0.994 |
| `⏎` | 0.980| 0.967| 0.588| 0.973| 0.735| 398| 655| 0.608 |
| `⏎⏎` | 0.973| 1.000| 0.528| 0.986| 0.684| 219| 415| 0.528 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 31| 413| 0.075 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 11| 416| 0.026 |
| `macro avg` | 0.697| 0.701| 0.473| 0.699| 0.556| 9539| 14580| 0.654 |
| `micro avg` | 0.974| 0.974| 0.637| 0.974| 0.770| 9539| 14580| 0.654 |
| `weighted avg` | 0.971| 0.974| 0.637| 0.972| 0.744| 9539| 14580| 0.654 |

### Confusion matrix

|refusal|  ␣| ∅| '| ⏎| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1992 |3102 |22 |0 |0 |0 |0 |0 |
|1793 |159 |2847 |0 |8 |4 |0 |0 |
|16 |0 |0 |2738 |0 |0 |0 |0 |
|257 |11 |0 |0 |385 |2 |0 |0 |
|196 |0 |0 |0 |0 |219 |0 |0 |
|382 |31 |0 |0 |0 |0 |0 |0 |
|405 |11 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/components/Dialog/Dialog.js | 63 |
| src/theme/Theme.js | 34 |
| src/utils/optionsSelect/optionsSelect.js | 31 |
| src/utils/objectManipulation/index.js | 20 |
| src/icons/index.js | 11 |
| src/utils/debounce/index.js | 8 |
| .storybook/preview.js | 8 |
| src/utils/outerWidth/index.js | 7 |
| rollup.config.js | 7 |
| src/components/Tabs/index.js | 4 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 2738}, "macro avg": {"f1-score": 0.6986867174057789, "precision": 0.697333984312278, "recall": 0.7005191985906468, "support": 9539}, "micro avg": {"f1-score": 0.9740014676590838, "precision": 0.9740014676590838, "recall": 0.9740014676590838, "support": 9539}, "weighted avg": {"f1-score": 0.971903206736731, "precision": 0.9707590058207414, "recall": 0.9740014676590838, "support": 9539}, "\u2205": {"f1-score": 0.9672158994394429, "precision": 0.9923318229348205, "recall": 0.9433399602385686, "support": 3018}, "\u23ce": {"f1-score": 0.9734513274336284, "precision": 0.9796437659033079, "recall": 0.9673366834170855, "support": 398}, "\u23ce\u23ce": {"f1-score": 0.9864864864864865, "precision": 0.9733333333333334, "recall": 1.0, "support": 219}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 31}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u2423": {"f1-score": 0.9636533084808946, "precision": 0.936028968014484, "recall": 0.9929577464788732, "support": 3124}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9970866715222142, "precision": 1.0, "recall": 0.9941902687000727, "support": 2754}, "macro avg": {"f1-score": 0.5562205437782737, "precision": 0.697333984312278, "recall": 0.4725413296215499, "support": 14580}, "micro avg": {"f1-score": 0.7704299514905262, "precision": 0.9740014676590838, "recall": 0.6372427983539095, "support": 14580}, "weighted avg": {"f1-score": 0.7437059317305263, "precision": 0.9164905761660851, "recall": 0.6372427983539095, "support": 14580}, "\u2205": {"f1-score": 0.74140625, "precision": 0.9923318229348205, "recall": 0.5917688630222407, "support": 4811}, "\u23ce": {"f1-score": 0.734732824427481, "precision": 0.9796437659033079, "recall": 0.5877862595419847, "support": 655}, "\u23ce\u23ce": {"f1-score": 0.684375, "precision": 0.9733333333333334, "recall": 0.5277108433734939, "support": 415}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 413}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 416}, "\u2423": {"f1-score": 0.7359430604982206, "precision": 0.936028968014484, "recall": 0.606333072713057, "support": 5116}},
  "ppcr": 0.6542524005486968
}
```
</details>
