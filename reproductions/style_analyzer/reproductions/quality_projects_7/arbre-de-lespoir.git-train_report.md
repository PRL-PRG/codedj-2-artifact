# Train report for javascript / file:///tmp/top-repos-quality-repos-rjx2w1ll/arbre-de-lespoir.git HEAD 9d2569e5c368888bb727c9c88b2186ed34fc4300

### Classification report

PPCR: 0.490

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.979| 1.000| 0.750| 0.990| 0.849| 1745| 2328| 0.750 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 37| 719| 0.051 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 234| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 252| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 107| 0.000 |
| `micro avg` | 0.979| 0.979| 0.479| 0.979| 0.644| 1782| 3640| 0.490 |
| `weighted avg` | 0.959| 0.979| 0.479| 0.969| 0.543| 1782| 3640| 0.490 |
| `macro avg` | 0.196| 0.200| 0.150| 0.198| 0.170| 1782| 3640| 0.490 |

### Confusion matrix

|refusal|  ∅| ␣| "| '| ⏎| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|583 |1745 |0 |0 |0 |0 |
|682 |37 |0 |0 |0 |0 |
|234 |0 |0 |0 |0 |0 |
|252 |0 |0 |0 |0 |0 |
|107 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/containers/navbar/navigation.js | 18 |
| src/serviceWorker.js | 4 |
| src/views/home/index.js | 4 |
| src/containers/navbar/item/index.js | 3 |
| src/views/actions/index.js | 3 |
| src/views/presentation/index.js | 3 |
| src/views/partenaires/index.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.1979018996314148, "precision": 0.19584736251402918, "recall": 0.2, "support": 1782}, "micro avg": {"f1-score": 0.9792368125701459, "precision": 0.9792368125701459, "recall": 0.9792368125701459, "support": 1782}, "weighted avg": {"f1-score": 0.9689641269832178, "precision": 0.958904735092539, "recall": 0.9792368125701459, "support": 1782}, "\u2205": {"f1-score": 0.989509498157074, "precision": 0.9792368125701459, "recall": 1.0, "support": 1745}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 37}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 234}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 252}, "macro avg": {"f1-score": 0.16982968369829682, "precision": 0.19584736251402918, "recall": 0.14991408934707903, "support": 3640}, "micro avg": {"f1-score": 0.6436739210623387, "precision": 0.9792368125701459, "recall": 0.4793956043956044, "support": 3640}, "weighted avg": {"f1-score": 0.5430817357824657, "precision": 0.6262811262811263, "recall": 0.4793956043956044, "support": 3640}, "\u2205": {"f1-score": 0.8491484184914841, "precision": 0.9792368125701459, "recall": 0.7495704467353952, "support": 2328}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 107}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 719}},
  "ppcr": 0.48956043956043954
}
```
</details>
