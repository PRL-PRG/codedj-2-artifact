# Train report for javascript / file:///tmp/top-repos-quality-repos-lo0xyz_1/telescope HEAD 534030114f47696fe3f3b08ea7ca49467428f2af

### Classification report

PPCR: 0.579

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.913| 1.000| 0.757| 0.954| 0.828| 324| 428| 0.757 |
| `'` | 1.000| 1.000| 0.812| 1.000| 0.896| 203| 250| 0.812 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 28| 192| 0.146 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 93| 0.032 |
| `weighted avg` | 0.894| 0.944| 0.547| 0.918| 0.600| 558| 963| 0.579 |
| `macro avg` | 0.478| 0.500| 0.392| 0.489| 0.431| 558| 963| 0.579 |
| `micro avg` | 0.944| 0.944| 0.547| 0.944| 0.693| 558| 963| 0.579 |

### Confusion matrix

|refusal|  ∅| '| ␣| ⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|104 |324 |0 |0 |0 |
|47 |0 |203 |0 |0 |
|164 |28 |0 |0 |0 |
|90 |3 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| resources/js/base.js | 21 |
| resources/js/app.js | 5 |
| webpack.mix.js | 3 |
| resources/js/routes.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 203}, "macro avg": {"f1-score": 0.4885861561119293, "precision": 0.47816901408450707, "recall": 0.5, "support": 558}, "micro avg": {"f1-score": 0.9444444444444444, "precision": 0.9444444444444444, "recall": 0.9444444444444444, "support": 558}, "weighted avg": {"f1-score": 0.9179348715431188, "precision": 0.8937402190923318, "recall": 0.9444444444444444, "support": 558}, "\u2205": {"f1-score": 0.9543446244477172, "precision": 0.9126760563380282, "recall": 1.0, "support": 324}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 28}},
  "cl_report_full": {"\u0027": {"f1-score": 0.8962472406181016, "precision": 1.0, "recall": 0.812, "support": 250}, "macro avg": {"f1-score": 0.4309583618786633, "precision": 0.47816901408450707, "recall": 0.39225233644859814, "support": 963}, "micro avg": {"f1-score": 0.6929651545036161, "precision": 0.9444444444444444, "recall": 0.5472481827622014, "support": 963}, "weighted avg": {"f1-score": 0.600486715167445, "precision": 0.6652392026092171, "recall": 0.5472481827622014, "support": 963}, "\u2205": {"f1-score": 0.8275862068965518, "precision": 0.9126760563380282, "recall": 0.7570093457943925, "support": 428}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 93}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 192}},
  "ppcr": 0.5794392523364486
}
```
</details>
