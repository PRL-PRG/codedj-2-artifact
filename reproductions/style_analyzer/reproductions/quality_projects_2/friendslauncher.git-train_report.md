# Train report for javascript / file:///tmp/top-repos-quality-repos-rzoa8z__/friendslauncher.git HEAD ffdda5d176760eb7d54e67ce35454bfd88aa6f5d

### Classification report

PPCR: 0.738

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.992| 0.994| 0.878| 0.993| 0.932| 18018| 20398| 0.883 |
| `␣` | 0.964| 0.986| 0.792| 0.975| 0.870| 6543| 8146| 0.803 |
| `'` | 1.000| 1.000| 0.947| 1.000| 0.973| 3010| 3179| 0.947 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 63| 1470| 0.043 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 54| 2117| 0.026 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 34| 1403| 0.024 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 29| 780| 0.037 |
| `⏎⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 96| 0.000 |
| `weighted avg` | 0.980| 0.986| 0.728| 0.983| 0.776| 27751| 37589| 0.738 |
| `micro avg` | 0.986| 0.986| 0.728| 0.986| 0.838| 27751| 37589| 0.738 |
| `macro avg` | 0.370| 0.373| 0.327| 0.371| 0.347| 27751| 37589| 0.738 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| ⏎⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2380 |17912 |106 |0 |0 |0 |0 |0 |0 |
|1603 |92 |6451 |0 |0 |0 |0 |0 |0 |
|169 |0 |0 |3010 |0 |0 |0 |0 |0 |
|2063 |8 |46 |0 |0 |0 |0 |0 |0 |
|1407 |6 |57 |0 |0 |0 |0 |0 |0 |
|1369 |34 |0 |0 |0 |0 |0 |0 |0 |
|751 |0 |29 |0 |0 |0 |0 |0 |0 |
|96 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| app/assets/js/scripts/landing.js | 65 |
| app/assets/js/assetguard.js | 55 |
| app/assets/js/scripts/settings.js | 47 |
| app/assets/js/processbuilder.js | 42 |
| app/assets/js/configmanager.js | 32 |
| app/assets/js/distromanager.js | 28 |
| app/assets/js/scripts/uibinder.js | 17 |
| app/assets/js/mojang.js | 16 |
| index.js | 16 |
| app/assets/js/scripts/overlay.js | 15 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 3010}, "macro avg": {"f1-score": 0.37103004842625975, "precision": 0.36958297779107396, "recall": 0.37250702071885017, "support": 27751}, "micro avg": {"f1-score": 0.9863788692299377, "precision": 0.9863788692299377, "recall": 0.9863788692299377, "support": 27751}, "weighted avg": {"f1-score": 0.9832055251227874, "precision": 0.9800893113806007, "recall": 0.9863788692299377, "support": 27751}, "\u2205": {"f1-score": 0.9931799279179375, "precision": 0.992244626634168, "recall": 0.9941169941169942, "support": 18018}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 54}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 29}, "\u23ce\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 63}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 34}, "\u2423": {"f1-score": 0.9750604594921403, "precision": 0.9644191956944237, "recall": 0.9859391716338072, "support": 6543}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9726934884472451, "precision": 1.0, "recall": 0.9468386284995282, "support": 3179}, "macro avg": {"f1-score": 0.3467621291505799, "precision": 0.36958297779107396, "recall": 0.3271107938514707, "support": 37589}, "micro avg": {"f1-score": 0.8378634833180288, "precision": 0.9863788692299377, "recall": 0.7282183617547687, "support": 37589}, "weighted avg": {"f1-score": 0.7763350260127365, "precision": 0.8320243864484433, "recall": 0.7282183617547687, "support": 37589}, "\u2205": {"f1-score": 0.931703511053316, "precision": 0.992244626634168, "recall": 0.8781253064025885, "support": 20398}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2117}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 780}, "\u23ce\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 96}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1470}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1403}, "\u2423": {"f1-score": 0.8697000337040783, "precision": 0.9644191956944237, "recall": 0.791922415909649, "support": 8146}},
  "ppcr": 0.7382744951980633
}
```
</details>
