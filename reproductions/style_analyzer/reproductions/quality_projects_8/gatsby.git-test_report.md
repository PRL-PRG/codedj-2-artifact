# Test report for javascript / file:///tmp/top-repos-quality-repos-y3yu8cwh/gatsby.git HEAD fd5869bd9bada740b6ed01f855e389cf953130cd

### Classification report

PPCR: 0.726

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.912| 0.996| 0.923| 0.952| 0.917| 698| 753| 0.927 |
| `"` | 0.966| 1.000| 0.817| 0.983| 0.885| 85| 104| 0.817 |
| `␣` | 0.783| 0.327| 0.067| 0.462| 0.124| 55| 267| 0.206 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 15| 19| 0.789 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 13| 42| 0.310 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 17| 0.412 |
| `weighted avg` | 0.873| 0.914| 0.664| 0.886| 0.679| 873| 1202| 0.726 |
| `micro avg` | 0.914| 0.914| 0.664| 0.914| 0.769| 873| 1202| 0.726 |
| `macro avg` | 0.443| 0.387| 0.301| 0.399| 0.321| 873| 1202| 0.726 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|55 |695 |3 |0 |0 |0 |0 |
|212 |34 |18 |3 |0 |0 |0 |
|19 |0 |0 |85 |0 |0 |0 |
|29 |11 |2 |0 |0 |0 |0 |
|10 |7 |0 |0 |0 |0 |0 |
|4 |15 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9826589595375723, "precision": 0.9659090909090909, "recall": 1.0, "support": 85}, "macro avg": {"f1-score": 0.399375369266097, "precision": 0.4434318795624855, "recall": 0.3871624555005644, "support": 873}, "micro avg": {"f1-score": 0.9140893470790378, "precision": 0.9140893470790378, "recall": 0.9140893470790378, "support": 873}, "weighted avg": {"f1-score": 0.885962054433736, "precision": 0.8725922652646837, "recall": 0.9140893470790378, "support": 873}, "\u2205": {"f1-score": 0.952054794520548, "precision": 0.9120734908136483, "recall": 0.995702005730659, "support": 698}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 13}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 15}, "\u2423": {"f1-score": 0.46153846153846156, "precision": 0.782608695652174, "recall": 0.32727272727272727, "support": 55}},
  "cl_report_full": {"\"": {"f1-score": 0.8854166666666666, "precision": 0.9659090909090909, "recall": 0.8173076923076923, "support": 104}, "macro avg": {"f1-score": 0.32117439114601115, "precision": 0.4434318795624855, "recall": 0.30128303170684206, "support": 1202}, "micro avg": {"f1-score": 0.769156626506024, "precision": 0.9140893470790378, "recall": 0.6638935108153078, "support": 1202}, "weighted avg": {"f1-score": 0.6789512878937214, "precision": 0.828787359214936, "recall": 0.6638935108153078, "support": 1202}, "\u2205": {"f1-score": 0.9174917491749174, "precision": 0.9120734908136483, "recall": 0.9229747675962815, "support": 753}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 42}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 17}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 19}, "\u2423": {"f1-score": 0.12413793103448274, "precision": 0.782608695652174, "recall": 0.06741573033707865, "support": 267}},
  "ppcr": 0.7262895174708819
}
```
</details>
