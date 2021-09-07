# Train report for javascript / file:///tmp/top-repos-quality-repos-hgyxie03/bluetooh-developer-app.git HEAD e27b373ff68a87dc91fbda18feb6a33aa4211d6d

### Classification report

PPCR: 0.780

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.986| 0.987| 0.959| 0.987| 0.972| 61491| 63327| 0.971 |
| `␣` | 0.948| 0.954| 0.767| 0.951| 0.848| 16284| 20256| 0.804 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.827| 0.970| 0.560| 0.892| 0.668| 1228| 2125| 0.578 |
| `"` | 0.996| 1.000| 0.268| 0.998| 0.422| 1082| 4038| 0.268 |
| `⏎` | 0.961| 0.713| 0.127| 0.819| 0.225| 935| 5246| 0.178 |
| `'` | 1.000| 0.983| 0.058| 0.992| 0.110| 242| 4104| 0.059 |
| `⏎␣⁻␣⁻` | 0.911| 0.750| 0.253| 0.823| 0.396| 232| 687| 0.338 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 56| 2185| 0.026 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 50| 1844| 0.027 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 22| 705| 0.031 |
| `⏎⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 14| 166| 0.084 |
| `weighted avg` | 0.974| 0.975| 0.760| 0.974| 0.800| 81636| 104683| 0.780 |
| `macro avg` | 0.603| 0.578| 0.272| 0.587| 0.331| 81636| 104683| 0.780 |
| `micro avg` | 0.975| 0.975| 0.760| 0.975| 0.854| 81636| 104683| 0.780 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| '| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1836 |60708 |702 |0 |0 |0 |0 |79 |0 |0 |2 |0 |
|3972 |705 |15539 |0 |0 |0 |0 |35 |0 |0 |5 |0 |
|4311 |82 |114 |667 |0 |0 |0 |72 |0 |0 |0 |0 |
|2956 |0 |0 |0 |1082 |0 |0 |0 |0 |0 |0 |0 |
|3862 |0 |0 |0 |4 |238 |0 |0 |0 |0 |0 |0 |
|2129 |42 |12 |2 |0 |0 |0 |0 |0 |0 |0 |0 |
|897 |24 |3 |0 |0 |0 |0 |1191 |0 |0 |10 |0 |
|1794 |10 |15 |23 |0 |0 |0 |2 |0 |0 |0 |0 |
|683 |7 |13 |2 |0 |0 |0 |0 |0 |0 |0 |0 |
|455 |8 |2 |0 |0 |0 |0 |48 |0 |0 |174 |0 |
|152 |0 |0 |0 |0 |0 |0 |14 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| tests/www/autotest/jasmine.js | 257 |
| tests/www/autotest/tests/whitelist.tests.js | 185 |
| tests/www/cordova-plugin-whitelist/test/autotest/tests/whitelist.tests.js | 163 |
| tests/www/autotest/tests/media.tests.js | 160 |
| tests/www/autotest/tests/file.tests.js | 111 |
| tests/www/autotest/tests/compass.tests.js | 84 |
| tests/www/inappbrowser/index.js | 80 |
| tests/www/autotest/tests/battery.tests.js | 76 |
| tests/www/autotest/tests/contacts.tests.js | 68 |
| tests/www/createmobilespec/createmobilespec.js | 61 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9981549815498155, "precision": 0.996316758747698, "recall": 1.0, "support": 1082}, "\u0027": {"f1-score": 0.9916666666666667, "precision": 1.0, "recall": 0.9834710743801653, "support": 242}, "macro avg": {"f1-score": 0.5873872972039942, "precision": 0.6025599550582146, "recall": 0.5780205240492283, "support": 81636}, "micro avg": {"f1-score": 0.9750477730413053, "precision": 0.9750477730413053, "recall": 0.9750477730413053, "support": 81636}, "weighted avg": {"f1-score": 0.9740491161102561, "precision": 0.9736928231218642, "recall": 0.9750477730413053, "support": 81636}, "\u2205": {"f1-score": 0.9865043834347602, "precision": 0.9857435131361023, "recall": 0.9872664292335463, "support": 61491}, "\u23ce": {"f1-score": 0.8189073050951504, "precision": 0.9610951008645533, "recall": 0.7133689839572193, "support": 935}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 50}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 56}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.822695035460993, "precision": 0.9109947643979057, "recall": 0.75, "support": 232}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.8924690895466467, "precision": 0.8265093684941013, "recall": 0.9698697068403909, "support": 1228}, "\u2423": {"f1-score": 0.9508628074899034, "precision": 0.9475, "recall": 0.9542495701301892, "support": 16284}},
  "cl_report_full": {"\"": {"f1-score": 0.42232630757220924, "precision": 0.996316758747698, "recall": 0.2679544328875681, "support": 4038}, "\u0027": {"f1-score": 0.10962690004606171, "precision": 1.0, "recall": 0.05799220272904483, "support": 4104}, "macro avg": {"f1-score": 0.3309723620084925, "precision": 0.6025599550582146, "recall": 0.2720555257732359, "support": 104683}, "micro avg": {"f1-score": 0.8544378190093336, "precision": 0.9750477730413053, "recall": 0.7603813417651386, "support": 104683}, "weighted avg": {"f1-score": 0.8000620100962743, "precision": 0.928211431068272, "recall": 0.7603813417651386, "support": 104683}, "\u2205": {"f1-score": 0.9720045151425392, "precision": 0.9857435131361023, "recall": 0.9586432327443271, "support": 63327}, "\u23ce": {"f1-score": 0.22457912457912457, "precision": 0.9610951008645533, "recall": 0.12714449104079298, "support": 5246}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1844}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 166}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 705}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2185}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.39635535307517084, "precision": 0.9109947643979057, "recall": 0.25327510917030566, "support": 687}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.6679753224901851, "precision": 0.8265093684941013, "recall": 0.5604705882352942, "support": 2125}, "\u2423": {"f1-score": 0.8478284591881274, "precision": 0.9475, "recall": 0.7671307266982622, "support": 20256}},
  "ppcr": 0.7798400886485867
}
```
</details>
