# Test report for javascript / file:///tmp/top-repos-quality-repos-7off3d1q/sudoslider.git HEAD 36461b800aeeda39f780d60a4e3a4203edbebe06

### Classification report

PPCR: 0.957

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.952| 0.956| 0.956| 0.954| 0.954| 270| 270| 1.000 |
| `␣` | 0.931| 0.936| 0.907| 0.933| 0.919| 187| 193| 0.969 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.929| 1.000| 1.000| 0.963| 0.963| 26| 26| 1.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.929| 1.000| 1.000| 0.963| 0.963| 26| 26| 1.000 |
| `⏎` | 1.000| 0.783| 0.439| 0.878| 0.610| 23| 41| 0.561 |
| `"` | 1.000| 0.500| 0.500| 0.667| 0.667| 2| 2| 1.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `micro avg` | 0.944| 0.944| 0.903| 0.944| 0.923| 534| 558| 0.957 |
| `weighted avg` | 0.945| 0.944| 0.903| 0.943| 0.916| 534| 558| 0.957 |
| `macro avg` | 0.718| 0.647| 0.600| 0.670| 0.634| 534| 558| 0.957 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| '| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|0 |258 |12 |0 |0 |0 |0 |
|6 |8 |175 |0 |0 |2 |2 |
|18 |4 |1 |18 |0 |0 |0 |
|0 |1 |0 |0 |1 |0 |0 |
|0 |0 |0 |0 |0 |26 |0 |
|0 |0 |0 |0 |0 |0 |26 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 2}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.6697204981908107, "precision": 0.717502930158481, "recall": 0.6467491410266346, "support": 534}, "micro avg": {"f1-score": 0.9438202247191011, "precision": 0.9438202247191011, "recall": 0.9438202247191011, "support": 534}, "weighted avg": {"f1-score": 0.9431815881131158, "precision": 0.9445745949467965, "recall": 0.9438202247191011, "support": 534}, "\u2205": {"f1-score": 0.9537892791127541, "precision": 0.9520295202952029, "recall": 0.9555555555555556, "support": 270}, "\u23ce": {"f1-score": 0.878048780487805, "precision": 1.0, "recall": 0.782608695652174, "support": 23}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.962962962962963, "precision": 0.9285714285714286, "recall": 1.0, "support": 26}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.962962962962963, "precision": 0.9285714285714286, "recall": 1.0, "support": 26}, "\u2423": {"f1-score": 0.9333333333333333, "precision": 0.9308510638297872, "recall": 0.9358288770053476, "support": 187}},
  "cl_report_full": {"\"": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 2}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.6343983167293057, "precision": 0.717502930158481, "recall": 0.6001644621368494, "support": 558}, "micro avg": {"f1-score": 0.9230769230769231, "precision": 0.9438202247191011, "recall": 0.9032258064516129, "support": 558}, "weighted avg": {"f1-score": 0.9162079746360261, "precision": 0.9462149463881149, "recall": 0.9032258064516129, "support": 558}, "\u2205": {"f1-score": 0.9537892791127541, "precision": 0.9520295202952029, "recall": 0.9555555555555556, "support": 270}, "\u23ce": {"f1-score": 0.6101694915254238, "precision": 1.0, "recall": 0.43902439024390244, "support": 41}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.962962962962963, "precision": 0.9285714285714286, "recall": 1.0, "support": 26}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.962962962962963, "precision": 0.9285714285714286, "recall": 1.0, "support": 26}, "\u2423": {"f1-score": 0.9186351706036746, "precision": 0.9308510638297872, "recall": 0.9067357512953368, "support": 193}},
  "ppcr": 0.956989247311828
}
```
</details>
