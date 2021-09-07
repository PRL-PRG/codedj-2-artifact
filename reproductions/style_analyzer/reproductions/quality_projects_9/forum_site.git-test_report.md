# Test report for javascript / file:///tmp/top-repos-quality-repos-fng7o6pr/forum_site.git HEAD becfef61e3aad89f1c41079a106f03633e484960

### Classification report

PPCR: 0.686

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.793| 1.000| 0.924| 0.885| 0.854| 606| 656| 0.924 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 75| 237| 0.316 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 32| 38| 0.842 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 22| 45| 0.489 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 16| 43| 0.372 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 20| 0.450 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 4| 1.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 28| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 42| 0.000 |
| `weighted avg` | 0.629| 0.793| 0.544| 0.702| 0.503| 764| 1113| 0.686 |
| `macro avg` | 0.088| 0.111| 0.103| 0.098| 0.095| 764| 1113| 0.686 |
| `micro avg` | 0.793| 0.793| 0.544| 0.793| 0.646| 764| 1113| 0.686 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| ⏎␣⁺␣⁺| ⏎⏎| ⏎␣⁻␣⁻| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|50 |606 |0 |0 |0 |0 |0 |0 |0 |0 |
|162 |75 |0 |0 |0 |0 |0 |0 |0 |0 |
|27 |16 |0 |0 |0 |0 |0 |0 |0 |0 |
|42 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|28 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|23 |22 |0 |0 |0 |0 |0 |0 |0 |0 |
|11 |9 |0 |0 |0 |0 |0 |0 |0 |0 |
|6 |32 |0 |0 |0 |0 |0 |0 |0 |0 |
|0 |4 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.09829683698296837, "precision": 0.08813263525305409, "recall": 0.1111111111111111, "support": 764}, "micro avg": {"f1-score": 0.7931937172774869, "precision": 0.7931937172774869, "recall": 0.7931937172774869, "support": 764}, "weighted avg": {"f1-score": 0.7017159017082586, "precision": 0.6291562731284779, "recall": 0.7931937172774869, "support": 764}, "\u2205": {"f1-score": 0.8846715328467154, "precision": 0.7931937172774869, "recall": 1.0, "support": 606}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 32}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 75}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 28}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 42}, "macro avg": {"f1-score": 0.09483568075117371, "precision": 0.08813263525305409, "recall": 0.10264227642276423, "support": 1113}, "micro avg": {"f1-score": 0.645711241342568, "precision": 0.7931937172774869, "recall": 0.5444743935309974, "support": 1113}, "weighted avg": {"f1-score": 0.5030636650089215, "precision": 0.46750680910514947, "recall": 0.5444743935309974, "support": 1113}, "\u2205": {"f1-score": 0.8535211267605635, "precision": 0.7931937172774869, "recall": 0.9237804878048781, "support": 656}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 43}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 20}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 45}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 38}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 237}},
  "ppcr": 0.6864330637915543
}
```
</details>
