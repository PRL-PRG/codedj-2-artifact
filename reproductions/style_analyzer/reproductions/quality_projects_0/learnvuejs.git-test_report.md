# Test report for javascript / file:///tmp/top-repos-quality-repos-6vi0e0f1/learnvuejs.git HEAD 714a4570c202aa412e8a5dbe044aed6271cdb685

### Classification report

PPCR: 0.931

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.949| 0.971| 0.953| 0.959| 0.951| 1732| 1764| 0.982 |
| `␣` | 0.858| 0.901| 0.851| 0.879| 0.854| 899| 952| 0.944 |
| `'` | 0.882| 0.908| 0.890| 0.895| 0.886| 347| 354| 0.980 |
| `⏎` | 0.862| 0.838| 0.617| 0.850| 0.719| 290| 394| 0.736 |
| `⏎␣⁺␣⁺` | 0.903| 0.896| 0.846| 0.900| 0.874| 135| 143| 0.944 |
| `⏎␣⁻␣⁻` | 0.877| 0.931| 0.864| 0.903| 0.871| 130| 140| 0.929 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 56| 56| 1.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 38| 93| 0.409 |
| `macro avg` | 0.666| 0.681| 0.628| 0.673| 0.644| 3627| 3896| 0.931 |
| `micro avg` | 0.907| 0.907| 0.845| 0.907| 0.875| 3627| 3896| 0.931 |
| `weighted avg` | 0.884| 0.907| 0.845| 0.895| 0.856| 3627| 3896| 0.931 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|32 |1681 |36 |0 |7 |2 |6 |0 |0 |
|53 |57 |810 |0 |10 |11 |11 |0 |0 |
|7 |6 |23 |315 |3 |0 |0 |0 |0 |
|104 |15 |32 |0 |243 |0 |0 |0 |0 |
|8 |5 |7 |2 |0 |121 |0 |0 |0 |
|10 |6 |1 |0 |2 |0 |121 |0 |0 |
|55 |2 |27 |0 |9 |0 |0 |0 |0 |
|0 |0 |8 |40 |8 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 56}, "\u0027": {"f1-score": 0.8948863636363636, "precision": 0.8823529411764706, "recall": 0.9077809798270894, "support": 347}, "macro avg": {"f1-score": 0.673203319290903, "precision": 0.666318522914696, "recall": 0.6805416157799686, "support": 3627}, "micro avg": {"f1-score": 0.9073614557485525, "precision": 0.9073614557485525, "recall": 0.9073614557485525, "support": 3627}, "weighted avg": {"f1-score": 0.8954494097232818, "precision": 0.8840365227094451, "recall": 0.9073614557485525, "support": 3627}, "\u2205": {"f1-score": 0.9594748858447488, "precision": 0.948645598194131, "recall": 0.9705542725173211, "support": 1732}, "\u23ce": {"f1-score": 0.8496503496503496, "precision": 0.8617021276595744, "recall": 0.8379310344827586, "support": 290}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 38}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.899628252788104, "precision": 0.9029850746268657, "recall": 0.8962962962962963, "support": 135}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9029850746268656, "precision": 0.8768115942028986, "recall": 0.9307692307692308, "support": 130}, "\u2423": {"f1-score": 0.8790016277807922, "precision": 0.8580508474576272, "recall": 0.9010011123470523, "support": 899}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 56}, "\u0027": {"f1-score": 0.8860759493670887, "precision": 0.8823529411764706, "recall": 0.8898305084745762, "support": 354}, "macro avg": {"f1-score": 0.6442978627585914, "precision": 0.666318522914696, "recall": 0.6276011899861391, "support": 3896}, "micro avg": {"f1-score": 0.8749169214409145, "precision": 0.9073614557485525, "recall": 0.8447125256673511, "support": 3896}, "weighted avg": {"f1-score": 0.8558387337636212, "precision": 0.8711548537804155, "recall": 0.8447125256673511, "support": 3896}, "\u2205": {"f1-score": 0.9507918552036199, "precision": 0.948645598194131, "recall": 0.9529478458049887, "support": 1764}, "\u23ce": {"f1-score": 0.7189349112426036, "precision": 0.8617021276595744, "recall": 0.616751269035533, "support": 394}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 93}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8736462093862816, "precision": 0.9029850746268657, "recall": 0.8461538461538461, "support": 143}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8705035971223022, "precision": 0.8768115942028986, "recall": 0.8642857142857143, "support": 140}, "\u2423": {"f1-score": 0.8544303797468353, "precision": 0.8580508474576272, "recall": 0.8508403361344538, "support": 952}},
  "ppcr": 0.9309548254620124
}
```
</details>