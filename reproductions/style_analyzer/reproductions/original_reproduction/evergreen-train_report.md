# Train report for javascript / file:///tmp/top-repos-quality-repos-1cec1aqd/evergreen HEAD ba22d511dad83c072842e47801ef42697d142f7c

### Classification report

PPCR: 0.925

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.957| 0.981| 0.960| 0.969| 0.958| 25246| 25800| 0.979 |
| `␣` | 0.940| 0.931| 0.818| 0.935| 0.875| 10577| 12033| 0.879 |
| `'` | 1.000| 1.000| 1.000| 1.000| 1.000| 5529| 5529| 1.000 |
| `⏎` | 0.842| 0.901| 0.707| 0.870| 0.769| 2843| 3622| 0.785 |
| `⏎␣⁻␣⁻` | 0.784| 0.628| 0.484| 0.697| 0.599| 1209| 1567| 0.772 |
| `⏎␣⁺␣⁺` | 0.878| 0.658| 0.457| 0.752| 0.601| 1059| 1525| 0.694 |
| `"` | 1.000| 1.000| 1.000| 1.000| 1.000| 1036| 1036| 1.000 |
| `⏎⏎` | 0.848| 0.684| 0.577| 0.757| 0.687| 686| 813| 0.844 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 60| 146| 0.411 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 19| 96| 0.198 |
| `weighted avg` | 0.943| 0.946| 0.875| 0.944| 0.901| 48264| 52167| 0.925 |
| `macro avg` | 0.725| 0.678| 0.600| 0.698| 0.649| 48264| 52167| 0.925 |
| `micro avg` | 0.946| 0.946| 0.875| 0.946| 0.909| 48264| 52167| 0.925 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|554 |24759 |297 |35 |0 |0 |113 |42 |0 |0 |0 |
|1456 |363 |9842 |190 |0 |82 |91 |9 |0 |0 |0 |
|779 |148 |103 |2561 |0 |1 |0 |30 |0 |0 |0 |
|0 |0 |0 |0 |5529 |0 |0 |0 |0 |0 |0 |
|466 |280 |79 |0 |0 |697 |0 |3 |0 |0 |0 |
|358 |276 |130 |44 |0 |0 |759 |0 |0 |0 |0 |
|127 |6 |0 |211 |0 |0 |0 |469 |0 |0 |0 |
|0 |0 |0 |0 |0 |0 |0 |0 |1036 |0 |0 |
|86 |32 |14 |0 |0 |14 |0 |0 |0 |0 |0 |
|77 |7 |6 |1 |0 |0 |5 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| packages/evergreen-buttons/src/styles/ButtonAppearances.js | 91 |
| packages/evergreen-typography/stories/index.stories.js | 61 |
| packages/evergreen-alert/src/components/Alert.js | 57 |
| packages/evergreen-buttons/docs/index.js | 52 |
| packages/evergreen-text-input/stories/index.stories.js | 50 |
| packages/evergreen-file-picker/test/index.js | 50 |
| packages/evergreen-shared-styles/src/styles/CheckboxAppearances.js | 48 |
| packages/evergreen-table/docs/index.js | 47 |
| packages/evergreen-avatar/stories/index.stories.js | 47 |
| packages/evergreen-switch/src/styles/SwitchAppearances.js | 45 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1036}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 5529}, "macro avg": {"f1-score": 0.6980908481125655, "precision": 0.7248853108517788, "recall": 0.6781661532043234, "support": 48264}, "micro avg": {"f1-score": 0.945880987899884, "precision": 0.945880987899884, "recall": 0.945880987899884, "support": 48264}, "weighted avg": {"f1-score": 0.9436907019950105, "precision": 0.9431533171537521, "recall": 0.945880987899884, "support": 48264}, "\u2205": {"f1-score": 0.9687188215270849, "precision": 0.9570175099532295, "recall": 0.9807098154163035, "support": 25246}, "\u23ce": {"f1-score": 0.8703483432455394, "precision": 0.8418803418803419, "recall": 0.9008090045726346, "support": 2843}, "\u23ce\u23ce": {"f1-score": 0.7570621468926553, "precision": 0.8481012658227848, "recall": 0.6836734693877551, "support": 686}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7522935779816513, "precision": 0.8778337531486146, "recall": 0.6581680830972616, "support": 1059}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 60}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.6972898484152504, "precision": 0.7840909090909091, "recall": 0.6277915632754343, "support": 1209}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 19}, "\u2423": {"f1-score": 0.9351957430634739, "precision": 0.9399293286219081, "recall": 0.9305095962938451, "support": 10577}},
  "cl_report_full": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1036}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 5529}, "macro avg": {"f1-score": 0.6488243082606564, "precision": 0.7248853108517788, "recall": 0.6002926452704367, "support": 52167}, "micro avg": {"f1-score": 0.9091216855353427, "precision": 0.945880987899884, "recall": 0.8751126190886959, "support": 52167}, "weighted avg": {"f1-score": 0.9011890794099299, "precision": 0.9368448487281682, "recall": 0.8751126190886959, "support": 52167}, "\u2205": {"f1-score": 0.9583325269493527, "precision": 0.9570175099532295, "recall": 0.9596511627906976, "support": 25800}, "\u23ce": {"f1-score": 0.7686074429771909, "precision": 0.8418803418803419, "recall": 0.707067918277195, "support": 3622}, "\u23ce\u23ce": {"f1-score": 0.6866764275256222, "precision": 0.8481012658227848, "recall": 0.5768757687576875, "support": 813}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.6011211729193618, "precision": 0.8778337531486146, "recall": 0.45704918032786884, "support": 1525}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 146}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.5988165680473373, "precision": 0.7840909090909091, "recall": 0.4843650287172942, "support": 1567}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 96}, "\u2423": {"f1-score": 0.8746889441876999, "precision": 0.9399293286219081, "recall": 0.8179173938336242, "support": 12033}},
  "ppcr": 0.9251825866927368
}
```
</details>
