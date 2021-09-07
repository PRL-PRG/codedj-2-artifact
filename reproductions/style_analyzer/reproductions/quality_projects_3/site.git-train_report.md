# Train report for javascript / file:///tmp/top-repos-quality-repos-asde7398/site.git HEAD bdacc9635bb7d14e74c6a4b754a55465301ef32c

### Classification report

PPCR: 0.685

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.979| 0.991| 0.908| 0.985| 0.942| 11024| 12034| 0.916 |
| `␣` | 0.959| 0.976| 0.649| 0.967| 0.774| 3540| 5325| 0.665 |
| `"` | 1.000| 1.000| 0.640| 1.000| 0.781| 354| 553| 0.640 |
| `⏎` | 0.956| 0.690| 0.137| 0.802| 0.240| 284| 1431| 0.198 |
| `'` | 1.000| 1.000| 0.116| 1.000| 0.208| 232| 1997| 0.116 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 32| 306| 0.105 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 29| 263| 0.110 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 29| 286| 0.101 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 23| 277| 0.083 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 223| 0.027 |
| `macro avg` | 0.489| 0.466| 0.245| 0.475| 0.294| 15553| 22695| 0.685 |
| `weighted avg` | 0.967| 0.975| 0.668| 0.971| 0.734| 15553| 22695| 0.685 |
| `micro avg` | 0.975| 0.975| 0.668| 0.975| 0.793| 15553| 22695| 0.685 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1010 |10926 |95 |0 |3 |0 |0 |0 |0 |0 |0 |
|1785 |81 |3455 |0 |4 |0 |0 |0 |0 |0 |0 |
|1765 |0 |0 |232 |0 |0 |0 |0 |0 |0 |0 |
|1147 |56 |32 |0 |196 |0 |0 |0 |0 |0 |0 |
|199 |0 |0 |0 |0 |354 |0 |0 |0 |0 |0 |
|254 |14 |9 |0 |0 |0 |0 |0 |0 |0 |0 |
|234 |28 |1 |0 |0 |0 |0 |0 |0 |0 |0 |
|217 |1 |5 |0 |0 |0 |0 |0 |0 |0 |0 |
|274 |26 |6 |0 |0 |0 |0 |0 |0 |0 |0 |
|257 |26 |1 |0 |2 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| server/app_mysql.js | 34 |
| features/step_definitions/steps.js | 34 |
| src/components/StartupProfile/index.js | 32 |
| src/components/Profile/addProfile.js | 29 |
| src/services/user.service.js | 28 |
| src/registerServiceWorker.js | 25 |
| src/components/Profile/index.js | 18 |
| src/components/SignIn/index.js | 18 |
| src/components/SignUp/index.js | 18 |
| src/components/StartupProfile/HorizontalNonLinearStepper.js | 16 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 354}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 232}, "macro avg": {"f1-score": 0.47540043053432174, "precision": 0.4893962352023536, "recall": 0.46572398504249446, "support": 15553}, "micro avg": {"f1-score": 0.9749244518742365, "precision": 0.9749244518742365, "recall": 0.9749244518742365, "support": 15553}, "weighted avg": {"f1-score": 0.9707265175913, "precision": 0.9673994611108714, "recall": 0.9749244518742365, "support": 15553}, "\u2205": {"f1-score": 0.9851230727616986, "precision": 0.9792077433231762, "recall": 0.9911103047895501, "support": 11024}, "\u23ce": {"f1-score": 0.8016359918200409, "precision": 0.9560975609756097, "recall": 0.6901408450704225, "support": 284}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 23}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 32}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 29}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 29}, "\u2423": {"f1-score": 0.9672452407614782, "precision": 0.9586570477247502, "recall": 0.9759887005649718, "support": 3540}},
  "cl_report_full": {"\"": {"f1-score": 0.7805953693495038, "precision": 1.0, "recall": 0.6401446654611211, "support": 553}, "\u0027": {"f1-score": 0.20816509645580974, "precision": 1.0, "recall": 0.11617426139208813, "support": 1997}, "macro avg": {"f1-score": 0.29444735770587005, "precision": 0.4893962352023536, "recall": 0.245003991240862, "support": 22695}, "micro avg": {"f1-score": 0.7928780589834763, "precision": 0.9749244518742365, "recall": 0.6681207314386429, "support": 22695}, "weighted avg": {"f1-score": 0.733636233778345, "precision": 0.9168015145204448, "recall": 0.6681207314386429, "support": 22695}, "\u2205": {"f1-score": 0.9422214556743704, "precision": 0.9792077433231762, "recall": 0.9079275386405186, "support": 12034}, "\u23ce": {"f1-score": 0.23960880195599024, "precision": 0.9560975609756097, "recall": 0.13696715583508037, "support": 1431}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 223}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 277}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 306}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 263}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 286}, "\u2423": {"f1-score": 0.773882853623026, "precision": 0.9586570477247502, "recall": 0.6488262910798122, "support": 5325}},
  "ppcr": 0.6853051332892708
}
```
</details>
