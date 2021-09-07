# Test report for javascript / file:///tmp/top-repos-quality-repos-lnub4lqn/react-video-recorder.git HEAD dc441024546cfe9862e5e4a8bc0ea3a72840833e

### Classification report

PPCR: 0.248

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.966| 0.910| 0.329| 0.937| 0.491| 155| 428| 0.362 |
| `␣` | 0.745| 0.976| 0.166| 0.845| 0.272| 42| 247| 0.170 |
| `⏎` | 0.920| 0.958| 0.264| 0.939| 0.411| 24| 87| 0.276 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 34| 0.059 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 64| 0.016 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 22| 0.045 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 29| 0.034 |
| `micro avg` | 0.907| 0.907| 0.225| 0.907| 0.361| 226| 911| 0.248 |
| `weighted avg` | 0.899| 0.907| 0.225| 0.899| 0.344| 226| 911| 0.248 |
| `macro avg` | 0.376| 0.406| 0.109| 0.389| 0.168| 226| 911| 0.248 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|273 |141 |14 |0 |0 |0 |0 |0 |
|205 |0 |41 |1 |0 |0 |0 |0 |
|63 |1 |0 |23 |0 |0 |0 |0 |
|63 |1 |0 |0 |0 |0 |0 |0 |
|32 |2 |0 |0 |0 |0 |0 |0 |
|28 |1 |0 |0 |0 |0 |0 |0 |
|21 |0 |0 |1 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "macro avg": {"f1-score": 0.38871620162261566, "precision": 0.37588685287315426, "recall": 0.40631446126837834, "support": 226}, "micro avg": {"f1-score": 0.9070796460176991, "precision": 0.9070796460176991, "recall": 0.9070796460176991, "support": 226}, "weighted avg": {"f1-score": 0.8993438660527744, "precision": 0.898587928013313, "recall": 0.9070796460176991, "support": 226}, "\u2205": {"f1-score": 0.93687707641196, "precision": 0.9657534246575342, "recall": 0.9096774193548387, "support": 155}, "\u23ce": {"f1-score": 0.9387755102040817, "precision": 0.92, "recall": 0.9583333333333334, "support": 24}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.845360824742268, "precision": 0.7454545454545455, "recall": 0.9761904761904762, "support": 42}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 64}, "macro avg": {"f1-score": 0.16764666616121496, "precision": 0.37588685287315426, "recall": 0.10854271018034438, "support": 911}, "micro avg": {"f1-score": 0.3605980650835532, "precision": 0.9070796460176991, "recall": 0.22502744237102085, "support": 911}, "weighted avg": {"f1-score": 0.3436554830144787, "precision": 0.743698944545222, "recall": 0.22502744237102085, "support": 911}, "\u2205": {"f1-score": 0.4912891986062718, "precision": 0.9657534246575342, "recall": 0.3294392523364486, "support": 428}, "\u23ce": {"f1-score": 0.41071428571428575, "precision": 0.92, "recall": 0.26436781609195403, "support": 87}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 34}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 29}, "\u2423": {"f1-score": 0.271523178807947, "precision": 0.7454545454545455, "recall": 0.1659919028340081, "support": 247}},
  "ppcr": 0.24807903402854006
}
```
</details>
