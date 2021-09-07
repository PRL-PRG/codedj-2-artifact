# Train report for javascript / file:///tmp/top-repos-quality-repos-q9ykj62p/storybook HEAD b28217f887af533a17cb1498887d6b4bd41bd643

### Classification report

PPCR: 0.936

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.961| 0.995| 0.987| 0.978| 0.974| 92134| 92916| 0.992 |
| `␣` | 0.966| 0.954| 0.914| 0.960| 0.939| 47465| 49507| 0.959 |
| `'` | 1.000| 1.000| 1.000| 1.000| 1.000| 18696| 18696| 1.000 |
| `⏎` | 0.898| 0.841| 0.552| 0.868| 0.683| 7391| 11273| 0.656 |
| `⏎␣⁻␣⁻` | 0.986| 0.830| 0.786| 0.901| 0.875| 5868| 6197| 0.947 |
| `⏎␣⁺␣⁺` | 0.883| 0.768| 0.690| 0.822| 0.775| 5723| 6373| 0.898 |
| `"` | 0.998| 0.993| 0.993| 0.995| 0.995| 882| 882| 1.000 |
| `⏎⏎` | 1.000| 0.332| 0.041| 0.498| 0.078| 609| 4955| 0.123 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 128| 131| 0.977 |
| `⏎⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 101| 0.020 |
| `weighted avg` | 0.962| 0.963| 0.901| 0.961| 0.916| 178898| 191031| 0.936 |
| `macro avg` | 0.769| 0.671| 0.596| 0.702| 0.632| 178898| 191031| 0.936 |
| `micro avg` | 0.963| 0.963| 0.901| 0.963| 0.931| 178898| 191031| 0.936 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|782 |91694 |425 |0 |3 |2 |10 |0 |0 |0 |0 |
|2042 |1444 |45263 |0 |176 |579 |3 |0 |0 |0 |0 |
|0 |0 |0 |18694 |0 |0 |0 |0 |2 |0 |0 |
|3882 |655 |518 |0 |6218 |0 |0 |0 |0 |0 |0 |
|650 |897 |426 |0 |5 |4395 |0 |0 |0 |0 |0 |
|329 |611 |202 |0 |184 |0 |4871 |0 |0 |0 |0 |
|4346 |46 |23 |0 |338 |0 |0 |202 |0 |0 |0 |
|0 |0 |0 |6 |0 |0 |0 |0 |876 |0 |0 |
|3 |63 |5 |0 |2 |0 |58 |0 |0 |0 |0 |
|99 |0 |0 |0 |2 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| examples/official-storybook/stories/addon-info.stories.js | 148 |
| examples/official-storybook/stories/addon-actions.stories.js | 116 |
| lib/components/src/layout/desktop.js | 100 |
| lib/components/src/tabs/tabs.stories.js | 93 |
| addons/jest/src/components/Result.js | 83 |
| lib/cli/lib/initiate.js | 78 |
| lib/ui/src/modules/ui/components/stories_panel/stories_tree/index.test.js | 77 |
| lib/codemod/src/transforms/__testfixtures__/update-addon-info/update-addon-info.input.js | 77 |
| lib/components/src/tabs/tabs.js | 67 |
| lib/core/src/client/preview/client_api.test.js | 65 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9954545454545456, "precision": 0.9977220956719818, "recall": 0.9931972789115646, "support": 882}, "\u0027": {"f1-score": 0.9997860733768317, "precision": 0.9996791443850267, "recall": 0.999893025246042, "support": 18696}, "macro avg": {"f1-score": 0.7022206028822238, "precision": 0.7690722295313897, "recall": 0.6712956638368073, "support": 178898}, "micro avg": {"f1-score": 0.9626323379803017, "precision": 0.9626323379803017, "recall": 0.9626323379803017, "support": 178898}, "weighted avg": {"f1-score": 0.9610346952221683, "precision": 0.9616766902668902, "recall": 0.9626323379803017, "support": 178898}, "\u2205": {"f1-score": 0.9778398669112315, "precision": 0.9610523005974216, "recall": 0.9952243471465474, "support": 92134}, "\u23ce": {"f1-score": 0.8684964033801243, "precision": 0.8975173210161663, "recall": 0.8412934650250304, "support": 7391}, "\u23ce\u23ce": {"f1-score": 0.4981504315659679, "precision": 1.0, "recall": 0.33169129720853857, "support": 609}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8215721095429479, "precision": 0.8832395498392283, "recall": 0.7679538703477198, "support": 5723}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9012025901942645, "precision": 0.9856333468231485, "recall": 0.830095432856169, "support": 5868}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 128}, "\u2423": {"f1-score": 0.9597040083963235, "precision": 0.9658785369809227, "recall": 0.9536079216264616, "support": 47465}},
  "cl_report_full": {"\"": {"f1-score": 0.9954545454545456, "precision": 0.9977220956719818, "recall": 0.9931972789115646, "support": 882}, "\u0027": {"f1-score": 0.9997860733768317, "precision": 0.9996791443850267, "recall": 0.999893025246042, "support": 18696}, "macro avg": {"f1-score": 0.6319090250565912, "precision": 0.7690722295313897, "recall": 0.5962217335700081, "support": 191031}, "micro avg": {"f1-score": 0.9310597438968018, "precision": 0.9626323379803017, "recall": 0.9014924279305453, "support": 191031}, "weighted avg": {"f1-score": 0.9160888096612804, "precision": 0.9605479986265759, "recall": 0.9014924279305453, "support": 191031}, "\u2205": {"f1-score": 0.9737795099986194, "precision": 0.9610523005974216, "recall": 0.9868483361315596, "support": 92916}, "\u23ce": {"f1-score": 0.6832591615845283, "precision": 0.8975173210161663, "recall": 0.5515834294331589, "support": 11273}, "\u23ce\u23ce": {"f1-score": 0.07834012022493697, "precision": 1.0, "recall": 0.04076690211907164, "support": 4955}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 101}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.77451757864129, "precision": 0.8832395498392283, "recall": 0.6896281186254511, "support": 6373}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8745847921716491, "precision": 0.9856333468231485, "recall": 0.7860254962078425, "support": 6197}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 131}, "\u2423": {"f1-score": 0.9393684691135117, "precision": 0.9658785369809227, "recall": 0.9142747490253903, "support": 49507}},
  "ppcr": 0.9364867482241103
}
```
</details>
