# Train report for javascript / file:///tmp/top-repos-quality-repos-8_pugp2s/matt-peet-illustration.git HEAD aab58c9d78342452aac4819e0b72474ce8b45e0a

### Classification report

PPCR: 0.156

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.990| 1.000| 0.270| 0.995| 0.425| 761| 2816| 0.270 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 1189| 0.006 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 133| 0.008 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 222| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 205| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 231| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 130| 0.000 |
| `macro avg` | 0.141| 0.143| 0.039| 0.142| 0.061| 769| 4926| 0.156 |
| `micro avg` | 0.990| 0.990| 0.154| 0.990| 0.267| 769| 4926| 0.156 |
| `weighted avg` | 0.979| 0.990| 0.154| 0.984| 0.243| 769| 4926| 0.156 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| '| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|2055 |761 |0 |0 |0 |0 |0 |0 |
|1182 |7 |0 |0 |0 |0 |0 |0 |
|222 |0 |0 |0 |0 |0 |0 |0 |
|205 |0 |0 |0 |0 |0 |0 |0 |
|231 |0 |0 |0 |0 |0 |0 |0 |
|132 |1 |0 |0 |0 |0 |0 |0 |
|130 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/templates/project-page.js | 6 |
| src/components/Navbar.js | 1 |
| src/components/PreviewCompatibleImage.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.14211017740429505, "precision": 0.14137098272338844, "recall": 0.14285714285714285, "support": 769}, "micro avg": {"f1-score": 0.9895968790637191, "precision": 0.9895968790637191, "recall": 0.9895968790637191, "support": 769}, "weighted avg": {"f1-score": 0.984422516297373, "precision": 0.979301983052653, "recall": 0.9895968790637191, "support": 769}, "\u2205": {"f1-score": 0.9947712418300654, "precision": 0.9895968790637191, "recall": 1.0, "support": 761}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 222}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 231}, "macro avg": {"f1-score": 0.060649531779238895, "precision": 0.14137098272338844, "recall": 0.03860592532467533, "support": 4926}, "micro avg": {"f1-score": 0.2672519754170325, "precision": 0.9895968790637191, "recall": 0.1544863987007714, "support": 4926}, "weighted avg": {"f1-score": 0.24269662412349924, "precision": 0.5657135224205101, "recall": 0.1544863987007714, "support": 4926}, "\u2205": {"f1-score": 0.4245467224546723, "precision": 0.9895968790637191, "recall": 0.2702414772727273, "support": 2816}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 205}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 130}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 133}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1189}},
  "ppcr": 0.15611043442955744
}
```
</details>
