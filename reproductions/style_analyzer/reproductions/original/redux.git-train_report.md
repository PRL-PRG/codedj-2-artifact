# Train report for javascript / file:///tmp/top-repos-quality-repos-0iidj2j0/redux.git HEAD 3ba88f24d31f6cc1670d0e0ccc968ba5c1261112

### Classification report

PPCR: 0.665

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.956| 0.997| 0.927| 0.976| 0.941| 12998| 13983| 0.930 |
| `␣` | 0.953| 0.935| 0.394| 0.944| 0.558| 3795| 9007| 0.421 |
| `'` | 1.000| 1.000| 0.952| 1.000| 0.975| 2767| 2908| 0.952 |
| `⏎` | 0.909| 0.675| 0.160| 0.775| 0.272| 446| 1882| 0.237 |
| `⏎⏎` | 0.883| 0.836| 0.308| 0.859| 0.457| 281| 762| 0.369 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 180| 1205| 0.149 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 172| 1310| 0.131 |
| `weighted avg` | 0.943| 0.960| 0.638| 0.951| 0.704| 20639| 31057| 0.665 |
| `macro avg` | 0.672| 0.635| 0.392| 0.651| 0.458| 20639| 31057| 0.665 |
| `micro avg` | 0.960| 0.960| 0.638| 0.960| 0.766| 20639| 31057| 0.665 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|985 |12957 |41 |0 |0 |0 |0 |0 |
|5212 |234 |3550 |0 |0 |0 |0 |11 |
|141 |0 |0 |2767 |0 |0 |0 |0 |
|1436 |72 |53 |0 |301 |0 |0 |20 |
|1138 |107 |65 |0 |0 |0 |0 |0 |
|1025 |165 |15 |0 |0 |0 |0 |0 |
|481 |16 |0 |0 |30 |0 |0 |235 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| examples/todomvc/src/reducers/todos.spec.js | 61 |
| examples/todos-flow/src/reducers/todos.spec.js | 26 |
| examples/todos/src/reducers/todos.spec.js | 21 |
| website/docusaurus.config.js | 20 |
| examples/todomvc/src/components/Footer.spec.js | 19 |
| examples/async/src/containers/App.js | 17 |
| examples/real-world/src/actions/index.js | 14 |
| examples/todomvc/src/components/MainSection.spec.js | 14 |
| examples/real-world/src/middleware/api.js | 14 |
| examples/todomvc/src/components/TodoItem.js | 13 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 2767}, "macro avg": {"f1-score": 0.6506233452173856, "precision": 0.6717522642035532, "recall": 0.6347819805070566, "support": 20639}, "micro avg": {"f1-score": 0.9598333252580067, "precision": 0.9598333252580067, "recall": 0.9598333252580067, "support": 20639}, "weighted avg": {"f1-score": 0.950851730867817, "precision": 0.9432022841896694, "recall": 0.9598333252580067, "support": 20639}, "\u2205": {"f1-score": 0.9760819616558063, "precision": 0.9561655966349347, "recall": 0.9968456685643945, "support": 12998}, "\u23ce": {"f1-score": 0.7747747747747747, "precision": 0.9093655589123867, "recall": 0.6748878923766816, "support": 446}, "\u23ce\u23ce": {"f1-score": 0.8592321755027422, "precision": 0.8834586466165414, "recall": 0.8362989323843416, "support": 281}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 172}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 180}, "\u2423": {"f1-score": 0.944274504588376, "precision": 0.9532760472610097, "recall": 0.9354413702239789, "support": 3795}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9751541850220264, "precision": 1.0, "recall": 0.9515130674002751, "support": 2908}, "macro avg": {"f1-score": 0.45760557415294617, "precision": 0.6717522642035532, "recall": 0.39151590515059725, "support": 31057}, "micro avg": {"f1-score": 0.7664035902197462, "precision": 0.9598333252580067, "recall": 0.6378594197765399, "support": 31057}, "weighted avg": {"f1-score": 0.7044959777596604, "precision": 0.8773816648749436, "recall": 0.6378594197765399, "support": 31057}, "\u2205": {"f1-score": 0.9411636522118108, "precision": 0.9561655966349347, "recall": 0.9266251877279553, "support": 13983}, "\u23ce": {"f1-score": 0.272028920018075, "precision": 0.9093655589123867, "recall": 0.15993623804463336, "support": 1882}, "\u23ce\u23ce": {"f1-score": 0.4571984435797666, "precision": 0.8834586466165414, "recall": 0.3083989501312336, "support": 762}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1310}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1205}, "\u2423": {"f1-score": 0.5576938182389444, "precision": 0.9532760472610097, "recall": 0.39413789275008326, "support": 9007}},
  "ppcr": 0.6645522748494703
}
```
</details>
