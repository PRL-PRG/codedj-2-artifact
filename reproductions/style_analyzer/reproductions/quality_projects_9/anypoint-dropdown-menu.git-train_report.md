# Train report for javascript / file:///tmp/top-repos-quality-repos-f0r11jdy/anypoint-dropdown-menu.git HEAD e02e5909d6b3a21d0b93174448da9b7a4cbc63ce

### Classification report

PPCR: 0.600

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.957| 1.000| 0.711| 0.978| 0.816| 3440| 4839| 0.711 |
| `'` | 1.000| 1.000| 0.721| 1.000| 0.838| 531| 736| 0.721 |
| `␣` | 1.000| 0.786| 0.195| 0.880| 0.326| 513| 2069| 0.248 |
| `⏎` | 0.957| 0.970| 0.590| 0.964| 0.730| 368| 605| 0.608 |
| `⏎␣⁺␣⁺` | 0.955| 0.925| 0.892| 0.940| 0.922| 321| 333| 0.964 |
| `⏎␣⁻␣⁻` | 1.000| 0.936| 0.758| 0.967| 0.862| 267| 330| 0.809 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 22| 197| 0.112 |
| `weighted avg` | 0.963| 0.966| 0.579| 0.963| 0.689| 5462| 9109| 0.600 |
| `macro avg` | 0.838| 0.802| 0.552| 0.818| 0.642| 5462| 9109| 0.600 |
| `micro avg` | 0.966| 0.966| 0.579| 0.966| 0.724| 5462| 9109| 0.600 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1399 |3440 |0 |0 |0 |0 |0 |0 |
|1556 |90 |403 |0 |6 |14 |0 |0 |
|205 |0 |0 |531 |0 |0 |0 |0 |
|237 |11 |0 |0 |357 |0 |0 |0 |
|12 |24 |0 |0 |0 |297 |0 |0 |
|63 |17 |0 |0 |0 |0 |250 |0 |
|175 |12 |0 |0 |10 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/AnypointDropdownMenu.js | 65 |
| test/anypoint-dropdown-menu.test.js | 65 |
| demo/index.js | 47 |
| anypoint-dropdown-menu.js | 2 |
| src/Styles.js | 2 |
| src/icons.js | 2 |
| index.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 531}, "macro avg": {"f1-score": 0.8183675947913169, "precision": 0.8384627553386746, "recall": 0.8024638538942731, "support": 5462}, "micro avg": {"f1-score": 0.9663127059685097, "precision": 0.9663127059685097, "recall": 0.9663127059685097, "support": 5462}, "weighted avg": {"f1-score": 0.96330867290101, "precision": 0.9634498521013243, "recall": 0.9663127059685097, "support": 5462}, "\u2205": {"f1-score": 0.9781063406312198, "precision": 0.9571508069003896, "recall": 1.0, "support": 3440}, "\u23ce": {"f1-score": 0.9635627530364372, "precision": 0.9571045576407506, "recall": 0.970108695652174, "support": 368}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.939873417721519, "precision": 0.954983922829582, "recall": 0.9252336448598131, "support": 321}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9671179883945841, "precision": 1.0, "recall": 0.9363295880149812, "support": 267}, "\u2423": {"f1-score": 0.8799126637554585, "precision": 1.0, "recall": 0.7855750487329435, "support": 513}},
  "cl_report_full": {"\u0027": {"f1-score": 0.8382004735595896, "precision": 1.0, "recall": 0.7214673913043478, "support": 736}, "macro avg": {"f1-score": 0.6420836200817801, "precision": 0.8384627553386746, "recall": 0.5523840646130267, "support": 9109}, "micro avg": {"f1-score": 0.7244526799807838, "precision": 0.9663127059685097, "recall": 0.5794269403886266, "support": 9109}, "weighted avg": {"f1-score": 0.6886260281010927, "precision": 0.9511154526584574, "recall": 0.5794269403886266, "support": 9109}, "\u2205": {"f1-score": 0.815842523419898, "precision": 0.9571508069003896, "recall": 0.7108906798925397, "support": 4839}, "\u23ce": {"f1-score": 0.7300613496932515, "precision": 0.9571045576407506, "recall": 0.5900826446280992, "support": 605}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 197}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.922360248447205, "precision": 0.954983922829582, "recall": 0.8918918918918919, "support": 333}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8620689655172413, "precision": 1.0, "recall": 0.7575757575757576, "support": 330}, "\u2423": {"f1-score": 0.32605177993527507, "precision": 1.0, "recall": 0.19478008699855, "support": 2069}},
  "ppcr": 0.5996267427818641
}
```
</details>
