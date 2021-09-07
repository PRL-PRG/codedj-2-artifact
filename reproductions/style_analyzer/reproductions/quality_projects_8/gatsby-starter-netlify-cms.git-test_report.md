# Test report for javascript / file:///tmp/top-repos-quality-repos-o55xlgl9/gatsby-starter-netlify-cms.git HEAD 5c5addbb565f1aae3c1ea15a37b17f42eb65ba6a

### Classification report

PPCR: 0.810

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.937| 0.980| 0.929| 0.958| 0.933| 882| 930| 0.948 |
| `␣` | 0.821| 0.743| 0.431| 0.780| 0.565| 167| 288| 0.580 |
| `'` | 1.000| 1.000| 0.957| 1.000| 0.978| 88| 92| 0.957 |
| `"` | 1.000| 1.000| 1.000| 1.000| 1.000| 76| 76| 1.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 19| 28| 0.679 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 26| 0.192 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 66| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 22| 0.000 |
| `weighted avg` | 0.912| 0.931| 0.754| 0.921| 0.783| 1237| 1528| 0.810 |
| `micro avg` | 0.931| 0.931| 0.754| 0.931| 0.833| 1237| 1528| 0.810 |
| `macro avg` | 0.470| 0.465| 0.415| 0.467| 0.434| 1237| 1528| 0.810 |

### Confusion matrix

|refusal|  ∅| ␣| '| "| ⏎| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|48 |864 |18 |0 |0 |0 |0 |0 |0 |
|121 |43 |124 |0 |0 |0 |0 |0 |0 |
|4 |0 |0 |88 |0 |0 |0 |0 |0 |
|0 |0 |0 |0 |76 |0 |0 |0 |0 |
|66 |0 |0 |0 |0 |0 |0 |0 |0 |
|9 |10 |9 |0 |0 |0 |0 |0 |0 |
|21 |5 |0 |0 |0 |0 |0 |0 |0 |
|22 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 76}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 88}, "macro avg": {"f1-score": 0.46721820134153313, "precision": 0.46978566605852523, "recall": 0.46526335084932174, "support": 1237}, "micro avg": {"f1-score": 0.931285367825384, "precision": 0.931285367825384, "recall": 0.931285367825384, "support": 1237}, "weighted avg": {"f1-score": 0.9208420095171999, "precision": 0.9116049650995628, "recall": 0.931285367825384, "support": 1237}, "\u2205": {"f1-score": 0.9578713968957872, "precision": 0.9370932754880694, "recall": 0.9795918367346939, "support": 882}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 19}, "\u2423": {"f1-score": 0.7798742138364779, "precision": 0.8211920529801324, "recall": 0.7425149700598802, "support": 167}},
  "cl_report_full": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 76}, "\u0027": {"f1-score": 0.9777777777777777, "precision": 1.0, "recall": 0.9565217391304348, "support": 92}, "macro avg": {"f1-score": 0.43446792593722344, "precision": 0.46978566605852523, "recall": 0.41451369409381333, "support": 1528}, "micro avg": {"f1-score": 0.8332730560578662, "precision": 0.931285367825384, "recall": 0.7539267015706806, "support": 1528}, "weighted avg": {"f1-score": 0.7829743296501408, "precision": 0.8350785716375541, "recall": 0.7539267015706806, "support": 1528}, "\u2205": {"f1-score": 0.9330453563714903, "precision": 0.9370932754880694, "recall": 0.9290322580645162, "support": 930}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 66}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 26}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 28}, "\u2423": {"f1-score": 0.5649202733485194, "precision": 0.8211920529801324, "recall": 0.4305555555555556, "support": 288}},
  "ppcr": 0.8095549738219895
}
```
</details>
