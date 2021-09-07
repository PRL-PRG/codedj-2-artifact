# Test report for javascript / file:///tmp/top-repos-quality-repos-rjx2w1ll/arbre-de-lespoir.git HEAD 9d2569e5c368888bb727c9c88b2186ed34fc4300

### Classification report

PPCR: 0.184

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `␣` | 0.627| 0.913| 0.278| 0.743| 0.385| 46| 151| 0.305 |
| `∅` | 0.872| 0.971| 0.099| 0.919| 0.178| 35| 342| 0.102 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 16| 41| 0.390 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 38| 0.237 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 4| 0.000 |
| `micro avg` | 0.717| 0.717| 0.132| 0.717| 0.223| 106| 576| 0.184 |
| `weighted avg` | 0.560| 0.717| 0.132| 0.626| 0.207| 106| 576| 0.184 |
| `macro avg` | 0.300| 0.377| 0.076| 0.332| 0.113| 106| 576| 0.184 |

### Confusion matrix

|refusal|  ∅| ␣| "| '| ⏎| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|307 |34 |1 |0 |0 |0 |
|105 |4 |42 |0 |0 |0 |
|4 |0 |0 |0 |0 |0 |
|29 |1 |8 |0 |0 |0 |
|25 |0 |16 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "macro avg": {"f1-score": 0.3324563501554652, "precision": 0.2997321086873326, "recall": 0.3768944099378882, "support": 106}, "micro avg": {"f1-score": 0.7169811320754716, "precision": 0.7169811320754716, "recall": 0.7169811320754716, "support": 106}, "weighted avg": {"f1-score": 0.6260080417702726, "precision": 0.5598928434749331, "recall": 0.7169811320754716, "support": 106}, "\u2205": {"f1-score": 0.9189189189189189, "precision": 0.8717948717948718, "recall": 0.9714285714285714, "support": 35}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u2423": {"f1-score": 0.7433628318584071, "precision": 0.6268656716417911, "recall": 0.9130434782608695, "support": 46}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 38}, "macro avg": {"f1-score": 0.11275975824122901, "precision": 0.2997321086873326, "recall": 0.0755121800085202, "support": 576}, "micro avg": {"f1-score": 0.22287390029325516, "precision": 0.7169811320754716, "recall": 0.13194444444444445, "support": 576}, "weighted avg": {"f1-score": 0.2069841255508199, "precision": 0.681962782242633, "recall": 0.13194444444444445, "support": 576}, "\u2205": {"f1-score": 0.1784776902887139, "precision": 0.8717948717948718, "recall": 0.09941520467836257, "support": 342}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 41}, "\u2423": {"f1-score": 0.3853211009174312, "precision": 0.6268656716417911, "recall": 0.2781456953642384, "support": 151}},
  "ppcr": 0.1840277777777778
}
```
</details>
