# Train report for javascript / file:///tmp/top-repos-quality-repos-a_ujekwm/money-mapper.git HEAD 358814894935652004760f9731e2b7a77d33aace

### Classification report

PPCR: 0.547

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.972| 1.000| 0.801| 0.986| 0.878| 3904| 4872| 0.801 |
| `␣` | 0.991| 0.894| 0.330| 0.940| 0.495| 837| 2265| 0.370 |
| `⏎` | 0.897| 0.962| 0.325| 0.928| 0.477| 236| 698| 0.338 |
| `'` | 1.000| 1.000| 0.277| 1.000| 0.433| 215| 777| 0.277 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 25| 314| 0.080 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 17| 327| 0.052 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 202| 0.025 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 123| 0.000 |
| `weighted avg` | 0.964| 0.972| 0.532| 0.968| 0.634| 5239| 9578| 0.547 |
| `micro avg` | 0.972| 0.972| 0.532| 0.972| 0.688| 5239| 9578| 0.547 |
| `macro avg` | 0.483| 0.482| 0.217| 0.482| 0.286| 5239| 9578| 0.547 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|968 |3904 |0 |0 |0 |0 |0 |0 |0 |
|1428 |63 |748 |0 |26 |0 |0 |0 |0 |
|562 |0 |0 |215 |0 |0 |0 |0 |0 |
|462 |9 |0 |0 |227 |0 |0 |0 |0 |
|310 |10 |7 |0 |0 |0 |0 |0 |0 |
|289 |25 |0 |0 |0 |0 |0 |0 |0 |
|197 |5 |0 |0 |0 |0 |0 |0 |0 |
|123 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| script/seed.js | 28 |
| server/index.js | 13 |
| client/store/user.js | 10 |
| client/components/auth-form.js | 9 |
| client/routes.js | 9 |
| client/components/user-account.js | 8 |
| client/components/utility.js | 8 |
| client/components/LegendDonut.js | 6 |
| server/api/plaidLink.js | 6 |
| client/app.js | 6 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 215}, "macro avg": {"f1-score": 0.4817478045242635, "precision": 0.48250915402338507, "recall": 0.48194153352368224, "support": 5239}, "micro avg": {"f1-score": 0.9723229623974041, "precision": 0.9723229623974041, "recall": 0.9723229623974041, "support": 5239}, "weighted avg": {"f1-score": 0.9676327432412584, "precision": 0.9641363383562811, "recall": 0.9723229623974041, "support": 5239}, "\u2205": {"f1-score": 0.9858585858585859, "precision": 0.9721115537848606, "recall": 1.0, "support": 3904}, "\u23ce": {"f1-score": 0.9284253578732107, "precision": 0.8972332015810277, "recall": 0.961864406779661, "support": 236}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 17}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 25}, "\u2423": {"f1-score": 0.9396984924623116, "precision": 0.990728476821192, "recall": 0.8936678614097969, "support": 837}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 123}, "\u0027": {"f1-score": 0.43346774193548393, "precision": 1.0, "recall": 0.2767052767052767, "support": 777}, "macro avg": {"f1-score": 0.2855890059810123, "precision": 0.48250915402338507, "recall": 0.21668457886570547, "support": 9578}, "micro avg": {"f1-score": 0.6875885806843492, "precision": 0.9723229623974041, "recall": 0.5318438087283358, "support": 9578}, "weighted avg": {"f1-score": 0.6339545837782797, "precision": 0.8752762857322404, "recall": 0.5318438087283358, "support": 9578}, "\u2205": {"f1-score": 0.8784878487848785, "precision": 0.9721115537848606, "recall": 0.8013136288998358, "support": 4872}, "\u23ce": {"f1-score": 0.47739221871713994, "precision": 0.8972332015810277, "recall": 0.32521489971346706, "support": 698}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 202}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 327}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 314}, "\u2423": {"f1-score": 0.49536423841059596, "precision": 0.990728476821192, "recall": 0.33024282560706403, "support": 2265}},
  "ppcr": 0.5469826686155773
}
```
</details>
