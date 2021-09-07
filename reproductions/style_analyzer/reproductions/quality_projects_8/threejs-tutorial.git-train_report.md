# Train report for javascript / file:///tmp/top-repos-quality-repos-jt0offeq/threejs-tutorial.git HEAD a707eef11f8c9d35ed99040685f14141b00fd812

### Classification report

PPCR: 0.700

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.979| 0.986| 0.809| 0.982| 0.886| 24991| 30457| 0.821 |
| `␣` | 0.961| 0.973| 0.760| 0.967| 0.849| 16809| 21521| 0.781 |
| `'` | 1.000| 1.000| 0.675| 1.000| 0.806| 1340| 1984| 0.675 |
| `⏎⏎` | 0.930| 0.848| 0.153| 0.887| 0.263| 408| 2264| 0.180 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 241| 3525| 0.068 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 48| 636| 0.075 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 32| 247| 0.130 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 23| 606| 0.038 |
| `⏎⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 20| 533| 0.038 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 234| 0.021 |
| `⏎⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 96| 0.031 |
| `⏎⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 524| 0.000 |
| `⏎⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 92| 0.000 |
| `weighted avg` | 0.964| 0.972| 0.681| 0.968| 0.756| 43920| 62719| 0.700 |
| `micro avg` | 0.972| 0.972| 0.681| 0.972| 0.801| 43920| 62719| 0.700 |
| `macro avg` | 0.298| 0.293| 0.184| 0.295| 0.216| 43920| 62719| 0.700 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| ⏎⏎| '| ⏎⏎⇥⁺| ⏎⏎⇥⁻| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⇥⁺| ⏎⇥⁻| ⏎⏎␣⁺␣⁺␣⁺␣⁺| ⏎⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|5466 |24646 |345 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|4712 |438 |16362 |0 |9 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3284 |72 |152 |0 |17 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1856 |10 |52 |0 |346 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|644 |0 |0 |0 |0 |1340 |0 |0 |0 |0 |0 |0 |0 |0 |
|513 |0 |20 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|524 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|588 |7 |41 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|583 |8 |15 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|215 |0 |32 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|229 |5 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|93 |0 |3 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|92 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| lib/loaders/OBJLoader.js | 197 |
| lib/three.inspect.js | 132 |
| lib/control/OrbitControls.js | 111 |
| lib/loaders/MTLLoader.js | 73 |
| lib/CSS3DRenderer.js | 64 |
| 06-pixijs/lib/tween.esm.js | 55 |
| 06-pixijs/lib/tween.umd.js | 55 |
| lib/Tween.js | 47 |
| lib/control/DragControls.js | 32 |
| 02-intermediate/20-camera-animate.js | 30 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1340}, "macro avg": {"f1-score": 0.2951400350520726, "precision": 0.297684130149329, "recall": 0.2928954884026652, "support": 43920}, "micro avg": {"f1-score": 0.9720856102003643, "precision": 0.9720856102003643, "recall": 0.9720856102003643, "support": 43920}, "weighted avg": {"f1-score": 0.9679228991247041, "precision": 0.9638416094972567, "recall": 0.9720856102003643, "support": 43920}, "\u2205": {"f1-score": 0.9823624369731152, "precision": 0.9785595171920909, "recall": 0.986195030210876, "support": 24991}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 241}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 32}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u23ce": {"f1-score": 0.8871794871794871, "precision": 0.9301075268817204, "recall": 0.8480392156862745, "support": 408}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 20}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 48}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 23}, "\u2423": {"f1-score": 0.9672785315243415, "precision": 0.9612266478674656, "recall": 0.9734071033374978, "support": 16809}},
  "cl_report_full": {"\u0027": {"f1-score": 0.8062575210589652, "precision": 1.0, "recall": 0.6754032258064516, "support": 1984}, "macro avg": {"f1-score": 0.21566645433711817, "precision": 0.297684130149329, "recall": 0.18443978147710977, "support": 62719}, "micro avg": {"f1-score": 0.8007201867984509, "precision": 0.9720856102003643, "recall": 0.6807187614598447, "support": 62719}, "weighted avg": {"f1-score": 0.7564937213436836, "precision": 0.8702356438198227, "recall": 0.6807187614598447, "support": 62719}, "\u2205": {"f1-score": 0.8858616537569866, "precision": 0.9785595171920909, "recall": 0.8092064221689595, "support": 30457}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3525}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 247}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 234}, "\u23ce\u23ce": {"f1-score": 0.26251896813353565, "precision": 0.9301075268817204, "recall": 0.15282685512367492, "support": 2264}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 533}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 524}, "\u23ce\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 96}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 92}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 636}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 606}, "\u2423": {"f1-score": 0.8490257634330488, "precision": 0.9612266478674656, "recall": 0.760280656103341, "support": 21521}},
  "ppcr": 0.7002662670004305
}
```
</details>
