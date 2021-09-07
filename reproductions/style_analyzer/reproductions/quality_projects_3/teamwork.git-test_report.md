# Test report for javascript / file:///tmp/top-repos-quality-repos-rnv6mo0r/teamwork.git HEAD deba9917cadbe438797e15502a82ffa5a671edbb

### Classification report

PPCR: 0.580

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.846| 0.997| 0.867| 0.915| 0.856| 652| 750| 0.869 |
| `␣` | 1.000| 0.575| 0.210| 0.730| 0.347| 134| 367| 0.365 |
| `"` | 0.941| 0.615| 0.282| 0.744| 0.434| 78| 170| 0.459 |
| `⏎` | 0.574| 0.600| 0.248| 0.587| 0.346| 45| 109| 0.413 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 21| 154| 0.136 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 11| 40| 0.275 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 35| 0.057 |
| `macro avg` | 0.480| 0.398| 0.230| 0.425| 0.283| 943| 1625| 0.580 |
| `weighted avg` | 0.833| 0.850| 0.494| 0.826| 0.542| 943| 1625| 0.580 |
| `micro avg` | 0.850| 0.850| 0.494| 0.850| 0.625| 943| 1625| 0.580 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| '| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|98 |650 |0 |2 |0 |0 |0 |0 |
|233 |56 |77 |1 |0 |0 |0 |0 |
|92 |10 |0 |48 |20 |0 |0 |0 |
|64 |18 |0 |0 |27 |0 |0 |0 |
|133 |21 |0 |0 |0 |0 |0 |0 |
|29 |11 |0 |0 |0 |0 |0 |0 |
|33 |2 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.744186046511628, "precision": 0.9411764705882353, "recall": 0.6153846153846154, "support": 78}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 21}, "macro avg": {"f1-score": 0.42521333512892145, "precision": 0.4802855317658978, "recall": 0.3981348566276687, "support": 943}, "micro avg": {"f1-score": 0.8504772004241782, "precision": 0.8504772004241782, "recall": 0.8504772004241782, "support": 943}, "weighted avg": {"f1-score": 0.8262586547446135, "precision": 0.8325405569483947, "recall": 0.8504772004241782, "support": 943}, "\u2205": {"f1-score": 0.9154929577464788, "precision": 0.8463541666666666, "recall": 0.9969325153374233, "support": 652}, "\u23ce": {"f1-score": 0.5869565217391305, "precision": 0.574468085106383, "recall": 0.6, "support": 45}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u2423": {"f1-score": 0.7298578199052133, "precision": 1.0, "recall": 0.5746268656716418, "support": 134}},
  "cl_report_full": {"\"": {"f1-score": 0.4343891402714932, "precision": 0.9411764705882353, "recall": 0.2823529411764706, "support": 170}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 154}, "macro avg": {"f1-score": 0.2833971171567079, "precision": 0.4802855317658978, "recall": 0.229505042023809, "support": 1625}, "micro avg": {"f1-score": 0.6246105919003114, "precision": 0.8504772004241782, "recall": 0.49353846153846154, "support": 1625}, "weighted avg": {"f1-score": 0.5422536652235648, "precision": 0.7534662438625205, "recall": 0.49353846153846154, "support": 1625}, "\u2205": {"f1-score": 0.8563899868247694, "precision": 0.8463541666666666, "recall": 0.8666666666666667, "support": 750}, "\u23ce": {"f1-score": 0.34615384615384615, "precision": 0.574468085106383, "recall": 0.24770642201834864, "support": 109}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 35}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 40}, "\u2423": {"f1-score": 0.3468468468468468, "precision": 1.0, "recall": 0.2098092643051771, "support": 367}},
  "ppcr": 0.5803076923076923
}
```
</details>
