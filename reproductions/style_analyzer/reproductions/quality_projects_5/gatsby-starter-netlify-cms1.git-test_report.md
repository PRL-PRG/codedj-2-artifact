# Test report for javascript / file:///tmp/top-repos-quality-repos-ocw146us/gatsby-starter-netlify-cms1.git HEAD 7c1d09728f585d13ce558891541b3f8630b24999

### Classification report

PPCR: 0.561

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.917| 1.000| 0.833| 0.957| 0.873| 55| 66| 0.833 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 12| 0.333 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 21| 0.048 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 4| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 4| 0.000 |
| `micro avg` | 0.917| 0.917| 0.514| 0.917| 0.659| 60| 107| 0.561 |
| `macro avg` | 0.183| 0.200| 0.167| 0.191| 0.175| 60| 107| 0.561 |
| `weighted avg` | 0.840| 0.917| 0.514| 0.877| 0.538| 60| 107| 0.561 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|11 |55 |0 |0 |0 |0 |
|20 |1 |0 |0 |0 |0 |
|4 |0 |0 |0 |0 |0 |
|4 |0 |0 |0 |0 |0 |
|8 |4 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.19130434782608696, "precision": 0.18333333333333332, "recall": 0.2, "support": 60}, "micro avg": {"f1-score": 0.9166666666666666, "precision": 0.9166666666666666, "recall": 0.9166666666666666, "support": 60}, "weighted avg": {"f1-score": 0.8768115942028986, "precision": 0.8402777777777778, "recall": 0.9166666666666666, "support": 60}, "\u2205": {"f1-score": 0.9565217391304348, "precision": 0.9166666666666666, "recall": 1.0, "support": 55}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "macro avg": {"f1-score": 0.1746031746031746, "precision": 0.18333333333333332, "recall": 0.16666666666666669, "support": 107}, "micro avg": {"f1-score": 0.6586826347305389, "precision": 0.9166666666666666, "recall": 0.514018691588785, "support": 107}, "weighted avg": {"f1-score": 0.5384957721406319, "precision": 0.5654205607476636, "recall": 0.514018691588785, "support": 107}, "\u2205": {"f1-score": 0.8730158730158729, "precision": 0.9166666666666666, "recall": 0.8333333333333334, "support": 66}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 21}},
  "ppcr": 0.5607476635514018
}
```
</details>
