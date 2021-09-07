# Test report for javascript / file:///tmp/top-repos-quality-repos-mvvat_io/nativosdk-reactnative-sampleapp.git HEAD 8f5c27718267b94d0a4ff69eb01dcf06c189fd85

### Classification report

PPCR: 0.451

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.972| 1.000| 0.767| 0.986| 0.857| 346| 451| 0.767 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 201| 0.050 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 12| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 68| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 57| 0.000 |
| `weighted avg` | 0.945| 0.972| 0.439| 0.958| 0.490| 356| 789| 0.451 |
| `micro avg` | 0.972| 0.972| 0.439| 0.972| 0.604| 356| 789| 0.451 |
| `macro avg` | 0.194| 0.200| 0.153| 0.197| 0.171| 356| 789| 0.451 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|105 |346 |0 |0 |0 |0 |
|191 |10 |0 |0 |0 |0 |
|68 |0 |0 |0 |0 |0 |
|57 |0 |0 |0 |0 |0 |
|12 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.19715099715099715, "precision": 0.1943820224719101, "recall": 0.2, "support": 356}, "micro avg": {"f1-score": 0.9719101123595506, "precision": 0.9719101123595506, "recall": 0.9719101123595506, "support": 356}, "weighted avg": {"f1-score": 0.9580652389641153, "precision": 0.9446092665067544, "recall": 0.9719101123595506, "support": 356}, "\u2205": {"f1-score": 0.9857549857549858, "precision": 0.9719101123595506, "recall": 1.0, "support": 346}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 68}, "macro avg": {"f1-score": 0.1714993804213135, "precision": 0.1943820224719101, "recall": 0.15343680709534369, "support": 789}, "micro avg": {"f1-score": 0.6043668122270743, "precision": 0.9719101123595506, "recall": 0.4385297845373891, "support": 789}, "weighted avg": {"f1-score": 0.49015348903683387, "precision": 0.5555531820965238, "recall": 0.4385297845373891, "support": 789}, "\u2205": {"f1-score": 0.8574969021065675, "precision": 0.9719101123595506, "recall": 0.7671840354767184, "support": 451}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 57}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 201}},
  "ppcr": 0.4512040557667934
}
```
</details>
