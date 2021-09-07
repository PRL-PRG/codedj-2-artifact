# Train report for javascript / file:///tmp/top-repos-quality-repos-p8y2nze6/dsl-toolkit.git HEAD 6e7f8cb1a0804f3bcff23d9c04d9f1d49f77dd8b

### Classification report

PPCR: 0.866

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.959| 0.985| 0.933| 0.972| 0.946| 20147| 21266| 0.947 |
| `␣` | 0.952| 0.960| 0.853| 0.956| 0.900| 8571| 9642| 0.889 |
| `'` | 1.000| 1.000| 0.995| 1.000| 0.997| 5141| 5169| 0.995 |
| `⏎␣⁻␣⁻` | 0.952| 0.751| 0.692| 0.840| 0.801| 893| 970| 0.921 |
| `⏎` | 0.941| 0.475| 0.108| 0.632| 0.194| 505| 2220| 0.227 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 125| 1039| 0.120 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 55| 591| 0.093 |
| `macro avg` | 0.686| 0.596| 0.512| 0.628| 0.548| 35437| 40897| 0.866 |
| `weighted avg` | 0.958| 0.963| 0.834| 0.959| 0.860| 35437| 40897| 0.866 |
| `micro avg` | 0.963| 0.963| 0.834| 0.963| 0.894| 35437| 40897| 0.866 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1119 |19843 |288 |0 |0 |0 |16 |0 |
|1071 |329 |8225 |0 |0 |0 |17 |0 |
|28 |0 |0 |5141 |0 |0 |0 |0 |
|1715 |192 |73 |0 |240 |0 |0 |0 |
|914 |80 |44 |0 |0 |0 |1 |0 |
|77 |217 |5 |0 |0 |0 |671 |0 |
|536 |33 |7 |0 |15 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| packages/cowlog/tests/tests/functional/functional-test.js | 85 |
| packages/cowlog/src/lib/logger/logger.js | 42 |
| packages/require-a-lot/tests/assets/dependency-injection/requires.js | 42 |
| packages/dsl-framework/src/core/unlimited-curry-factory/container/core/index.js | 42 |
| packages/require-a-lot/tests/assets/map-dir/requires.js | 32 |
| packages/require-a-lot/tests/tests/suites/flow/chain-calls/tag/tag.js | 32 |
| packages/require-a-lot/tests/lib/requires.js | 30 |
| packages/require-a-lot/src/logging-and-linking/linker/index.js | 27 |
| packages/directory-fixture-provider/src/index.js | 27 |
| packages/require-a-lot/tests/tests/suites/flow/chain-calls/log.js | 23 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 5141}, "macro avg": {"f1-score": 0.6283959043400429, "precision": 0.6862248243036412, "recall": 0.595884217219535, "support": 35437}, "micro avg": {"f1-score": 0.9628354544684934, "precision": 0.9628354544684934, "recall": 0.9628354544684934, "support": 35437}, "weighted avg": {"f1-score": 0.9588338979789072, "precision": 0.9578166236395078, "recall": 0.9628354544684934, "support": 35437}, "\u2205": {"f1-score": 0.9717195955045175, "precision": 0.9588769691698077, "recall": 0.9849109048493572, "support": 20147}, "\u23ce": {"f1-score": 0.631578947368421, "precision": 0.9411764705882353, "recall": 0.4752475247524752, "support": 505}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 55}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 125}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8397997496871089, "precision": 0.9517730496453901, "recall": 0.7513997760358343, "support": 893}, "\u2423": {"f1-score": 0.9556730378202521, "precision": 0.9517472807220551, "recall": 0.9596313148990783, "support": 8571}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9972841901066924, "precision": 1.0, "recall": 0.9945830915070614, "support": 5169}, "macro avg": {"f1-score": 0.5482738377512161, "precision": 0.6862248243036412, "recall": 0.5115097488906859, "support": 40897}, "micro avg": {"f1-score": 0.8939659915633925, "precision": 0.9628354544684934, "recall": 0.8342910237914761, "support": 40897}, "weighted avg": {"f1-score": 0.85950101191092, "precision": 0.9230470824253392, "recall": 0.8342910237914761, "support": 40897}, "\u2205": {"f1-score": 0.9458055290753098, "precision": 0.9588769691698077, "recall": 0.9330856766669802, "support": 21266}, "\u23ce": {"f1-score": 0.19393939393939394, "precision": 0.9411764705882353, "recall": 0.10810810810810811, "support": 2220}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 591}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1039}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8011940298507463, "precision": 0.9517730496453901, "recall": 0.6917525773195876, "support": 970}, "\u2423": {"f1-score": 0.8996937212863706, "precision": 0.9517472807220551, "recall": 0.8530387886330637, "support": 9642}},
  "ppcr": 0.8664938748563464
}
```
</details>
