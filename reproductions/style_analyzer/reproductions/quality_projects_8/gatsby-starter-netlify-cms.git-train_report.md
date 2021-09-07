# Train report for javascript / file:///tmp/top-repos-quality-repos-o55xlgl9/gatsby-starter-netlify-cms.git HEAD 5c5addbb565f1aae3c1ea15a37b17f42eb65ba6a

### Classification report

PPCR: 0.118

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `"` | 1.000| 0.996| 0.987| 0.998| 0.993| 522| 527| 0.991 |
| `'` | 0.996| 1.000| 0.870| 0.998| 0.929| 508| 584| 0.870 |
| `∅` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 5335| 0.000 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1693| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 413| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 14| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 22| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 122| 0.000 |
| `weighted avg` | 0.998| 0.998| 0.118| 0.998| 0.122| 1030| 8710| 0.118 |
| `micro avg` | 0.998| 0.998| 0.118| 0.998| 0.211| 1030| 8710| 0.118 |
| `macro avg` | 0.250| 0.250| 0.232| 0.250| 0.240| 1030| 8710| 0.118 |

### Confusion matrix

|refusal|  ∅| ␣| '| "| ⏎| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|5335 |0 |0 |0 |0 |0 |0 |0 |0 |
|1693 |0 |0 |0 |0 |0 |0 |0 |0 |
|76 |0 |0 |508 |0 |0 |0 |0 |0 |
|5 |0 |0 |2 |520 |0 |0 |0 |0 |
|413 |0 |0 |0 |0 |0 |0 |0 |0 |
|14 |0 |0 |0 |0 |0 |0 |0 |0 |
|22 |0 |0 |0 |0 |0 |0 |0 |0 |
|122 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/components/Layout.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9980806142034548, "precision": 1.0, "recall": 0.9961685823754789, "support": 522}, "\u0027": {"f1-score": 0.9980353634577603, "precision": 0.996078431372549, "recall": 1.0, "support": 508}, "macro avg": {"f1-score": 0.2495144972076519, "precision": 0.24950980392156863, "recall": 0.24952107279693486, "support": 1030}, "micro avg": {"f1-score": 0.9980582524271845, "precision": 0.9980582524271845, "recall": 0.9980582524271845, "support": 1030}, "weighted avg": {"f1-score": 0.9980582963599473, "precision": 0.9980658671235484, "recall": 0.9980582524271845, "support": 1030}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\"": {"f1-score": 0.9933142311365807, "precision": 1.0, "recall": 0.9867172675521821, "support": 527}, "\u0027": {"f1-score": 0.9287020109689214, "precision": 0.996078431372549, "recall": 0.8698630136986302, "support": 584}, "macro avg": {"f1-score": 0.24025203026318775, "precision": 0.24950980392156863, "recall": 0.23207253515635154, "support": 8710}, "micro avg": {"f1-score": 0.211088295687885, "precision": 0.9980582524271845, "recall": 0.11802525832376579, "support": 8710}, "weighted avg": {"f1-score": 0.12236952631628335, "precision": 0.12729159631705725, "recall": 0.11802525832376579, "support": 8710}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5335}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 413}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 122}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1693}},
  "ppcr": 0.1182548794489093
}
```
</details>
