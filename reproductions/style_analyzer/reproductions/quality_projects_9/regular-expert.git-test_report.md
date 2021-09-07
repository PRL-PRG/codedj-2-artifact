# Test report for javascript / file:///tmp/top-repos-quality-repos-fb5u__4v/regular-expert.git HEAD 81456c308df79e9137a6e1c4778e928c6c08c7b3

### Classification report

PPCR: 0.729

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.934| 0.984| 0.846| 0.958| 0.887| 929| 1081| 0.859 |
| `␣` | 0.854| 0.830| 0.510| 0.842| 0.638| 218| 355| 0.614 |
| `"` | 0.929| 0.980| 0.833| 0.954| 0.879| 148| 174| 0.851 |
| `⏎␣⁻␣⁻` | 1.000| 0.675| 0.429| 0.806| 0.600| 40| 63| 0.635 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 20| 108| 0.185 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 16| 73| 0.219 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 31| 0.097 |
| `weighted avg` | 0.896| 0.922| 0.672| 0.908| 0.730| 1374| 1885| 0.729 |
| `macro avg` | 0.531| 0.496| 0.374| 0.509| 0.429| 1374| 1885| 0.729 |
| `micro avg` | 0.922| 0.922| 0.672| 0.922| 0.778| 1374| 1885| 0.729 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|152 |914 |15 |0 |0 |0 |0 |0 |
|137 |32 |181 |5 |0 |0 |0 |0 |
|26 |0 |3 |145 |0 |0 |0 |0 |
|88 |9 |6 |5 |0 |0 |0 |0 |
|57 |9 |6 |1 |0 |0 |0 |0 |
|23 |12 |1 |0 |0 |0 |27 |0 |
|28 |3 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9539473684210527, "precision": 0.9294871794871795, "recall": 0.9797297297297297, "support": 148}, "macro avg": {"f1-score": 0.5085498945167227, "precision": 0.5309809263593449, "recall": 0.49555122358793074, "support": 1374}, "micro avg": {"f1-score": 0.9221251819505094, "precision": 0.9221251819505094, "recall": 0.9221251819505094, "support": 1374}, "weighted avg": {"f1-score": 0.9075668238145298, "precision": 0.8959290087826839, "recall": 0.9221251819505094, "support": 1374}, "\u2205": {"f1-score": 0.9580712788259957, "precision": 0.933605720122574, "recall": 0.9838536060279871, "support": 929}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 20}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8059701492537313, "precision": 1.0, "recall": 0.675, "support": 40}, "\u2423": {"f1-score": 0.841860465116279, "precision": 0.8537735849056604, "recall": 0.8302752293577982, "support": 218}},
  "cl_report_full": {"\"": {"f1-score": 0.8787878787878788, "precision": 0.9294871794871795, "recall": 0.8333333333333334, "support": 174}, "macro avg": {"f1-score": 0.42923064162084046, "precision": 0.5309809263593449, "recall": 0.37389676147719314, "support": 1885}, "micro avg": {"f1-score": 0.7775391224301933, "precision": 0.9221251819505094, "recall": 0.6721485411140583, "support": 1885}, "weighted avg": {"f1-score": 0.7302994331941993, "precision": 0.8154101725860908, "recall": 0.6721485411140583, "support": 1885}, "\u2205": {"f1-score": 0.887378640776699, "precision": 0.933605720122574, "recall": 0.845513413506013, "support": 1081}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 108}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 31}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 73}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.6, "precision": 1.0, "recall": 0.42857142857142855, "support": 63}, "\u2423": {"f1-score": 0.6384479717813051, "precision": 0.8537735849056604, "recall": 0.5098591549295775, "support": 355}},
  "ppcr": 0.7289124668435013
}
```
</details>
