# Train report for javascript / file:///tmp/top-repos-quality-repos-x6932c50/trolifystorefrontdemo.git HEAD 33759aaf93dc5ea02a24832affa82f81d4189749

### Classification report

PPCR: 0.555

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.977| 1.000| 0.920| 0.989| 0.948| 5733| 6233| 0.920 |
| `␣` | 1.000| 0.915| 0.214| 0.956| 0.353| 834| 3565| 0.234 |
| `'` | 1.000| 1.000| 0.503| 1.000| 0.669| 796| 1582| 0.503 |
| `⏎` | 0.908| 0.908| 0.376| 0.908| 0.532| 325| 785| 0.414 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 28| 679| 0.041 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 21| 480| 0.044 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 12| 647| 0.019 |
| `weighted avg` | 0.972| 0.979| 0.543| 0.975| 0.618| 7749| 13971| 0.555 |
| `micro avg` | 0.979| 0.979| 0.543| 0.979| 0.699| 7749| 13971| 0.555 |
| `macro avg` | 0.555| 0.546| 0.288| 0.550| 0.357| 7749| 13971| 0.555 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|500 |5733 |0 |0 |0 |0 |0 |0 |
|2731 |62 |763 |0 |9 |0 |0 |0 |
|786 |0 |0 |796 |0 |0 |0 |0 |
|460 |30 |0 |0 |295 |0 |0 |0 |
|651 |28 |0 |0 |0 |0 |0 |0 |
|635 |12 |0 |0 |0 |0 |0 |0 |
|459 |0 |0 |0 |21 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| store/customer.js | 39 |
| tests/step-definitions/steps.js | 23 |
| plugins/moltin.js | 7 |
| nuxt.config.js | 7 |
| directives/password-strength.js | 6 |
| server/api/forgottenPassword.js | 6 |
| store/confirmAccount.js | 6 |
| pages/__tests__/unit/confirm-account/_token/index.spec.js | 6 |
| components/__tests__/unit/MyAccount.spec.js | 5 |
| server/api/confirmAccount.js | 5 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 796}, "macro avg": {"f1-score": 0.5502646679478821, "precision": 0.5550265591186307, "recall": 0.5460800590296994, "support": 7749}, "micro avg": {"f1-score": 0.9790940766550522, "precision": 0.9790940766550522, "recall": 0.9790940766550522, "support": 7749}, "weighted avg": {"f1-score": 0.9750513475589422, "precision": 0.9716054773505961, "recall": 0.9790940766550522, "support": 7749}, "\u2205": {"f1-score": 0.9886187273667874, "precision": 0.9774936061381074, "recall": 1.0, "support": 5733}, "\u23ce": {"f1-score": 0.9076923076923076, "precision": 0.9076923076923077, "recall": 0.9076923076923077, "support": 325}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 21}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 28}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u2423": {"f1-score": 0.95554164057608, "precision": 1.0, "recall": 0.9148681055155875, "support": 834}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6694701429772918, "precision": 1.0, "recall": 0.5031605562579013, "support": 1582}, "macro avg": {"f1-score": 0.3573356336003614, "precision": 0.5550265591186307, "recall": 0.287537683793909, "support": 13971}, "micro avg": {"f1-score": 0.6986187845303867, "precision": 0.9790940766550522, "recall": 0.5430534678977883, "support": 13971}, "weighted avg": {"f1-score": 0.618475224354726, "precision": 0.8555046960559219, "recall": 0.5430534678977883, "support": 13971}, "\u2205": {"f1-score": 0.9477599603240205, "precision": 0.9774936061381074, "recall": 0.9197818065137173, "support": 6233}, "\u23ce": {"f1-score": 0.5315315315315315, "precision": 0.9076923076923077, "recall": 0.37579617834394907, "support": 785}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 480}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 679}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 647}, "\u2423": {"f1-score": 0.3525878003696858, "precision": 1.0, "recall": 0.21402524544179524, "support": 3565}},
  "ppcr": 0.5546489156109083
}
```
</details>
