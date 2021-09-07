# Train report for javascript / file:///tmp/top-repos-quality-repos-dqfg01ai/react-conf-registration-web.git HEAD 6f3ea2fd8ccd690cfa1d29e5b317b669b97a1e13

### Classification report

PPCR: 0.738

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.986| 0.994| 0.832| 0.990| 0.902| 17130| 20472| 0.837 |
| `␣` | 0.968| 0.960| 0.705| 0.964| 0.816| 4942| 6728| 0.735 |
| `"` | 1.000| 1.000| 0.639| 1.000| 0.780| 1622| 2538| 0.639 |
| `⏎` | 0.958| 0.984| 0.515| 0.971| 0.670| 865| 1654| 0.523 |
| `⏎␣⁻␣⁻` | 1.000| 0.954| 0.378| 0.976| 0.549| 456| 1151| 0.396 |
| `⏎␣⁺␣⁺` | 0.948| 0.631| 0.171| 0.758| 0.289| 320| 1184| 0.270 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 601| 0.008 |
| `macro avg` | 0.837| 0.789| 0.463| 0.808| 0.572| 25340| 34328| 0.738 |
| `micro avg` | 0.982| 0.982| 0.725| 0.982| 0.834| 25340| 34328| 0.738 |
| `weighted avg` | 0.982| 0.982| 0.725| 0.982| 0.816| 25340| 34328| 0.738 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|3342 |17032 |86 |0 |1 |11 |0 |0 |
|1786 |165 |4746 |0 |31 |0 |0 |0 |
|916 |0 |0 |1622 |0 |0 |0 |0 |
|789 |5 |9 |0 |851 |0 |0 |0 |
|864 |57 |61 |0 |0 |202 |0 |0 |
|695 |20 |1 |0 |0 |0 |435 |0 |
|596 |0 |0 |0 |5 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/Components/RegisterComponents/RegisterReviewPage/RegisterReviewPage.js | 37 |
| src/serviceWorker.js | 32 |
| src/App.js | 27 |
| src/Components/RegisterComponents/RegisterPage/Subcomponents/RegisteringContent.js | 20 |
| src/Components/RegisterComponents/RegisterPage/Subcomponents/RegisterButtons.js | 18 |
| src/actions/register.js | 18 |
| src/Components/LandingPageComponents/LandingJumbotron/LandingJumbotron.js | 14 |
| src/Components/RegisterComponents/RegisterReviewPage/Subcomponents/CreditCardInput.js | 11 |
| src/Components/RegisterComponents/RegisterPage/RegisterPage.js | 11 |
| src/Components/LandingPageComponents/LandingPage/__tests__/LandingPage.test.js | 11 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1622}, "macro avg": {"f1-score": 0.8084899308058484, "precision": 0.8371963029449361, "recall": 0.7890901976115511, "support": 25340}, "micro avg": {"f1-score": 0.982162588792423, "precision": 0.982162588792423, "recall": 0.982162588792423, "support": 25340}, "weighted avg": {"f1-score": 0.9815582664630478, "precision": 0.9818197952245004, "recall": 0.982162588792423, "support": 25340}, "\u2205": {"f1-score": 0.9899735534307884, "precision": 0.9857051912726431, "recall": 0.9942790426152948, "support": 17130}, "\u23ce": {"f1-score": 0.970907016543069, "precision": 0.9583333333333334, "recall": 0.9838150289017341, "support": 865}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7579737335834896, "precision": 0.9483568075117371, "recall": 0.63125, "support": 320}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9764309764309764, "precision": 1.0, "recall": 0.9539473684210527, "support": 456}, "\u2423": {"f1-score": 0.9641442356526155, "precision": 0.9679787884968387, "recall": 0.9603399433427762, "support": 4942}},
  "cl_report_full": {"\"": {"f1-score": 0.7798076923076923, "precision": 1.0, "recall": 0.6390858944050434, "support": 2538}, "macro avg": {"f1-score": 0.5722183989971764, "precision": 0.8371963029449361, "recall": 0.4627874787080519, "support": 34328}, "micro avg": {"f1-score": 0.8342159951732923, "precision": 0.982162588792423, "recall": 0.7250058261477511, "support": 34328}, "weighted avg": {"f1-score": 0.816349209676602, "precision": 0.9639028128107525, "recall": 0.7250058261477511, "support": 34328}, "\u2205": {"f1-score": 0.9023337130142246, "precision": 0.9857051912726431, "recall": 0.8319656115670183, "support": 20472}, "\u23ce": {"f1-score": 0.6695515342250198, "precision": 0.9583333333333334, "recall": 0.5145102781136639, "support": 1654}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 601}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.2891911238367931, "precision": 0.9483568075117371, "recall": 0.17060810810810811, "support": 1184}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.5485498108448928, "precision": 1.0, "recall": 0.3779322328410078, "support": 1151}, "\u2423": {"f1-score": 0.8160949187516121, "precision": 0.9679787884968387, "recall": 0.705410225921522, "support": 6728}},
  "ppcr": 0.7381729200652528
}
```
</details>
