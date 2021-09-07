# Test report for javascript / file:///tmp/top-repos-quality-repos-3kibs5bn/akshay23sept.github.io.git HEAD a5dedc1a3efe295a5c8b01433a6d41f32c82221b

### Classification report

PPCR: 0.712

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.883| 0.996| 0.924| 0.936| 0.903| 523| 564| 0.927 |
| `⏎` | 0.960| 0.960| 0.950| 0.960| 0.955| 99| 100| 0.990 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 57| 219| 0.260 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 26| 0.385 |
| `␣⇥` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 31| 0.000 |
| `␣⇥⇥` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 28| 0.000 |
| `macro avg` | 0.307| 0.326| 0.312| 0.316| 0.310| 689| 968| 0.712 |
| `micro avg` | 0.894| 0.894| 0.636| 0.894| 0.744| 689| 968| 0.712 |
| `weighted avg` | 0.808| 0.894| 0.636| 0.849| 0.625| 689| 968| 0.712 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| ␣⇥| ␣⇥⇥| '| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|41 |521 |0 |2 |0 |0 |0 |
|162 |55 |0 |2 |0 |0 |0 |
|1 |4 |0 |95 |0 |0 |0 |
|31 |0 |0 |0 |0 |0 |0 |
|28 |0 |0 |0 |0 |0 |0 |
|16 |10 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "macro avg": {"f1-score": 0.31596740087306124, "precision": 0.30710780117559777, "recall": 0.32596197796962617, "support": 689}, "micro avg": {"f1-score": 0.8940493468795355, "precision": 0.8940493468795355, "recall": 0.8940493468795355, "support": 689}, "weighted avg": {"f1-score": 0.8485297780420599, "precision": 0.8081793805810435, "recall": 0.8940493468795355, "support": 689}, "\u2205": {"f1-score": 0.9362084456424079, "precision": 0.8830508474576271, "recall": 0.9961759082217974, "support": 523}, "\u23ce": {"f1-score": 0.9595959595959596, "precision": 0.9595959595959596, "recall": 0.9595959595959596, "support": 99}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 57}, "\u2423\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423\u21e5\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 26}, "macro avg": {"f1-score": 0.3096200238628149, "precision": 0.30710780117559777, "recall": 0.31229314420803783, "support": 968}, "micro avg": {"f1-score": 0.7435123717561858, "precision": 0.8940493468795355, "recall": 0.6363636363636364, "support": 968}, "weighted avg": {"f1-score": 0.6247304600980178, "precision": 0.6136366466174563, "recall": 0.6363636363636364, "support": 968}, "\u2205": {"f1-score": 0.9029462738301559, "precision": 0.8830508474576271, "recall": 0.9237588652482269, "support": 564}, "\u23ce": {"f1-score": 0.9547738693467336, "precision": 0.9595959595959596, "recall": 0.95, "support": 100}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 219}, "\u2423\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 31}, "\u2423\u21e5\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 28}},
  "ppcr": 0.7117768595041323
}
```
</details>
