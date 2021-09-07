# Test report for javascript / file:///tmp/top-repos-quality-repos-b3hsmgrz/reach.git HEAD 036fde6c2a83f5b014b17b423c72d79730e43a8c

### Classification report

PPCR: 0.775

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.946| 0.936| 0.858| 0.941| 0.900| 471| 514| 0.916 |
| `␣` | 0.881| 0.980| 0.844| 0.928| 0.862| 453| 526| 0.861 |
| `'` | 1.000| 0.700| 0.480| 0.824| 0.649| 70| 102| 0.686 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 11| 66| 0.167 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 41| 0.195 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 39| 0.154 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 27| 0.000 |
| `weighted avg` | 0.898| 0.917| 0.710| 0.904| 0.747| 1019| 1315| 0.775 |
| `micro avg` | 0.917| 0.917| 0.710| 0.917| 0.800| 1019| 1315| 0.775 |
| `macro avg` | 0.404| 0.374| 0.312| 0.385| 0.344| 1019| 1315| 0.775 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|43 |441 |30 |0 |0 |0 |0 |0 |
|73 |9 |444 |0 |0 |0 |0 |0 |
|32 |5 |16 |49 |0 |0 |0 |0 |
|55 |5 |6 |0 |0 |0 |0 |0 |
|33 |0 |8 |0 |0 |0 |0 |0 |
|33 |6 |0 |0 |0 |0 |0 |0 |
|27 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.8235294117647058, "precision": 1.0, "recall": 0.7, "support": 70}, "macro avg": {"f1-score": 0.38467587514760204, "precision": 0.4039006160404076, "recall": 0.37377688325931463, "support": 1019}, "micro avg": {"f1-score": 0.9165848871442591, "precision": 0.9165848871442591, "recall": 0.9165848871442591, "support": 1019}, "weighted avg": {"f1-score": 0.9041598350212268, "precision": 0.897746013962788, "recall": 0.9165848871442591, "support": 1019}, "\u2205": {"f1-score": 0.9413020277481323, "precision": 0.9463519313304721, "recall": 0.9363057324840764, "support": 471}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u2423": {"f1-score": 0.9278996865203761, "precision": 0.8809523809523809, "recall": 0.9801324503311258, "support": 453}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6490066225165563, "precision": 1.0, "recall": 0.4803921568627451, "support": 102}, "macro avg": {"f1-score": 0.3444489349780933, "precision": 0.4039006160404076, "recall": 0.31178218206251, "support": 1315}, "micro avg": {"f1-score": 0.8003427592116539, "precision": 0.9165848871442591, "recall": 0.7102661596958175, "support": 1315}, "weighted avg": {"f1-score": 0.7469826392717261, "precision": 0.7998523536766654, "recall": 0.7102661596958175, "support": 1315}, "\u2205": {"f1-score": 0.8999999999999999, "precision": 0.9463519313304721, "recall": 0.857976653696498, "support": 514}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 66}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 27}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 41}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 39}, "\u2423": {"f1-score": 0.8621359223300971, "precision": 0.8809523809523809, "recall": 0.844106463878327, "support": 526}},
  "ppcr": 0.7749049429657795
}
```
</details>
