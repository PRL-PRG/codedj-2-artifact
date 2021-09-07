# Test report for javascript / file:///tmp/top-repos-quality-repos-q45bejf2/axios HEAD 21ae22dbd3ae3d3a55d9efd4eead3dd7fb6d8e6e

### Classification report

PPCR: 0.872

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.977| 0.979| 0.894| 0.978| 0.934| 2863| 3135| 0.913 |
| `␣` | 0.930| 0.965| 0.874| 0.947| 0.901| 1001| 1105| 0.906 |
| `'` | 0.960| 0.974| 0.968| 0.967| 0.964| 614| 618| 0.994 |
| `⏎␣⁺␣⁺` | 0.986| 0.945| 0.920| 0.965| 0.952| 219| 225| 0.973 |
| `⏎␣⁻␣⁻` | 1.000| 0.970| 0.743| 0.985| 0.853| 164| 214| 0.766 |
| `⏎⏎` | 1.000| 0.788| 0.323| 0.881| 0.488| 66| 161| 0.410 |
| `⏎` | 1.000| 0.449| 0.088| 0.620| 0.162| 49| 249| 0.197 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `weighted avg` | 0.967| 0.966| 0.842| 0.965| 0.882| 4976| 5707| 0.872 |
| `macro avg` | 0.856| 0.759| 0.601| 0.793| 0.657| 4976| 5707| 0.872 |
| `micro avg` | 0.966| 0.966| 0.842| 0.966| 0.900| 4976| 5707| 0.872 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|272 |2804 |58 |0 |0 |1 |0 |0 |
|104 |9 |966 |24 |0 |2 |0 |0 |
|4 |16 |0 |598 |0 |0 |0 |0 |
|200 |13 |14 |0 |22 |0 |0 |0 |
|6 |10 |1 |1 |0 |207 |0 |0 |
|50 |5 |0 |0 |0 |0 |159 |0 |
|95 |14 |0 |0 |0 |0 |0 |52 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.9668552950687146, "precision": 0.9598715890850722, "recall": 0.9739413680781759, "support": 614}, "macro avg": {"f1-score": 0.7928211575608576, "precision": 0.856498649138005, "recall": 0.7587430791623229, "support": 4976}, "micro avg": {"f1-score": 0.9662379421221865, "precision": 0.9662379421221865, "recall": 0.9662379421221865, "support": 4976}, "weighted avg": {"f1-score": 0.9652497139554062, "precision": 0.9668587527493724, "recall": 0.9662379421221865, "support": 4976}, "\u2205": {"f1-score": 0.978025810952215, "precision": 0.9766631835597352, "recall": 0.9793922458959133, "support": 2863}, "\u23ce": {"f1-score": 0.6197183098591549, "precision": 1.0, "recall": 0.4489795918367347, "support": 49}, "\u23ce\u23ce": {"f1-score": 0.8813559322033898, "precision": 1.0, "recall": 0.7878787878787878, "support": 66}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9650349650349651, "precision": 0.9857142857142858, "recall": 0.9452054794520548, "support": 219}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9845201238390092, "precision": 1.0, "recall": 0.9695121951219512, "support": 164}, "\u2423": {"f1-score": 0.9470588235294118, "precision": 0.9297401347449471, "recall": 0.965034965034965, "support": 1001}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.9637389202256246, "precision": 0.9598715890850722, "recall": 0.9676375404530745, "support": 618}, "macro avg": {"f1-score": 0.6566858557723882, "precision": 0.856498649138005, "recall": 0.6013236228010279, "support": 5707}, "micro avg": {"f1-score": 0.9001216886642329, "precision": 0.9662379421221865, "recall": 0.8424741545470474, "support": 5707}, "weighted avg": {"f1-score": 0.8821099850041337, "precision": 0.9686680016984801, "recall": 0.8424741545470474, "support": 5707}, "\u2205": {"f1-score": 0.9337329337329336, "precision": 0.9766631835597352, "recall": 0.8944178628389154, "support": 3135}, "\u23ce": {"f1-score": 0.16236162361623616, "precision": 1.0, "recall": 0.08835341365461848, "support": 249}, "\u23ce\u23ce": {"f1-score": 0.48826291079812206, "precision": 1.0, "recall": 0.32298136645962733, "support": 161}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9517241379310346, "precision": 0.9857142857142858, "recall": 0.92, "support": 225}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8525469168900804, "precision": 1.0, "recall": 0.7429906542056075, "support": 214}, "\u2423": {"f1-score": 0.9011194029850746, "precision": 0.9297401347449471, "recall": 0.8742081447963801, "support": 1105}},
  "ppcr": 0.8719116874014369
}
```
</details>