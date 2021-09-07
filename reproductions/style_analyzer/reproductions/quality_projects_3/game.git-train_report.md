# Train report for javascript / file:///tmp/top-repos-quality-repos-rpiesur9/game.git HEAD e5691501199e1d7fa8679275bb79b9176899e3ed

### Classification report

PPCR: 0.371

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.999| 1.000| 0.527| 0.999| 0.690| 3696| 7015| 0.527 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 716| 0.006 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1868| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 378| 0.000 |
| `macro avg` | 0.250| 0.250| 0.132| 0.250| 0.172| 3700| 9977| 0.371 |
| `weighted avg` | 0.998| 0.999| 0.370| 0.998| 0.485| 3700| 9977| 0.371 |
| `micro avg` | 0.999| 0.999| 0.370| 0.999| 0.540| 3700| 9977| 0.371 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|3319 |3696 |0 |0 |0 |
|1868 |0 |0 |0 |0 |
|378 |0 |0 |0 |0 |
|712 |4 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| exiled_game/static/js/MainMenu.js | 2 |
| exiled_game/static/js/Game.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.24986479177934018, "precision": 0.24972972972972973, "recall": 0.25, "support": 3700}, "micro avg": {"f1-score": 0.9989189189189189, "precision": 0.9989189189189189, "recall": 0.9989189189189189, "support": 3700}, "weighted avg": {"f1-score": 0.998378670720477, "precision": 0.9978390065741417, "recall": 0.9989189189189189, "support": 3700}, "\u2205": {"f1-score": 0.9994591671173607, "precision": 0.9989189189189189, "recall": 1.0, "support": 3696}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 378}, "macro avg": {"f1-score": 0.17246850209986, "precision": 0.24972972972972973, "recall": 0.13171774768353528, "support": 9977}, "micro avg": {"f1-score": 0.5404694011844703, "precision": 0.9989189189189189, "recall": 0.37045203969129, "support": 9977}, "weighted avg": {"f1-score": 0.48506226009041514, "precision": 0.7023570428201079, "recall": 0.37045203969129, "support": 9977}, "\u2205": {"f1-score": 0.68987400839944, "precision": 0.9989189189189189, "recall": 0.5268709907341411, "support": 7015}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 716}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1868}},
  "ppcr": 0.370852961812168
}
```
</details>
