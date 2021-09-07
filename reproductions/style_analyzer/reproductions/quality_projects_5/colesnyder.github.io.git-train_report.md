# Train report for javascript / file:///tmp/top-repos-quality-repos-hyyqowki/colesnyder.github.io.git HEAD 64e939b7b93e9b5710650a620c219fc0745de868

### Classification report

PPCR: 0.804

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.993| 0.992| 0.969| 0.992| 0.981| 48896| 50064| 0.977 |
| `␣` | 0.957| 0.994| 0.866| 0.975| 0.909| 24781| 28470| 0.870 |
| `'` | 0.995| 1.000| 0.654| 0.998| 0.789| 3808| 5824| 0.654 |
| `⏎` | 0.952| 0.754| 0.225| 0.841| 0.364| 1766| 5922| 0.298 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 354| 3483| 0.102 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 76| 1480| 0.051 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 72| 3115| 0.023 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 33| 292| 0.113 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 18| 217| 0.083 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 18| 266| 0.068 |
| `⏎⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 206| 0.010 |
| `micro avg` | 0.981| 0.981| 0.788| 0.981| 0.874| 79824| 99339| 0.804 |
| `macro avg` | 0.354| 0.340| 0.247| 0.346| 0.277| 79824| 99339| 0.804 |
| `weighted avg` | 0.974| 0.981| 0.788| 0.977| 0.823| 79824| 99339| 0.804 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1168 |48500 |396 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3689 |137 |24644 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2016 |0 |0 |3808 |0 |0 |0 |0 |0 |0 |0 |0 |
|4156 |49 |386 |0 |1331 |0 |0 |0 |0 |0 |0 |0 |
|3129 |40 |294 |0 |20 |0 |0 |0 |0 |0 |0 |0 |
|3043 |62 |6 |0 |4 |0 |0 |0 |0 |0 |0 |0 |
|1404 |29 |5 |0 |42 |0 |0 |0 |0 |0 |0 |0 |
|199 |0 |0 |18 |0 |0 |0 |0 |0 |0 |0 |0 |
|259 |5 |28 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|248 |16 |2 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|204 |0 |1 |0 |1 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| JAVASCRIPT/less.js-master/lib/less/parser/parser.js | 235 |
| JAVASCRIPT/less.js-master/lib/source-map/source-map-0.1.31.js | 114 |
| JAVASCRIPT/less.js-master/lib/less/tree/ruleset.js | 83 |
| JAVASCRIPT/less.js-master/test/browser/common.js | 59 |
| JAVASCRIPT/less.js-master/lib/less/visitors/extend-visitor.js | 56 |
| JAVASCRIPT/less.js-master/test/less-test.js | 44 |
| JAVASCRIPT/less.js-master/bin/lessc | 40 |
| JAVASCRIPT/less.js-master/lib/less/parser/parser-input.js | 37 |
| JAVASCRIPT/less.js-master/Gruntfile.js | 33 |
| JAVASCRIPT/less.js-master/test/browser/jasmine-jsreporter.js | 32 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 18}, "\u0027": {"f1-score": 0.9976421273251244, "precision": 0.9952953476215368, "recall": 1.0, "support": 3808}, "macro avg": {"f1-score": 0.3460583345457524, "precision": 0.35427742250133387, "recall": 0.34000485301578914, "support": 79824}, "micro avg": {"f1-score": 0.9806950290639407, "precision": 0.9806950290639407, "recall": 0.9806950290639407, "support": 79824}, "weighted avg": {"f1-score": 0.9768902746361711, "precision": 0.9738251572865068, "recall": 0.9806950290639407, "support": 79824}, "\u2205": {"f1-score": 0.9924898193054618, "precision": 0.9930791596707482, "recall": 0.9919011780104712, "support": 48896}, "\u23ce": {"f1-score": 0.8413400758533501, "precision": 0.952074391988555, "recall": 0.7536806342015855, "support": 1766}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 76}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 33}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 354}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 18}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 72}, "\u2423": {"f1-score": 0.97516965751934, "precision": 0.9566027482338327, "recall": 0.9944715709616239, "support": 24781}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 217}, "\u0027": {"f1-score": 0.7892227979274611, "precision": 0.9952953476215368, "recall": 0.6538461538461539, "support": 5824}, "macro avg": {"f1-score": 0.2765899048652738, "precision": 0.35427742250133387, "recall": 0.24663401974877094, "support": 99339}, "micro avg": {"f1-score": 0.8738746281319245, "precision": 0.9806950290639407, "recall": 0.7880389373760557, "support": 99339}, "weighted avg": {"f1-score": 0.8226961274982423, "precision": 0.8897490406172562, "recall": 0.7880389373760557, "support": 99339}, "\u2205": {"f1-score": 0.980768841883885, "precision": 0.9930791596707482, "recall": 0.968759987216363, "support": 50064}, "\u23ce": {"f1-score": 0.36366120218579234, "precision": 0.952074391988555, "recall": 0.2247551502870652, "support": 5922}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1480}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 206}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 292}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3483}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 266}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3115}, "\u2423": {"f1-score": 0.9088361115208734, "precision": 0.9566027482338327, "recall": 0.8656129258868985, "support": 28470}},
  "ppcr": 0.8035514752514118
}
```
</details>
