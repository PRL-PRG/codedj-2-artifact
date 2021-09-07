# Test report for javascript / file:///tmp/top-repos-quality-repos-jjdfw5jt/blogspot.git HEAD 8b0bf09fb493cf5af051f1c061f38ede9dee187c

### Classification report

PPCR: 0.804

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.980| 0.950| 0.913| 0.965| 0.945| 2761| 2872| 0.961 |
| `␣` | 0.820| 0.996| 0.988| 0.899| 0.896| 768| 774| 0.992 |
| `⏎` | 0.984| 0.953| 0.469| 0.968| 0.635| 127| 258| 0.492 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 46| 348| 0.132 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 18| 180| 0.100 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 13| 164| 0.079 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 43| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 3| 0.000 |
| `micro avg` | 0.940| 0.940| 0.756| 0.940| 0.838| 3733| 4642| 0.804 |
| `weighted avg` | 0.927| 0.940| 0.756| 0.931| 0.770| 3733| 4642| 0.804 |
| `macro avg` | 0.232| 0.242| 0.198| 0.236| 0.206| 3733| 4642| 0.804 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⇥⁻| ⏎⇥⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|111 |2623 |138 |0 |0 |0 |0 |0 |0 |
|6 |3 |765 |0 |0 |0 |0 |0 |0 |
|302 |43 |3 |0 |0 |0 |0 |0 |0 |
|131 |4 |2 |0 |121 |0 |0 |0 |0 |
|162 |2 |16 |0 |0 |0 |0 |0 |0 |
|151 |2 |9 |0 |2 |0 |0 |0 |0 |
|43 |0 |0 |0 |0 |0 |0 |0 |0 |
|3 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 46}, "macro avg": {"f1-score": 0.23601365010608602, "precision": 0.2319586412145004, "recall": 0.24157231374103919, "support": 3733}, "micro avg": {"f1-score": 0.9399946423787838, "precision": 0.9399946423787838, "recall": 0.9399946423787838, "support": 3733}, "weighted avg": {"f1-score": 0.9314885487997161, "precision": 0.9268553807163362, "recall": 0.9399946423787838, "support": 3733}, "\u2205": {"f1-score": 0.9646929018021332, "precision": 0.9798281658573029, "recall": 0.9500181093806592, "support": 2761}, "\u23ce": {"f1-score": 0.968, "precision": 0.983739837398374, "recall": 0.952755905511811, "support": 127}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 18}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 13}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.8994708994708994, "precision": 0.819935691318328, "recall": 0.99609375, "support": 768}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 348}, "macro avg": {"f1-score": 0.20640629041919353, "precision": 0.2319586412145004, "recall": 0.19755543139498896, "support": 4642}, "micro avg": {"f1-score": 0.8379701492537314, "precision": 0.9399946423787838, "recall": 0.7559241706161137, "support": 4642}, "weighted avg": {"f1-score": 0.7696668446786561, "precision": 0.7976091330183845, "recall": 0.7559241706161137, "support": 4642}, "\u2205": {"f1-score": 0.9453955667687871, "precision": 0.9798281658573029, "recall": 0.9133008356545961, "support": 2872}, "\u23ce": {"f1-score": 0.6351706036745407, "precision": 0.983739837398374, "recall": 0.4689922480620155, "support": 258}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 43}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 180}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 164}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u2423": {"f1-score": 0.8963093145869947, "precision": 0.819935691318328, "recall": 0.9883720930232558, "support": 774}},
  "ppcr": 0.8041792330891857
}
```
</details>
