# Train report for javascript / file:///tmp/top-repos-quality-repos-8bmdcf1m/rona-v2.git HEAD a56dca26de054977783196add322b6b8c2f622a8

### Classification report

PPCR: 0.066

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.991| 1.000| 0.109| 0.996| 0.197| 459| 4207| 0.109 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 289| 0.014 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1390| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 418| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 662| 0.000 |
| `micro avg` | 0.991| 0.991| 0.066| 0.991| 0.124| 463| 6966| 0.066 |
| `weighted avg` | 0.983| 0.991| 0.066| 0.987| 0.119| 463| 6966| 0.066 |
| `macro avg` | 0.198| 0.200| 0.022| 0.199| 0.039| 463| 6966| 0.066 |

### Confusion matrix

|refusal|  ∅| ␣| '| "| ⏎| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|3748 |459 |0 |0 |0 |0 |
|1390 |0 |0 |0 |0 |0 |
|418 |0 |0 |0 |0 |0 |
|662 |0 |0 |0 |0 |0 |
|285 |4 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| cloudinary.js | 3 |
| src/pages/guest.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.19913232104121475, "precision": 0.19827213822894169, "recall": 0.2, "support": 463}, "micro avg": {"f1-score": 0.9913606911447084, "precision": 0.9913606911447084, "recall": 0.9913606911447084, "support": 463}, "weighted avg": {"f1-score": 0.987059777083343, "precision": 0.9827960199469139, "recall": 0.9913606911447084, "support": 463}, "\u2205": {"f1-score": 0.9956616052060737, "precision": 0.9913606911447084, "recall": 1.0, "support": 459}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 662}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 418}, "macro avg": {"f1-score": 0.039314775160599565, "precision": 0.19827213822894169, "recall": 0.021820774898977893, "support": 6966}, "micro avg": {"f1-score": 0.12356979405034325, "precision": 0.9913606911447084, "recall": 0.06589147286821706, "support": 6966}, "weighted avg": {"f1-score": 0.11871752734757564, "precision": 0.5987158236643394, "recall": 0.06589147286821706, "support": 6966}, "\u2205": {"f1-score": 0.19657387580299782, "precision": 0.9913606911447084, "recall": 0.10910387449488947, "support": 4207}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 289}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1390}},
  "ppcr": 0.06646569049669825
}
```
</details>
