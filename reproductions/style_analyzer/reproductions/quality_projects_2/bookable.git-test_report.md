# Test report for javascript / file:///tmp/top-repos-quality-repos-zlnskj7g/bookable.git HEAD 82fe398320ce73f2895f576907461013ba1e454c

### Classification report

PPCR: 0.547

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.917| 1.000| 0.830| 0.957| 0.871| 322| 388| 0.830 |
| `␣` | 1.000| 0.681| 0.196| 0.810| 0.328| 91| 316| 0.288 |
| `'` | 1.000| 1.000| 0.500| 1.000| 0.667| 34| 68| 0.500 |
| `⏎␣⁺␣⁺` | 0.718| 0.933| 0.778| 0.812| 0.747| 30| 36| 0.833 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 52| 0.154 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 29| 0.034 |
| `weighted avg` | 0.909| 0.918| 0.502| 0.906| 0.578| 486| 889| 0.547 |
| `micro avg` | 0.918| 0.918| 0.502| 0.918| 0.649| 486| 889| 0.547 |
| `macro avg` | 0.606| 0.602| 0.384| 0.596| 0.435| 486| 889| 0.547 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|66 |322 |0 |0 |0 |0 |0 |
|225 |18 |62 |0 |0 |11 |0 |
|34 |0 |0 |34 |0 |0 |0 |
|44 |8 |0 |0 |0 |0 |0 |
|6 |2 |0 |0 |0 |28 |0 |
|28 |1 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 34}, "macro avg": {"f1-score": 0.5964935133847095, "precision": 0.6058879392212726, "recall": 0.6024420024420025, "support": 486}, "micro avg": {"f1-score": 0.9176954732510288, "precision": 0.9176954732510288, "recall": 0.9176954732510288, "support": 486}, "weighted avg": {"f1-score": 0.9058112640707241, "precision": 0.9093301912231955, "recall": 0.9176954732510288, "support": 486}, "\u2205": {"f1-score": 0.9569093610698365, "precision": 0.9173789173789174, "recall": 1.0, "support": 322}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8115942028985509, "precision": 0.717948717948718, "recall": 0.9333333333333333, "support": 30}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.8104575163398693, "precision": 1.0, "recall": 0.6813186813186813, "support": 91}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 68}, "macro avg": {"f1-score": 0.4354705939911172, "precision": 0.6058879392212726, "recall": 0.38397953610664043, "support": 889}, "micro avg": {"f1-score": 0.6487272727272727, "precision": 0.9176954732510288, "recall": 0.5016872890888638, "support": 889}, "weighted avg": {"f1-score": 0.5781737853681364, "precision": 0.8614051448697118, "recall": 0.5016872890888638, "support": 889}, "\u2205": {"f1-score": 0.871447902571042, "precision": 0.9173789173789174, "recall": 0.8298969072164949, "support": 388}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 52}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7466666666666666, "precision": 0.717948717948718, "recall": 0.7777777777777778, "support": 36}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 29}, "\u2423": {"f1-score": 0.328042328042328, "precision": 1.0, "recall": 0.1962025316455696, "support": 316}},
  "ppcr": 0.5466816647919011
}
```
</details>
