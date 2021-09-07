# Train report for javascript / file:///tmp/top-repos-quality-repos-1qqw5wjb/speedlazer.git HEAD 7876dce85b5f5706b475267cd03976f4a65d3755

### Classification report

PPCR: 0.913

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.957| 0.986| 0.975| 0.971| 0.966| 50462| 51047| 0.989 |
| `␣` | 0.953| 0.946| 0.845| 0.949| 0.896| 27054| 30285| 0.893 |
| `"` | 1.000| 1.000| 0.972| 1.000| 0.986| 6803| 6998| 0.972 |
| `⏎` | 0.941| 0.848| 0.494| 0.892| 0.648| 4491| 7705| 0.583 |
| `⏎␣⁻␣⁻` | 0.913| 0.866| 0.781| 0.889| 0.842| 3587| 3976| 0.902 |
| `⏎␣⁺␣⁺` | 0.899| 0.772| 0.667| 0.831| 0.766| 3558| 4118| 0.864 |
| `⏎⏎` | 0.925| 0.655| 0.179| 0.767| 0.300| 357| 1307| 0.273 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 87| 98| 0.888 |
| `macro avg` | 0.823| 0.759| 0.614| 0.787| 0.675| 96399| 105534| 0.913 |
| `micro avg` | 0.955| 0.955| 0.872| 0.955| 0.912| 96399| 105534| 0.913 |
| `weighted avg` | 0.953| 0.955| 0.872| 0.954| 0.902| 96399| 105534| 0.913 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|585 |49758 |612 |0 |0 |0 |92 |0 |0 |
|3231 |839 |25587 |158 |0 |310 |160 |0 |0 |
|3214 |346 |316 |3810 |0 |0 |0 |19 |0 |
|195 |0 |0 |0 |6803 |0 |0 |0 |0 |
|560 |507 |304 |0 |0 |2747 |0 |0 |0 |
|389 |430 |36 |14 |0 |0 |3107 |0 |0 |
|950 |54 |4 |65 |0 |0 |0 |234 |0 |
|11 |41 |1 |0 |0 |0 |45 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/components/Composable.js | 230 |
| src/editor/lib/render-crafty.js | 180 |
| src/components/Weapon.js | 173 |
| src/components/ParticleEmitter.js | 138 |
| src/editor/components/Menu.js | 116 |
| src/components/controls/Gamepad.js | 115 |
| src/editor/modules/compositions/index.js | 106 |
| src/editor/modules/weapons/index.js | 92 |
| src/scripts/stage1/enemies/battleship.lazer.js | 91 |
| src/editor/modules/audio/index.js | 89 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 6803}, "macro avg": {"f1-score": 0.7874688480236678, "precision": 0.823454171198702, "recall": 0.75923699997107, "support": 96399}, "micro avg": {"f1-score": 0.9548439299162854, "precision": 0.9548439299162854, "recall": 0.9548439299162854, "support": 96399}, "weighted avg": {"f1-score": 0.9536462785959685, "precision": 0.9534721263868835, "recall": 0.9548439299162854, "support": 96399}, "\u2205": {"f1-score": 0.9714849126780362, "precision": 0.9573448773448774, "recall": 0.9860489080892553, "support": 50462}, "\u23ce": {"f1-score": 0.8924806746310611, "precision": 0.9414381022979985, "recall": 0.8483633934535738, "support": 4491}, "\u23ce\u23ce": {"f1-score": 0.7672131147540984, "precision": 0.924901185770751, "recall": 0.6554621848739496, "support": 357}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8305366591080877, "precision": 0.8985933922145894, "recall": 0.7720629567172569, "support": 3558}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8888571019882706, "precision": 0.9127497062279671, "recall": 0.8661834402007248, "support": 3587}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 87}, "\u2423": {"f1-score": 0.9491783210297882, "precision": 0.9526061057334326, "recall": 0.9457751164337991, "support": 27054}},
  "cl_report_full": {"\"": {"f1-score": 0.9858705890877473, "precision": 1.0, "recall": 0.9721348956844813, "support": 6998}, "macro avg": {"f1-score": 0.6754338024248573, "precision": 0.823454171198702, "recall": 0.6142234304500191, "support": 105534}, "micro avg": {"f1-score": 0.9116489132534058, "precision": 0.9548439299162854, "recall": 0.8721928478026039, "support": 105534}, "weighted avg": {"f1-score": 0.9022544905878954, "precision": 0.9523886777253917, "recall": 0.8721928478026039, "support": 105534}, "\u2205": {"f1-score": 0.9659684339267341, "precision": 0.9573448773448774, "recall": 0.9747487609457951, "support": 51047}, "\u23ce": {"f1-score": 0.6484002722940776, "precision": 0.9414381022979985, "recall": 0.4944841012329656, "support": 7705}, "\u23ce\u23ce": {"f1-score": 0.3, "precision": 0.924901185770751, "recall": 0.17903596021423107, "support": 1307}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7657142857142857, "precision": 0.8985933922145894, "recall": 0.6670713938805245, "support": 4118}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8420054200542005, "precision": 0.9127497062279671, "recall": 0.7814386317907445, "support": 3976}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 98}, "\u2423": {"f1-score": 0.8955114183218129, "precision": 0.9526061057334326, "recall": 0.8448736998514116, "support": 30285}},
  "ppcr": 0.913440218318267
}
```
</details>
