# Train report for javascript / file:///tmp/top-repos-quality-repos-lt4zbk3p/donde_front_end.git HEAD e7fdae24d6bbaeb1f8463be9c336946fc24365a9

### Classification report

PPCR: 0.634

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.970| 0.994| 0.866| 0.982| 0.915| 12853| 14759| 0.871 |
| `␣` | 0.954| 0.876| 0.340| 0.913| 0.501| 2114| 5448| 0.388 |
| `⏎` | 0.932| 0.892| 0.472| 0.911| 0.626| 600| 1134| 0.529 |
| `'` | 1.000| 1.000| 0.177| 1.000| 0.300| 205| 1160| 0.177 |
| `⏎⏎` | 0.914| 0.779| 0.366| 0.841| 0.523| 190| 404| 0.470 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 47| 666| 0.071 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 39| 657| 0.059 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1088| 0.000 |
| `micro avg` | 0.967| 0.967| 0.613| 0.967| 0.750| 16048| 25316| 0.634 |
| `weighted avg` | 0.961| 0.967| 0.613| 0.964| 0.691| 16048| 25316| 0.634 |
| `macro avg` | 0.596| 0.568| 0.278| 0.581| 0.358| 16048| 25316| 0.634 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1906 |12776 |74 |3 |0 |0 |0 |0 |0 |
|3334 |249 |1852 |0 |0 |0 |0 |0 |13 |
|534 |58 |6 |535 |0 |0 |0 |0 |1 |
|955 |0 |0 |0 |205 |0 |0 |0 |0 |
|1088 |0 |0 |0 |0 |0 |0 |0 |0 |
|619 |41 |2 |4 |0 |0 |0 |0 |0 |
|618 |27 |5 |7 |0 |0 |0 |0 |0 |
|214 |15 |2 |25 |0 |0 |0 |0 |148 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/serviceWorker.js | 57 |
| src/pages/ItemPage.js | 41 |
| src/components/SpaceDisplay.js | 29 |
| src/reducers/appReducer.js | 27 |
| src/components/Search.js | 25 |
| src/components/ContainerDisplay.js | 24 |
| src/pages/ItemsPage.js | 22 |
| src/containers/HouseholdMessagesContainer.js | 16 |
| src/pages/HouseholdPage.js | 15 |
| src/containers/HouseholdContainer.js | 15 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 205}, "macro avg": {"f1-score": 0.5809768544031525, "precision": 0.5962701987213548, "recall": 0.5675859436052136, "support": 16048}, "micro avg": {"f1-score": 0.9668494516450648, "precision": 0.9668494516450648, "recall": 0.9668494516450648, "support": 16048}, "weighted avg": {"f1-score": 0.9636679096479595, "precision": 0.9613131104227323, "recall": 0.9668494516450648, "support": 16048}, "\u2205": {"f1-score": 0.9820515776932242, "precision": 0.9703782469998481, "recall": 0.994009180736015, "support": 12853}, "\u23ce": {"f1-score": 0.9114139693356049, "precision": 0.9320557491289199, "recall": 0.8916666666666667, "support": 600}, "\u23ce\u23ce": {"f1-score": 0.8409090909090909, "precision": 0.9135802469135802, "recall": 0.7789473684210526, "support": 190}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 47}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 39}, "\u2423": {"f1-score": 0.9134401972872996, "precision": 0.9541473467284904, "recall": 0.8760643330179754, "support": 2114}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1088}, "\u0027": {"f1-score": 0.30036630036630035, "precision": 1.0, "recall": 0.17672413793103448, "support": 1160}, "macro avg": {"f1-score": 0.35826328435137494, "precision": 0.5962701987213548, "recall": 0.27755308039614335, "support": 25316}, "micro avg": {"f1-score": 0.7502175805047868, "precision": 0.9668494516450648, "recall": 0.6128930320745773, "support": 25316}, "weighted avg": {"f1-score": 0.691496827670261, "precision": 0.8732044924835225, "recall": 0.6128930320745773, "support": 25316}, "\u2205": {"f1-score": 0.915022381378693, "precision": 0.9703782469998481, "recall": 0.8656413036113558, "support": 14759}, "\u23ce": {"f1-score": 0.6264637002341921, "precision": 0.9320557491289199, "recall": 0.47178130511463845, "support": 1134}, "\u23ce\u23ce": {"f1-score": 0.5229681978798587, "precision": 0.9135802469135802, "recall": 0.36633663366336633, "support": 404}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 666}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 657}, "\u2423": {"f1-score": 0.5012856949519555, "precision": 0.9541473467284904, "recall": 0.3399412628487518, "support": 5448}},
  "ppcr": 0.633907410333386
}
```
</details>
