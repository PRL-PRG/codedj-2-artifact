# Test report for javascript / file:///tmp/top-repos-quality-repos-k0djppcj/pomodo.git HEAD 9d78c3c8efecbebe67841e0d82ce82efbbfad201

### Classification report

PPCR: 0.622

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.950| 1.000| 0.905| 0.974| 0.927| 19| 21| 0.905 |
| `␣` | 1.000| 0.750| 0.333| 0.857| 0.500| 4| 9| 0.444 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 4| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 2| 0.000 |
| `macro avg` | 0.279| 0.250| 0.177| 0.262| 0.204| 23| 37| 0.622 |
| `weighted avg` | 0.959| 0.957| 0.595| 0.954| 0.648| 23| 37| 0.622 |
| `micro avg` | 0.957| 0.957| 0.595| 0.957| 0.733| 23| 37| 0.622 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|2 |19 |0 |0 |0 |0 |
|5 |1 |3 |0 |0 |0 |
|4 |0 |0 |0 |0 |0 |
|1 |0 |0 |0 |0 |0 |
|2 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.2616431187859759, "precision": 0.2785714285714286, "recall": 0.25, "support": 23}, "micro avg": {"f1-score": 0.9565217391304348, "precision": 0.9565217391304348, "recall": 0.9565217391304348, "support": 23}, "weighted avg": {"f1-score": 0.9539735626692147, "precision": 0.9586956521739131, "recall": 0.9565217391304348, "support": 23}, "\u2205": {"f1-score": 0.9743589743589743, "precision": 0.95, "recall": 1.0, "support": 19}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.8571428571428571, "precision": 1.0, "recall": 0.75, "support": 4}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "macro avg": {"f1-score": 0.2038327526132404, "precision": 0.2785714285714286, "recall": 0.17687074829931973, "support": 37}, "micro avg": {"f1-score": 0.7333333333333334, "precision": 0.9565217391304348, "recall": 0.5945945945945946, "support": 37}, "weighted avg": {"f1-score": 0.6476598549769281, "precision": 0.7824324324324324, "recall": 0.5945945945945946, "support": 37}, "\u2205": {"f1-score": 0.9268292682926829, "precision": 0.95, "recall": 0.9047619047619048, "support": 21}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u2423": {"f1-score": 0.5, "precision": 1.0, "recall": 0.3333333333333333, "support": 9}},
  "ppcr": 0.6216216216216216
}
```
</details>
