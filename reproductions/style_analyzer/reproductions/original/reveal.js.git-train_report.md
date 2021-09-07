# Train report for javascript / file:///tmp/top-repos-quality-repos-aydhjq3i/reveal.js.git HEAD 01d8d669bc2b681b595262ccbe27293eec2fcb44

### Classification report

PPCR: 0.852

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.972| 0.865| 0.844| 0.915| 0.904| 27037| 27703| 0.976 |
| `␣` | 0.820| 0.967| 0.920| 0.887| 0.867| 19628| 20624| 0.952 |
| `'` | 1.000| 1.000| 0.967| 1.000| 0.983| 2509| 2595| 0.967 |
| `⏎` | 0.975| 0.822| 0.420| 0.892| 0.587| 1811| 3545| 0.511 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 156| 1831| 0.085 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 68| 1547| 0.044 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 30| 1571| 0.019 |
| `⏎⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 346| 0.023 |
| `⏎⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 387| 0.000 |
| `weighted avg` | 0.910| 0.905| 0.771| 0.903| 0.790| 51247| 60149| 0.852 |
| `macro avg` | 0.419| 0.406| 0.350| 0.410| 0.371| 51247| 60149| 0.852 |
| `micro avg` | 0.905| 0.905| 0.771| 0.905| 0.832| 51247| 60149| 0.852 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎⇥⁺| ⏎⇥⁻| ⏎⏎⇥⁻| ⏎⏎⇥⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|666 |23390 |3647 |0 |0 |0 |0 |0 |0 |0 |
|996 |655 |18973 |0 |0 |0 |0 |0 |0 |0 |
|86 |0 |0 |2509 |0 |0 |0 |0 |0 |0 |
|1734 |12 |311 |0 |1488 |0 |0 |0 |0 |0 |
|1675 |0 |138 |0 |18 |0 |0 |0 |0 |0 |
|1479 |0 |68 |0 |0 |0 |0 |0 |0 |0 |
|1541 |6 |10 |0 |14 |0 |0 |0 |0 |0 |
|338 |1 |1 |0 |6 |0 |0 |0 |0 |0 |
|387 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| test/qunit-2.5.0.js | 4149 |
| js/reveal.js | 260 |
| plugin/markdown/markdown.js | 73 |
| gulpfile.js | 66 |
| plugin/multiplex/index.js | 49 |
| js/controllers/autoanimate.js | 45 |
| js/controllers/slidecontent.js | 34 |
| js/controllers/plugins.js | 33 |
| plugin/zoom-js/zoom.js | 30 |
| plugin/notes/notes.js | 30 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 2509}, "macro avg": {"f1-score": 0.410483020393176, "precision": 0.4185253886967561, "recall": 0.40593173099140456, "support": 51247}, "micro avg": {"f1-score": 0.9046383202919195, "precision": 0.9046383202919195, "recall": 0.9046383202919195, "support": 51247}, "weighted avg": {"f1-score": 0.9032063721084891, "precision": 0.910150927471256, "recall": 0.9046383202919195, "support": 51247}, "\u2205": {"f1-score": 0.9154419678675564, "precision": 0.9719913563829787, "recall": 0.8651107741243481, "support": 27037}, "\u23ce": {"f1-score": 0.8918189991009888, "precision": 0.9750982961992136, "recall": 0.8216454997239094, "support": 1811}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 68}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 30}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 156}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u2423": {"f1-score": 0.8870862165700392, "precision": 0.8196388456886124, "recall": 0.9666293050743835, "support": 19628}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9831504702194358, "precision": 1.0, "recall": 0.966859344894027, "support": 2595}, "macro avg": {"f1-score": 0.3711758531172836, "precision": 0.4185253886967561, "recall": 0.3500962211463321, "support": 60149}, "micro avg": {"f1-score": 0.832345865201623, "precision": 0.9046383202919195, "recall": 0.7707526309664333, "support": 60149}, "weighted avg": {"f1-score": 0.7904521341267651, "precision": 0.8293243704697636, "recall": 0.7707526309664333, "support": 60149}, "\u2205": {"f1-score": 0.9036644966870786, "precision": 0.9719913563829787, "recall": 0.8443128903006895, "support": 27703}, "\u23ce": {"f1-score": 0.5868664957602051, "precision": 0.9750982961992136, "recall": 0.41974612129760225, "support": 3545}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1547}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1571}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1831}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 387}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 346}, "\u2423": {"f1-score": 0.866901215388833, "precision": 0.8196388456886124, "recall": 0.9199476338246703, "support": 20624}},
  "ppcr": 0.8520008645197759
}
```
</details>
