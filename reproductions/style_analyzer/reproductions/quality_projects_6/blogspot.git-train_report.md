# Train report for javascript / file:///tmp/top-repos-quality-repos-jjdfw5jt/blogspot.git HEAD 8b0bf09fb493cf5af051f1c061f38ede9dee187c

### Classification report

PPCR: 0.523

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.981| 1.000| 0.913| 0.991| 0.946| 10872| 11904| 0.913 |
| `␣` | 1.000| 0.620| 0.050| 0.765| 0.096| 434| 5357| 0.081 |
| `⏎` | 0.935| 0.929| 0.130| 0.932| 0.229| 169| 1206| 0.140 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 23| 488| 0.047 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 316| 0.016 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 535| 0.006 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 179| 0.017 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 128| 0.023 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 199| 0.005 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 126| 0.008 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1300| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 284| 0.000 |
| `micro avg` | 0.981| 0.981| 0.513| 0.981| 0.674| 11514| 22022| 0.523 |
| `weighted avg` | 0.978| 0.981| 0.513| 0.978| 0.547| 11514| 22022| 0.523 |
| `macro avg` | 0.243| 0.212| 0.091| 0.224| 0.106| 11514| 22022| 0.523 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⇥⁻| ⏎⇥⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1032 |10872 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|4923 |165 |269 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1300 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1037 |12 |0 |0 |157 |0 |0 |0 |0 |0 |0 |0 |0 |
|532 |3 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|465 |18 |0 |0 |5 |0 |0 |0 |0 |0 |0 |0 |0 |
|284 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|311 |0 |0 |0 |5 |0 |0 |0 |0 |0 |0 |0 |0 |
|198 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|176 |3 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|125 |3 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|125 |0 |0 |0 |1 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| jquery.mockjax.js | 163 |
| jquery.autocomplete.js | 13 |
| cslider.js | 9 |
| gmap.js | 7 |
| sameheight.js | 6 |
| sticky-kit.js | 6 |
| slidemenu.js | 5 |
| sticky.js | 3 |
| collapse.js | 2 |
| pagination.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.22397520966556891, "precision": 0.24300141613324336, "recall": 0.21240081258691682, "support": 11514}, "micro avg": {"f1-score": 0.9812402292860865, "precision": 0.9812402292860865, "recall": 0.9812402292860865, "support": 11514}, "weighted avg": {"f1-score": 0.9779450899678013, "precision": 0.9781768647797573, "recall": 0.9812402292860865, "support": 11514}, "\u2205": {"f1-score": 0.9906601667501935, "precision": 0.9814931840751105, "recall": 1.0, "support": 10872}, "\u23ce": {"f1-score": 0.9317507418397627, "precision": 0.9345238095238095, "recall": 0.9289940828402367, "support": 169}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 23}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u2423": {"f1-score": 0.7652916073968705, "precision": 1.0, "recall": 0.619815668202765, "support": 434}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 284}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1300}, "macro avg": {"f1-score": 0.10586085077947234, "precision": 0.24300141613324336, "recall": 0.0911419621026137, "support": 22022}, "micro avg": {"f1-score": 0.673783396946565, "precision": 0.9812402292860865, "recall": 0.5130324221233312, "support": 22022}, "weighted avg": {"f1-score": 0.5472312101143704, "precision": 0.8249809543872413, "recall": 0.5130324221233312, "support": 22022}, "\u2205": {"f1-score": 0.9461729254601627, "precision": 0.9814931840751105, "recall": 0.9133064516129032, "support": 11904}, "\u23ce": {"f1-score": 0.22852983988355163, "precision": 0.9345238095238095, "recall": 0.13018242122719734, "support": 1206}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 126}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 128}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 316}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 535}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 199}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 488}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 179}, "\u2423": {"f1-score": 0.09562744400995378, "precision": 1.0, "recall": 0.050214672391263766, "support": 5357}},
  "ppcr": 0.5228407955680683
}
```
</details>
