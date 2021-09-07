# Train report for javascript / file:///tmp/top-repos-quality-repos-i3dkitfq/urule.git HEAD 1b7edeef8bb6e9b365b249cb9900fe82cf23ad5d

### Classification report

PPCR: 0.819

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.972| 0.999| 0.948| 0.985| 0.960| 79403| 83629| 0.949 |
| `␣` | 0.983| 0.760| 0.625| 0.857| 0.764| 7547| 9169| 0.823 |
| `⏎` | 0.938| 0.980| 0.682| 0.958| 0.789| 5430| 7808| 0.695 |
| `"` | 0.984| 0.992| 0.461| 0.988| 0.628| 3196| 6876| 0.465 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 1.000| 0.780| 0.209| 0.876| 0.346| 449| 1676| 0.268 |
| `'` | 0.935| 0.873| 0.178| 0.903| 0.299| 394| 1933| 0.204 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 1.000| 0.471| 0.090| 0.641| 0.166| 331| 1728| 0.192 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 262| 780| 0.336 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 104| 2356| 0.044 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 63| 2419| 0.026 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 19| 127| 0.150 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 13| 123| 0.106 |
| `macro avg` | 0.568| 0.488| 0.266| 0.517| 0.329| 97211| 118624| 0.819 |
| `weighted avg` | 0.967| 0.971| 0.796| 0.967| 0.836| 97211| 118624| 0.819 |
| `micro avg` | 0.971| 0.971| 0.796| 0.971| 0.875| 97211| 118624| 0.819 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| '| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⇥⁺| ⏎⇥⁻| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|4226 |79316 |78 |9 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1622 |1801 |5733 |13 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2378 |95 |12 |5323 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3680 |0 |0 |0 |3172 |24 |0 |0 |0 |0 |0 |0 |0 |
|1539 |0 |0 |0 |50 |344 |0 |0 |0 |0 |0 |0 |0 |
|1397 |173 |2 |0 |0 |0 |156 |0 |0 |0 |0 |0 |0 |
|1227 |90 |1 |8 |0 |0 |0 |350 |0 |0 |0 |0 |0 |
|2356 |58 |1 |4 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2252 |51 |0 |53 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|518 |1 |0 |261 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|108 |11 |5 |3 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|110 |10 |0 |3 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| urule-console-js/src/frame/action.js | 234 |
| urule-console-js/src/editor/table/manualRowResize.js | 176 |
| urule-console-js/src/editor/table/manualColumnResize.js | 173 |
| urule-console-js/src/flow/CodeMirror.js | 152 |
| urule-console-js/src/editor/common/if-hint.js | 121 |
| urule-console-js/src/editor/ul/urule-hint.js | 118 |
| urule-console-js/src/editor/scriptdecisiontable/table-if-hint.js | 112 |
| urule-console-js/src/editor/common/ComparisonOperator.js | 90 |
| urule-console-js/src/editor/Math.uuid.js | 86 |
| urule-console-js/src/editor/decisiontable/DecisionTable.js | 82 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9884699283265814, "precision": 0.9844816883923029, "recall": 0.9924906132665833, "support": 3196}, "\u0027": {"f1-score": 0.9028871391076114, "precision": 0.9347826086956522, "recall": 0.8730964467005076, "support": 394}, "macro avg": {"f1-score": 0.5174046240184973, "precision": 0.5676558706400178, "recall": 0.4879362292166351, "support": 97211}, "micro avg": {"f1-score": 0.971021797944677, "precision": 0.971021797944677, "recall": 0.971021797944677, "support": 97211}, "weighted avg": {"f1-score": 0.9672114192272097, "precision": 0.9667611397918711, "recall": 0.971021797944677, "support": 97211}, "\u2205": {"f1-score": 0.9852368501139687, "precision": 0.9719383378673137, "recall": 0.9989043235142249, "support": 79403}, "\u23ce": {"f1-score": 0.9584946430179166, "precision": 0.9376431213669192, "recall": 0.9802946593001841, "support": 5430}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 63}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 104}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 262}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 19}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.6406570841889117, "precision": 1.0, "recall": 0.47129909365558914, "support": 331}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 13}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.8760951188986233, "precision": 1.0, "recall": 0.779510022271715, "support": 449}, "\u2423": {"f1-score": 0.8570147245683535, "precision": 0.9830246913580247, "recall": 0.7596395918908175, "support": 7547}},
  "cl_report_full": {"\"": {"f1-score": 0.6282432164785106, "precision": 0.9844816883923029, "recall": 0.46131471785922046, "support": 6876}, "\u0027": {"f1-score": 0.299000434593655, "precision": 0.9347826086956522, "recall": 0.17796171753750648, "support": 1933}, "macro avg": {"f1-score": 0.32935122457343047, "precision": 0.5676558706400178, "recall": 0.2661506207062507, "support": 118624}, "micro avg": {"f1-score": 0.8746866819561239, "precision": 0.971021797944677, "recall": 0.7957411653628271, "support": 118624}, "weighted avg": {"f1-score": 0.8364460941614498, "precision": 0.9239018496779268, "recall": 0.7957411653628271, "support": 118624}, "\u2205": {"f1-score": 0.9600387327140134, "precision": 0.9719383378673137, "recall": 0.9484269810711595, "support": 83629}, "\u23ce": {"f1-score": 0.789469781238413, "precision": 0.9376431213669192, "recall": 0.6817366803278688, "support": 7808}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2419}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2356}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 780}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 127}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.1656050955414013, "precision": 1.0, "recall": 0.09027777777777778, "support": 1728}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 123}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.3455083909180652, "precision": 1.0, "recall": 0.20883054892601433, "support": 1676}, "\u2423": {"f1-score": 0.7643490433971069, "precision": 0.9830246913580247, "recall": 0.6252590249754608, "support": 9169}},
  "ppcr": 0.8194884677636903
}
```
</details>
