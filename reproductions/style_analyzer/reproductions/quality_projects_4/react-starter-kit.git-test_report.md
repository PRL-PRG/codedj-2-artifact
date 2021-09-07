# Test report for javascript / file:///tmp/top-repos-quality-repos-jq9lowhm/react-starter-kit.git HEAD b821a03895ff6901722ed6f4be100699cc72a674

### Classification report

PPCR: 0.640

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.897| 0.994| 0.863| 0.943| 0.880| 649| 747| 0.869 |
| `␣` | 0.873| 0.681| 0.309| 0.765| 0.457| 182| 401| 0.454 |
| `'` | 0.994| 1.000| 0.795| 0.997| 0.883| 159| 200| 0.795 |
| `⏎` | 0.974| 0.881| 0.325| 0.925| 0.487| 42| 114| 0.368 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 12| 61| 0.197 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 68| 0.147 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 61| 0.066 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 2| 0.500 |
| `macro avg` | 0.467| 0.445| 0.287| 0.454| 0.338| 1059| 1654| 0.640 |
| `weighted avg` | 0.888| 0.911| 0.583| 0.896| 0.649| 1059| 1654| 0.640 |
| `micro avg` | 0.911| 0.911| 0.583| 0.911| 0.711| 1059| 1654| 0.640 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|98 |645 |4 |0 |0 |0 |0 |0 |0 |
|219 |58 |124 |0 |0 |0 |0 |0 |0 |
|41 |0 |0 |159 |0 |0 |0 |0 |0 |
|72 |1 |4 |0 |37 |0 |0 |0 |0 |
|58 |5 |5 |0 |0 |0 |0 |0 |0 |
|49 |6 |5 |0 |1 |0 |0 |0 |0 |
|57 |4 |0 |0 |0 |0 |0 |0 |0 |
|1 |0 |0 |1 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u0027": {"f1-score": 0.9968652037617556, "precision": 0.99375, "recall": 1.0, "support": 159}, "macro avg": {"f1-score": 0.4537849698334423, "precision": 0.4672191154899163, "recall": 0.4445134667592295, "support": 1059}, "micro avg": {"f1-score": 0.9112370160528801, "precision": 0.9112370160528801, "recall": 0.9112370160528801, "support": 1059}, "weighted avg": {"f1-score": 0.8958034215377859, "precision": 0.8876629036192287, "recall": 0.9112370160528801, "support": 1059}, "\u2205": {"f1-score": 0.9429824561403507, "precision": 0.8970792767732962, "recall": 0.9938366718027735, "support": 649}, "\u23ce": {"f1-score": 0.925, "precision": 0.9736842105263158, "recall": 0.8809523809523809, "support": 42}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u2423": {"f1-score": 0.7654320987654322, "precision": 0.8732394366197183, "recall": 0.6813186813186813, "support": 182}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u0027": {"f1-score": 0.8833333333333334, "precision": 0.99375, "recall": 0.795, "support": 200}, "macro avg": {"f1-score": 0.3383553479528417, "precision": 0.4672191154899163, "recall": 0.2865302689297682, "support": 1654}, "micro avg": {"f1-score": 0.7113896056026539, "precision": 0.9112370160528801, "recall": 0.5834340991535671, "support": 1654}, "weighted avg": {"f1-score": 0.6485074913618524, "precision": 0.8041337568525753, "recall": 0.5834340991535671, "support": 1654}, "\u2205": {"f1-score": 0.8799454297407914, "precision": 0.8970792767732962, "recall": 0.8634538152610441, "support": 747}, "\u23ce": {"f1-score": 0.4868421052631579, "precision": 0.9736842105263158, "recall": 0.32456140350877194, "support": 114}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 61}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 68}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 61}, "\u2423": {"f1-score": 0.4567219152854512, "precision": 0.8732394366197183, "recall": 0.3092269326683292, "support": 401}},
  "ppcr": 0.6402660217654171
}
```
</details>
