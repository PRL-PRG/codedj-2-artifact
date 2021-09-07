# Test report for javascript / file:///tmp/top-repos-quality-repos-3u2jbfdv/jpfp-redo.git HEAD 18c087278b7c1ee5ad557de45ed88f5a71fd2f19

### Classification report

PPCR: 0.754

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.697| 0.982| 0.851| 0.816| 0.766| 342| 395| 0.866 |
| `␣` | 0.851| 0.579| 0.400| 0.689| 0.544| 197| 285| 0.691 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 37| 50| 0.740 |
| `'` | 1.000| 0.912| 0.456| 0.954| 0.626| 34| 68| 0.500 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 29| 29| 1.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 31| 0.258 |
| `macro avg` | 0.425| 0.412| 0.284| 0.410| 0.323| 647| 858| 0.754 |
| `weighted avg` | 0.680| 0.743| 0.561| 0.691| 0.583| 647| 858| 0.754 |
| `micro avg` | 0.743| 0.743| 0.561| 0.743| 0.639| 647| 858| 0.754 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|53 |336 |6 |0 |0 |0 |0 |
|88 |83 |114 |0 |0 |0 |0 |
|34 |3 |0 |31 |0 |0 |0 |
|13 |24 |13 |0 |0 |0 |0 |
|23 |7 |1 |0 |0 |0 |0 |
|0 |29 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.9538461538461539, "precision": 1.0, "recall": 0.9117647058823529, "support": 34}, "macro avg": {"f1-score": 0.40970031444908983, "precision": 0.42464028405689397, "recall": 0.41215017487981925, "support": 647}, "micro avg": {"f1-score": 0.7434312210200927, "precision": 0.7434312210200927, "recall": 0.7434312210200927, "support": 647}, "weighted avg": {"f1-score": 0.6909447848320975, "precision": 0.6800674712975618, "recall": 0.7434312210200927, "support": 647}, "\u2205": {"f1-score": 0.8155339805825242, "precision": 0.6970954356846473, "recall": 0.9824561403508771, "support": 342}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 37}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 29}, "\u2423": {"f1-score": 0.688821752265861, "precision": 0.8507462686567164, "recall": 0.5786802030456852, "support": 197}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6262626262626263, "precision": 1.0, "recall": 0.45588235294117646, "support": 68}, "macro avg": {"f1-score": 0.32277732426318817, "precision": 0.42464028405689397, "recall": 0.28441921072226356, "support": 858}, "micro avg": {"f1-score": 0.639202657807309, "precision": 0.7434312210200927, "recall": 0.5606060606060606, "support": 858}, "weighted avg": {"f1-score": 0.5831440300775865, "precision": 0.6827685124272725, "recall": 0.5606060606060606, "support": 858}, "\u2205": {"f1-score": 0.7662485746864309, "precision": 0.6970954356846473, "recall": 0.850632911392405, "support": 395}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 50}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 31}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 29}, "\u2423": {"f1-score": 0.5441527446300717, "precision": 0.8507462686567164, "recall": 0.4, "support": 285}},
  "ppcr": 0.754079254079254
}
```
</details>
