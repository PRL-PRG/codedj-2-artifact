# Train report for javascript / file:///tmp/top-repos-quality-repos-d5axd4fp/dylans-cms.git HEAD d71a957fde2255a444ee13e1adabd4b309fcbffe

### Classification report

PPCR: 0.470

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.947| 1.000| 0.746| 0.973| 0.835| 5016| 6722| 0.746 |
| `'` | 1.000| 1.000| 0.742| 1.000| 0.852| 736| 992| 0.742 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 114| 2917| 0.039 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 92| 411| 0.224 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 45| 770| 0.058 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 29| 433| 0.067 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 388| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 213| 0.000 |
| `weighted avg` | 0.910| 0.954| 0.448| 0.931| 0.503| 6032| 12846| 0.470 |
| `micro avg` | 0.954| 0.954| 0.448| 0.954| 0.609| 6032| 12846| 0.470 |
| `macro avg` | 0.243| 0.250| 0.186| 0.247| 0.211| 6032| 12846| 0.470 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| "| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1706 |5016 |0 |0 |0 |0 |0 |0 |0 |
|2803 |114 |0 |0 |0 |0 |0 |0 |0 |
|256 |0 |0 |736 |0 |0 |0 |0 |0 |
|725 |45 |0 |0 |0 |0 |0 |0 |0 |
|404 |29 |0 |0 |0 |0 |0 |0 |0 |
|388 |0 |0 |0 |0 |0 |0 |0 |0 |
|319 |92 |0 |0 |0 |0 |0 |0 |0 |
|213 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/components/FormControlled.js | 23 |
| src/components/Image.js | 19 |
| src/templates/BlogIndex.js | 19 |
| src/components/InstagramFeed.js | 18 |
| src/components/FormSimpleAjax.js | 16 |
| src/components/Nav.js | 15 |
| src/cms/cms.js | 14 |
| src/components/Content.js | 13 |
| src/components/Observer.js | 11 |
| src/templates/ContactPage.js | 10 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 736}, "macro avg": {"f1-score": 0.24660589604344452, "precision": 0.24339123867069487, "recall": 0.25, "support": 6032}, "micro avg": {"f1-score": 0.9535809018567639, "precision": 0.9535809018567639, "recall": 0.9535809018567639, "support": 6032}, "weighted avg": {"f1-score": 0.9310015577638167, "precision": 0.9096159856395297, "recall": 0.9535809018567639, "support": 6032}, "\u2205": {"f1-score": 0.9728471683475562, "precision": 0.947129909365559, "recall": 1.0, "support": 5016}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 45}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 29}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 92}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 114}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 388}, "\u0027": {"f1-score": 0.8518518518518519, "precision": 1.0, "recall": 0.7419354838709677, "support": 992}, "macro avg": {"f1-score": 0.21082496625432223, "precision": 0.24339123867069487, "recall": 0.18601774625447495, "support": 12846}, "micro avg": {"f1-score": 0.6093865875622417, "precision": 0.9535809018567639, "recall": 0.44776584150708393, "support": 12846}, "weighted avg": {"f1-score": 0.502585417576002, "precision": 0.5728325744010032, "recall": 0.44776584150708393, "support": 12846}, "\u2205": {"f1-score": 0.8347478781827259, "precision": 0.947129909365559, "recall": 0.7462064861648319, "support": 6722}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 770}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 213}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 433}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 411}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2917}},
  "ppcr": 0.46956250973065544
}
```
</details>
