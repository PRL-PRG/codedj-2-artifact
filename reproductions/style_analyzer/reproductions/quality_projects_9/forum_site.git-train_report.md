# Train report for javascript / file:///tmp/top-repos-quality-repos-fng7o6pr/forum_site.git HEAD becfef61e3aad89f1c41079a106f03633e484960

### Classification report

PPCR: 0.418

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.968| 1.000| 0.667| 0.984| 0.790| 3142| 4710| 0.667 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 47| 1610| 0.029 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 19| 141| 0.135 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 18| 172| 0.105 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 16| 92| 0.174 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 339| 0.009 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 292| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 282| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 124| 0.000 |
| `weighted avg` | 0.938| 0.968| 0.405| 0.953| 0.479| 3245| 7762| 0.418 |
| `macro avg` | 0.108| 0.111| 0.074| 0.109| 0.088| 3245| 7762| 0.418 |
| `micro avg` | 0.968| 0.968| 0.405| 0.968| 0.571| 3245| 7762| 0.418 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| ⏎␣⁺␣⁺| ⏎⏎| ⏎␣⁻␣⁻| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1568 |3142 |0 |0 |0 |0 |0 |0 |0 |0 |
|1563 |47 |0 |0 |0 |0 |0 |0 |0 |0 |
|336 |3 |0 |0 |0 |0 |0 |0 |0 |0 |
|292 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|282 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|154 |18 |0 |0 |0 |0 |0 |0 |0 |0 |
|124 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|122 |19 |0 |0 |0 |0 |0 |0 |0 |0 |
|76 |16 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| client/src/providers/PostDataProvider.js | 18 |
| client/src/Components/PostCard.js | 16 |
| client/src/App.js | 9 |
| client/src/Components/Post.backup.js | 9 |
| routes/postRouter.js | 8 |
| client/src/pages/Topics.js | 7 |
| routes/commentRouter.js | 6 |
| routes/topicRouter.js | 6 |
| client/src/pages/Topic.js | 5 |
| client/src/Components/Post.js | 5 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.10931927700363586, "precision": 0.10758431775380928, "recall": 0.1111111111111111, "support": 3245}, "micro avg": {"f1-score": 0.9682588597842835, "precision": 0.9682588597842835, "recall": 0.9682588597842835, "support": 3245}, "weighted avg": {"f1-score": 0.9526442265358444, "precision": 0.9375252195507608, "recall": 0.9682588597842835, "support": 3245}, "\u2205": {"f1-score": 0.9838734930327228, "precision": 0.9682588597842835, "recall": 1.0, "support": 3142}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 18}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 19}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 47}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 282}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 292}, "macro avg": {"f1-score": 0.0877714924226552, "precision": 0.10758431775380928, "recall": 0.07412125501297476, "support": 7762}, "micro avg": {"f1-score": 0.570909421277369, "precision": 0.9682588597842835, "recall": 0.40479257923215667, "support": 7762}, "weighted avg": {"f1-score": 0.47933954699772663, "precision": 0.5875417713970594, "recall": 0.40479257923215667, "support": 7762}, "\u2205": {"f1-score": 0.7899434318038968, "precision": 0.9682588597842835, "recall": 0.6670912951167728, "support": 4710}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 339}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 124}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 172}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 141}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 92}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1610}},
  "ppcr": 0.41806235506312805
}
```
</details>
