# Test report for javascript / file:///tmp/top-repos-quality-repos-w5v778i1/openebench_scientific_visualizer.git HEAD 859040441e881d748992b66079df96fdd3cf31e6

### Classification report

PPCR: 0.861

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.986| 0.987| 0.903| 0.986| 0.942| 694| 759| 0.914 |
| `␣` | 0.954| 0.966| 0.754| 0.960| 0.842| 238| 305| 0.780 |
| `"` | 0.803| 0.990| 0.990| 0.887| 0.887| 198| 198| 1.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 48| 48| 1.000 |
| `⏎` | 0.875| 0.875| 0.394| 0.875| 0.544| 32| 71| 0.451 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 10| 0.200 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 13| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 3| 0.000 |
| `micro avg` | 0.940| 0.940| 0.810| 0.940| 0.870| 1212| 1407| 0.861 |
| `weighted avg` | 0.906| 0.940| 0.810| 0.921| 0.843| 1212| 1407| 0.861 |
| `macro avg` | 0.452| 0.477| 0.380| 0.464| 0.402| 1212| 1407| 0.861 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| '| ⏎⏎| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|65 |685 |7 |0 |2 |0 |0 |0 |0 |
|67 |8 |230 |0 |0 |0 |0 |0 |0 |
|0 |0 |2 |196 |0 |0 |0 |0 |0 |
|39 |2 |2 |0 |28 |0 |0 |0 |0 |
|0 |0 |0 |48 |0 |0 |0 |0 |0 |
|13 |0 |0 |0 |0 |0 |0 |0 |0 |
|3 |0 |0 |0 |0 |0 |0 |0 |0 |
|8 |0 |0 |0 |2 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.8868778280542987, "precision": 0.8032786885245902, "recall": 0.98989898989899, "support": 198}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 48}, "macro avg": {"f1-score": 0.46356661894928886, "precision": 0.45228088072362327, "recall": 0.4772896556011279, "support": 1212}, "micro avg": {"f1-score": 0.9397689768976898, "precision": 0.9397689768976898, "recall": 0.9397689768976898, "support": 1212}, "weighted avg": {"f1-score": 0.9213433567357703, "precision": 0.9061060216812363, "recall": 0.9397689768976898, "support": 1212}, "\u2205": {"f1-score": 0.986321094312455, "precision": 0.9856115107913669, "recall": 0.9870317002881844, "support": 694}, "\u23ce": {"f1-score": 0.875, "precision": 0.875, "recall": 0.875, "support": 32}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9603340292275574, "precision": 0.9543568464730291, "recall": 0.9663865546218487, "support": 238}},
  "cl_report_full": {"\"": {"f1-score": 0.8868778280542987, "precision": 0.8032786885245902, "recall": 0.98989898989899, "support": 198}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 48}, "macro avg": {"f1-score": 0.40191079081991876, "precision": 0.45228088072362327, "recall": 0.3801083551931835, "support": 1407}, "micro avg": {"f1-score": 0.8697976326842307, "precision": 0.9397689768976898, "recall": 0.8095238095238095, "support": 1407}, "weighted avg": {"f1-score": 0.8431519299232823, "precision": 0.8957584614021252, "recall": 0.8095238095238095, "support": 1407}, "\u2205": {"f1-score": 0.9422283356258596, "precision": 0.9856115107913669, "recall": 0.9025032938076416, "support": 759}, "\u23ce": {"f1-score": 0.5436893203883495, "precision": 0.875, "recall": 0.39436619718309857, "support": 71}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 13}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u2423": {"f1-score": 0.8424908424908424, "precision": 0.9543568464730291, "recall": 0.7540983606557377, "support": 305}},
  "ppcr": 0.8614072494669509
}
```
</details>
