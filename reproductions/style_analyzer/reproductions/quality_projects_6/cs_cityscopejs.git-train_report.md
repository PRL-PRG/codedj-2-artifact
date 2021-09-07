# Train report for javascript / file:///tmp/top-repos-quality-repos-tft7vsmd/cs_cityscopejs.git HEAD 7038e9aa1aa45c6f849f5602a8f34d1ebfec50d8

### Classification report

PPCR: 0.629

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.943| 1.000| 0.905| 0.971| 0.924| 10540| 11638| 0.906 |
| `␣` | 0.961| 0.839| 0.329| 0.896| 0.490| 2182| 5568| 0.392 |
| `"` | 1.000| 1.000| 0.315| 1.000| 0.479| 527| 1675| 0.315 |
| `⏎` | 0.962| 0.903| 0.265| 0.931| 0.416| 474| 1614| 0.294 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 167| 753| 0.222 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 141| 718| 0.196 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 352| 0.028 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 6| 0.333 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 5| 0.200 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 3| 0.000 |
| `micro avg` | 0.949| 0.949| 0.596| 0.949| 0.732| 14044| 22332| 0.629 |
| `weighted avg` | 0.927| 0.949| 0.596| 0.937| 0.670| 14044| 22332| 0.629 |
| `macro avg` | 0.387| 0.374| 0.181| 0.380| 0.231| 14044| 22332| 0.629 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| '| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1098 |10536 |4 |0 |0 |0 |0 |0 |0 |0 |0 |
|3386 |345 |1830 |7 |0 |0 |0 |0 |0 |0 |0 |
|1140 |38 |8 |428 |0 |0 |0 |0 |0 |0 |0 |
|1148 |0 |0 |0 |527 |0 |0 |0 |0 |0 |0 |
|3 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|586 |110 |57 |0 |0 |0 |0 |0 |0 |0 |0 |
|577 |136 |5 |0 |0 |0 |0 |0 |0 |0 |0 |
|342 |0 |0 |10 |0 |0 |0 |0 |0 |0 |0 |
|4 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|4 |2 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/views/GridEditor/EditorMenu/GridProps/CommitGrid/index.js | 55 |
| src/views/CityScopeJS/MenuContainer/EditMenu/index.js | 43 |
| src/redux/reducer.js | 38 |
| src/views/CityScopeJS/DeckglMap/utils/BaseMapUtils.js | 34 |
| src/views/CityScopeJS/VisContainer/AreaCalc/index.js | 27 |
| src/views/GridEditor/EditorMenu/GridProps/GridMaker/gridCreator.js | 25 |
| src/views/CityScopeJS/DeckglMap/components/AnimationComponent.js | 24 |
| src/views/CityScopeJS/DeckglMap/index.js | 24 |
| src/views/CityScopeJS/VisContainer/Radar/index.js | 22 |
| src/views/CityScopeJS/MenuContainer/SaveMenu/components/ScenarioItems/index.js | 22 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 527}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.379789082710825, "precision": 0.3866341948710845, "recall": 0.37412541898473584, "support": 14044}, "micro avg": {"f1-score": 0.9485189404727997, "precision": 0.9485189404727997, "recall": 0.9485189404727997, "support": 14044}, "weighted avg": {"f1-score": 0.9366429520714233, "precision": 0.9273445026296743, "recall": 0.9485189404727997, "support": 14044}, "\u2205": {"f1-score": 0.970702045328911, "precision": 0.9434097421203438, "recall": 0.9996204933586338, "support": 10540}, "\u23ce": {"f1-score": 0.9314472252448311, "precision": 0.9617977528089887, "recall": 0.9029535864978903, "support": 474}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 167}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 141}, "\u2423": {"f1-score": 0.8957415565345082, "precision": 0.9611344537815126, "recall": 0.8386801099908341, "support": 2182}},
  "cl_report_full": {"\"": {"f1-score": 0.4786557674841053, "precision": 1.0, "recall": 0.3146268656716418, "support": 1675}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "macro avg": {"f1-score": 0.23081876323551, "precision": 0.3866341948710845, "recall": 0.18137805273485985, "support": 22332}, "micro avg": {"f1-score": 0.7324059819661315, "precision": 0.9485189404727997, "recall": 0.596498298405875, "support": 22332}, "weighted avg": {"f1-score": 0.669588235840613, "precision": 0.875798889059902, "recall": 0.596498298405875, "support": 22332}, "\u2205": {"f1-score": 0.923967377006051, "precision": 0.9434097421203438, "recall": 0.9053101907544252, "support": 11638}, "\u23ce": {"f1-score": 0.4157357940747936, "precision": 0.9617977528089887, "recall": 0.265179677819083, "support": 1614}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 352}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 753}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 718}, "\u2423": {"f1-score": 0.48982869379014987, "precision": 0.9611344537815126, "recall": 0.3286637931034483, "support": 5568}},
  "ppcr": 0.6288733655740641
}
```
</details>
