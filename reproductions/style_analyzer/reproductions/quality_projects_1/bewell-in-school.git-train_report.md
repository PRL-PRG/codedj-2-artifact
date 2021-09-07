# Train report for javascript / file:///tmp/top-repos-quality-repos-piup7489/bewell-in-school.git HEAD b798cbaa2a23ba420554e1bed34f488df409b754

### Classification report

PPCR: 0.459

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.958| 1.000| 0.605| 0.978| 0.742| 3732| 6168| 0.605 |
| `␣` | 0.998| 0.905| 0.217| 0.949| 0.357| 621| 2586| 0.240 |
| `"` | 1.000| 1.000| 0.719| 1.000| 0.837| 361| 502| 0.719 |
| `'` | 1.000| 1.000| 0.405| 1.000| 0.576| 189| 467| 0.405 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 45| 260| 0.173 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 39| 459| 0.085 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 23| 306| 0.075 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 157| 0.000 |
| `macro avg` | 0.494| 0.488| 0.243| 0.491| 0.314| 5010| 10905| 0.459 |
| `micro avg` | 0.967| 0.967| 0.444| 0.967| 0.609| 5010| 10905| 0.459 |
| `weighted avg` | 0.947| 0.967| 0.444| 0.956| 0.567| 5010| 10905| 0.459 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2436 |3732 |0 |0 |0 |0 |0 |0 |0 |
|1965 |59 |562 |0 |0 |0 |0 |0 |0 |
|278 |0 |0 |189 |0 |0 |0 |0 |0 |
|420 |39 |0 |0 |0 |0 |0 |0 |0 |
|141 |0 |0 |0 |0 |361 |0 |0 |0 |
|283 |23 |0 |0 |0 |0 |0 |0 |0 |
|215 |44 |1 |0 |0 |0 |0 |0 |0 |
|157 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| ClientApp/src/components/AddResource/AddResource.js | 15 |
| ClientApp/src/components/Pages/Survey/Survey.js | 14 |
| ClientApp/src/registerServiceWorker.js | 12 |
| ClientApp/src/helpers/data/surveyQuestionRequest.js | 12 |
| ClientApp/src/helpers/data/answerRequest.js | 12 |
| ClientApp/src/components/Pages/Resources/Resources.js | 12 |
| ClientApp/src/components/AddQuestion/AddQuestion.js | 11 |
| ClientApp/src/App/App.js | 11 |
| ClientApp/src/helpers/data/studentRequest.js | 10 |
| ClientApp/src/helpers/data/surveyRequest.js | 8 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 361}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 189}, "macro avg": {"f1-score": 0.4909620407371587, "precision": 0.49448544241573994, "recall": 0.48812399355877617, "support": 5010}, "micro avg": {"f1-score": 0.9668662674650698, "precision": 0.9668662674650699, "recall": 0.9668662674650699, "support": 5010}, "weighted avg": {"f1-score": 0.9562504421707859, "precision": 0.9468828590117522, "recall": 0.9668662674650699, "support": 5010}, "\u2205": {"f1-score": 0.9783720015729453, "precision": 0.9576597382602001, "recall": 1.0, "support": 3732}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 39}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 23}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 45}, "\u2423": {"f1-score": 0.9493243243243243, "precision": 0.9982238010657194, "recall": 0.9049919484702094, "support": 621}},
  "cl_report_full": {"\"": {"f1-score": 0.8366164542294322, "precision": 1.0, "recall": 0.7191235059760956, "support": 502}, "\u0027": {"f1-score": 0.576219512195122, "precision": 1.0, "recall": 0.40471092077087795, "support": 467}, "macro avg": {"f1-score": 0.31391930110875377, "precision": 0.49448544241573994, "recall": 0.2432771056370753, "support": 10905}, "micro avg": {"f1-score": 0.6087338988375747, "precision": 0.9668662674650699, "recall": 0.44419990829894546, "support": 10905}, "weighted avg": {"f1-score": 0.567279528976443, "precision": 0.8672399830485891, "recall": 0.44419990829894546, "support": 10905}, "\u2205": {"f1-score": 0.7415797317436661, "precision": 0.9576597382602001, "recall": 0.6050583657587548, "support": 6168}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 459}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 157}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 306}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 260}, "\u2423": {"f1-score": 0.35693871070181005, "precision": 0.9982238010657194, "recall": 0.21732405259087392, "support": 2586}},
  "ppcr": 0.4594222833562586
}
```
</details>
