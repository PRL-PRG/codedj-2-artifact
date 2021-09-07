# Test report for javascript / file:///tmp/top-repos-quality-repos-31kju0mo/stoop.git HEAD 99b093ba96c1daa70c261ae929b13a7cef84ef2a

### Classification report

PPCR: 0.761

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.776| 0.988| 0.901| 0.869| 0.834| 886| 971| 0.912 |
| `␣` | 0.896| 0.565| 0.418| 0.693| 0.570| 336| 455| 0.738 |
| `'` | 0.977| 0.878| 0.512| 0.925| 0.672| 98| 168| 0.583 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 41| 70| 0.586 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 40| 92| 0.435 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 18| 46| 0.391 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 67| 0.119 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 6| 0.000 |
| `micro avg` | 0.807| 0.807| 0.614| 0.807| 0.697| 1427| 1875| 0.761 |
| `weighted avg` | 0.760| 0.807| 0.614| 0.767| 0.630| 1427| 1875| 0.761 |
| `macro avg` | 0.331| 0.304| 0.229| 0.311| 0.259| 1427| 1875| 0.761 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| "| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|85 |875 |11 |0 |0 |0 |0 |0 |0 |
|119 |144 |190 |2 |0 |0 |0 |0 |0 |
|70 |12 |0 |86 |0 |0 |0 |0 |0 |
|52 |36 |4 |0 |0 |0 |0 |0 |0 |
|29 |36 |5 |0 |0 |0 |0 |0 |0 |
|6 |0 |0 |0 |0 |0 |0 |0 |0 |
|59 |8 |0 |0 |0 |0 |0 |0 |0 |
|28 |16 |2 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.9247311827956989, "precision": 0.9772727272727273, "recall": 0.8775510204081632, "support": 98}, "macro avg": {"f1-score": 0.3109388837168717, "precision": 0.3312370822368772, "recall": 0.30382648262465256, "support": 1427}, "micro avg": {"f1-score": 0.8065872459705676, "precision": 0.8065872459705676, "recall": 0.8065872459705676, "support": 1427}, "weighted avg": {"f1-score": 0.7665450416456251, "precision": 0.7601913114941761, "recall": 0.8065872459705676, "support": 1427}, "\u2205": {"f1-score": 0.8693492300049678, "precision": 0.7763975155279503, "recall": 0.9875846501128668, "support": 886}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 40}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 18}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 41}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u2423": {"f1-score": 0.6934306569343066, "precision": 0.8962264150943396, "recall": 0.5654761904761905, "support": 336}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u0027": {"f1-score": 0.6718749999999999, "precision": 0.9772727272727273, "recall": 0.5119047619047619, "support": 168}, "macro avg": {"f1-score": 0.25946473539177745, "precision": 0.3312370822368772, "recall": 0.22882750402704058, "support": 1875}, "micro avg": {"f1-score": 0.697153240460327, "precision": 0.8065872459705676, "recall": 0.6138666666666667, "support": 1875}, "weighted avg": {"f1-score": 0.6304178272160393, "precision": 0.7071183064679374, "recall": 0.6138666666666667, "support": 1875}, "\u2205": {"f1-score": 0.8341277407054338, "precision": 0.7763975155279503, "recall": 0.9011328527291452, "support": 971}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 92}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 46}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 70}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 67}, "\u2423": {"f1-score": 0.5697151424287857, "precision": 0.8962264150943396, "recall": 0.4175824175824176, "support": 455}},
  "ppcr": 0.7610666666666667
}
```
</details>
