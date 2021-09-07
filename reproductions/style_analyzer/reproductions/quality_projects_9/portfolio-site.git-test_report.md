# Test report for javascript / file:///tmp/top-repos-quality-repos-z_b870uy/portfolio-site.git HEAD 677a844282dfc0b83610c38d303b2df042334354

### Classification report

PPCR: 0.436

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.932| 0.986| 0.722| 0.959| 0.814| 363| 496| 0.732 |
| `␣` | 0.850| 0.607| 0.126| 0.708| 0.219| 56| 270| 0.207 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 27| 0.111 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 27| 0.074 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 14| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 62| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 54| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 22| 0.000 |
| `weighted avg` | 0.910| 0.925| 0.403| 0.914| 0.476| 424| 972| 0.436 |
| `macro avg` | 0.223| 0.199| 0.106| 0.208| 0.129| 424| 972| 0.436 |
| `micro avg` | 0.925| 0.925| 0.403| 0.925| 0.562| 424| 972| 0.436 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|133 |358 |5 |0 |0 |0 |0 |0 |0 |
|214 |22 |34 |0 |0 |0 |0 |0 |0 |
|62 |0 |0 |0 |0 |0 |0 |0 |0 |
|54 |0 |0 |0 |0 |0 |0 |0 |0 |
|14 |0 |0 |0 |0 |0 |0 |0 |0 |
|25 |2 |0 |0 |0 |0 |0 |0 |0 |
|24 |2 |1 |0 |0 |0 |0 |0 |0 |
|22 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.20835425033467203, "precision": 0.22278645833333333, "recall": 0.1991710940574577, "support": 424}, "micro avg": {"f1-score": 0.9245283018867925, "precision": 0.9245283018867925, "recall": 0.9245283018867925, "support": 424}, "weighted avg": {"f1-score": 0.9141566265060243, "precision": 0.9104289504716981, "recall": 0.9245283018867925, "support": 424}, "\u2205": {"f1-score": 0.9585006693440429, "precision": 0.9322916666666666, "recall": 0.9862258953168044, "support": 363}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u2423": {"f1-score": 0.7083333333333333, "precision": 0.85, "recall": 0.6071428571428571, "support": 56}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 62}, "macro avg": {"f1-score": 0.12912390029325513, "precision": 0.22278645833333333, "recall": 0.10596251493428913, "support": 972}, "micro avg": {"f1-score": 0.5616045845272206, "precision": 0.9245283018867925, "recall": 0.40329218106995884, "support": 972}, "weighted avg": {"f1-score": 0.4761208259416145, "precision": 0.7118484224965707, "recall": 0.40329218106995884, "support": 972}, "\u2205": {"f1-score": 0.8136363636363636, "precision": 0.9322916666666666, "recall": 0.7217741935483871, "support": 496}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 54}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 27}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 27}, "\u2423": {"f1-score": 0.21935483870967742, "precision": 0.85, "recall": 0.1259259259259259, "support": 270}},
  "ppcr": 0.43621399176954734
}
```
</details>
