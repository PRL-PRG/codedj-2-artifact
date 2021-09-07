# Test report for javascript / file:///tmp/top-repos-quality-repos-yyo5obn_/whatsfordinner3.git HEAD 52f4c0e9e4995cd22746d0c8bb8c28c7a26f5342

### Classification report

PPCR: 0.595

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.940| 0.993| 0.722| 0.966| 0.816| 282| 388| 0.727 |
| `␣` | 0.983| 0.887| 0.504| 0.933| 0.667| 133| 234| 0.568 |
| `'` | 1.000| 1.000| 0.500| 1.000| 0.667| 51| 102| 0.500 |
| `⏎` | 0.867| 1.000| 0.406| 0.929| 0.553| 26| 64| 0.406 |
| `"` | 1.000| 1.000| 1.000| 1.000| 1.000| 8| 8| 1.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 16| 0.250 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 19| 0.105 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 19| 0.053 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1| 0.000 |
| `macro avg` | 0.479| 0.488| 0.313| 0.483| 0.370| 507| 852| 0.595 |
| `micro avg` | 0.953| 0.953| 0.567| 0.953| 0.711| 507| 852| 0.595 |
| `weighted avg` | 0.941| 0.953| 0.567| 0.946| 0.686| 507| 852| 0.595 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⇥⁻| ⏎⇥⁺| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|106 |280 |2 |0 |0 |0 |0 |0 |0 |0 |0 |
|101 |15 |118 |0 |0 |0 |0 |0 |0 |0 |0 |
|51 |0 |0 |51 |0 |0 |0 |0 |0 |0 |0 |
|38 |0 |0 |0 |26 |0 |0 |0 |0 |0 |0 |
|0 |0 |0 |0 |0 |8 |0 |0 |0 |0 |0 |
|1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|17 |2 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|18 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|12 |0 |0 |0 |4 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 8}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 51}, "macro avg": {"f1-score": 0.48268949940614103, "precision": 0.47895973154362415, "recall": 0.48801258465312214, "support": 507}, "micro avg": {"f1-score": 0.9526627218934911, "precision": 0.9526627218934911, "recall": 0.9526627218934911, "support": 507}, "weighted avg": {"f1-score": 0.9457237876105367, "precision": 0.9413868039836032, "recall": 0.9526627218934911, "support": 507}, "\u2205": {"f1-score": 0.9655172413793103, "precision": 0.9395973154362416, "recall": 0.9929078014184397, "support": 282}, "\u23ce": {"f1-score": 0.9285714285714286, "precision": 0.8666666666666667, "recall": 1.0, "support": 26}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9328063241106719, "precision": 0.9833333333333333, "recall": 0.8872180451127819, "support": 133}},
  "cl_report_full": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 8}, "\u0027": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 102}, "macro avg": {"f1-score": 0.37028513533072804, "precision": 0.47895973154362415, "recall": 0.31321729888095867, "support": 852}, "micro avg": {"f1-score": 0.7108167770419426, "precision": 0.9526627218934911, "recall": 0.5669014084507042, "support": 852}, "weighted avg": {"f1-score": 0.685609095301291, "precision": 0.8921718603942822, "recall": 0.5669014084507042, "support": 852}, "\u2205": {"f1-score": 0.8163265306122448, "precision": 0.9395973154362416, "recall": 0.7216494845360825, "support": 388}, "\u23ce": {"f1-score": 0.5531914893617021, "precision": 0.8666666666666667, "recall": 0.40625, "support": 64}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 19}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 19}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.6666666666666665, "precision": 0.9833333333333333, "recall": 0.5042735042735043, "support": 234}},
  "ppcr": 0.5950704225352113
}
```
</details>
