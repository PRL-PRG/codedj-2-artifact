# Test report for javascript / file:///tmp/top-repos-quality-repos-j1zadpqs/certificates-for-everyone-netlify.git HEAD 04db893536e992b028c2f4cfe2e79bfbcd3e9b1b

### Classification report

PPCR: 0.744

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.963| 0.985| 0.823| 0.974| 0.887| 731| 875| 0.835 |
| `␣` | 0.911| 0.786| 0.505| 0.844| 0.650| 248| 386| 0.642 |
| `"` | 0.848| 1.000| 1.000| 0.918| 0.918| 206| 206| 1.000 |
| `⏎⏎` | 0.912| 1.000| 0.861| 0.954| 0.886| 31| 36| 0.861 |
| `⏎` | 1.000| 0.789| 0.147| 0.882| 0.256| 19| 102| 0.186 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 39| 0.256 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 41| 0.220 |
| `weighted avg` | 0.918| 0.931| 0.693| 0.922| 0.756| 1254| 1685| 0.744 |
| `micro avg` | 0.931| 0.931| 0.693| 0.931| 0.794| 1254| 1685| 0.744 |
| `macro avg` | 0.662| 0.652| 0.477| 0.653| 0.514| 1254| 1685| 0.744 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|144 |720 |11 |0 |0 |0 |0 |0 |
|138 |16 |195 |37 |0 |0 |0 |0 |
|0 |0 |0 |206 |0 |0 |0 |0 |
|83 |0 |1 |0 |15 |0 |0 |3 |
|32 |9 |0 |0 |0 |0 |0 |0 |
|29 |3 |7 |0 |0 |0 |0 |0 |
|5 |0 |0 |0 |0 |0 |0 |31 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9175946547884187, "precision": 0.8477366255144033, "recall": 1.0, "support": 206}, "macro avg": {"f1-score": 0.65308291794427, "precision": 0.6618975899410815, "recall": 0.6515308753106012, "support": 1254}, "micro avg": {"f1-score": 0.9306220095693781, "precision": 0.930622009569378, "recall": 0.930622009569378, "support": 1254}, "weighted avg": {"f1-score": 0.9221955524877845, "precision": 0.9182744200844487, "recall": 0.930622009569378, "support": 1254}, "\u2205": {"f1-score": 0.9736308316430019, "precision": 0.9625668449197861, "recall": 0.9849521203830369, "support": 731}, "\u23ce": {"f1-score": 0.8823529411764706, "precision": 1.0, "recall": 0.7894736842105263, "support": 19}, "\u23ce\u23ce": {"f1-score": 0.9538461538461539, "precision": 0.9117647058823529, "recall": 1.0, "support": 31}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u2423": {"f1-score": 0.8441558441558442, "precision": 0.9112149532710281, "recall": 0.7862903225806451, "support": 248}},
  "cl_report_full": {"\"": {"f1-score": 0.9175946547884187, "precision": 0.8477366255144033, "recall": 1.0, "support": 206}, "macro avg": {"f1-score": 0.5138521482782973, "precision": 0.6618975899410815, "recall": 0.4766012035211321, "support": 1685}, "micro avg": {"f1-score": 0.7941476692752637, "precision": 0.930622009569378, "recall": 0.6925816023738872, "support": 1685}, "weighted avg": {"f1-score": 0.7562636025113975, "precision": 0.8922446501692353, "recall": 0.6925816023738872, "support": 1685}, "\u2205": {"f1-score": 0.88724584103512, "precision": 0.9625668449197861, "recall": 0.8228571428571428, "support": 875}, "\u23ce": {"f1-score": 0.25641025641025644, "precision": 1.0, "recall": 0.14705882352941177, "support": 102}, "\u23ce\u23ce": {"f1-score": 0.8857142857142858, "precision": 0.9117647058823529, "recall": 0.8611111111111112, "support": 36}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 39}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 41}, "\u2423": {"f1-score": 0.65, "precision": 0.9112149532710281, "recall": 0.5051813471502591, "support": 386}},
  "ppcr": 0.7442136498516321
}
```
</details>
