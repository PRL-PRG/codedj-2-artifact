# Test report for javascript / file:///tmp/top-repos-quality-repos-x46pxq3p/uop-backend.git HEAD f5ca4fa230a01423236ad95753710e2efe37c320

### Classification report

PPCR: 0.929

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.904| 1.000| 1.000| 0.949| 0.949| 188| 188| 1.000 |
| `␣` | 0.941| 0.783| 0.767| 0.855| 0.845| 143| 146| 0.979 |
| `'` | 1.000| 0.944| 0.944| 0.971| 0.971| 36| 36| 1.000 |
| `⏎␣⁺␣⁺` | 0.522| 0.706| 0.706| 0.600| 0.600| 17| 17| 1.000 |
| `⏎␣⁻␣⁻` | 0.895| 1.000| 1.000| 0.944| 0.944| 17| 17| 1.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 19| 0.105 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 11| 0.000 |
| `micro avg` | 0.901| 0.901| 0.836| 0.901| 0.867| 403| 434| 0.929 |
| `weighted avg` | 0.905| 0.901| 0.836| 0.898| 0.837| 403| 434| 0.929 |
| `macro avg` | 0.609| 0.633| 0.631| 0.617| 0.616| 403| 434| 0.929 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|0 |188 |0 |0 |0 |0 |0 |0 |
|3 |18 |112 |0 |0 |11 |2 |0 |
|17 |2 |0 |0 |0 |0 |0 |0 |
|0 |0 |2 |0 |34 |0 |0 |0 |
|0 |0 |5 |0 |0 |12 |0 |0 |
|0 |0 |0 |0 |0 |0 |17 |0 |
|11 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.9714285714285714, "precision": 1.0, "recall": 0.9444444444444444, "support": 36}, "macro avg": {"f1-score": 0.6171899710612906, "precision": 0.6087855138534907, "recall": 0.6333633686574863, "support": 403}, "micro avg": {"f1-score": 0.9007444168734491, "precision": 0.9007444168734491, "recall": 0.9007444168734491, "support": 403}, "weighted avg": {"f1-score": 0.8982421255999193, "precision": 0.9046933095542813, "recall": 0.9007444168734491, "support": 403}, "\u2205": {"f1-score": 0.9494949494949495, "precision": 0.9038461538461539, "recall": 1.0, "support": 188}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.6, "precision": 0.5217391304347826, "recall": 0.7058823529411765, "support": 17}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9444444444444444, "precision": 0.8947368421052632, "recall": 1.0, "support": 17}, "\u2423": {"f1-score": 0.8549618320610688, "precision": 0.9411764705882353, "recall": 0.7832167832167832, "support": 143}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9714285714285714, "precision": 1.0, "recall": 0.9444444444444444, "support": 36}, "macro avg": {"f1-score": 0.61580728346227, "precision": 0.6087855138534907, "recall": 0.6310642978652649, "support": 434}, "micro avg": {"f1-score": 0.8673835125448028, "precision": 0.9007444168734491, "recall": 0.836405529953917, "support": 434}, "weighted avg": {"f1-score": 0.8367358419049576, "precision": 0.8465781870095394, "recall": 0.836405529953917, "support": 434}, "\u2205": {"f1-score": 0.9494949494949495, "precision": 0.9038461538461539, "recall": 1.0, "support": 188}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 19}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.6, "precision": 0.5217391304347826, "recall": 0.7058823529411765, "support": 17}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9444444444444444, "precision": 0.8947368421052632, "recall": 1.0, "support": 17}, "\u2423": {"f1-score": 0.8452830188679246, "precision": 0.9411764705882353, "recall": 0.7671232876712328, "support": 146}},
  "ppcr": 0.9285714285714286
}
```
</details>
