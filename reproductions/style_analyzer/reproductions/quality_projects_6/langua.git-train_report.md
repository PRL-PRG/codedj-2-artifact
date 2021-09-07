# Train report for javascript / file:///tmp/top-repos-quality-repos-hq78x1nq/langua.git HEAD e46c690b3eec9f92e434060f6948d1759592140b

### Classification report

PPCR: 0.788

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.970| 0.979| 0.852| 0.975| 0.907| 17333| 19922| 0.870 |
| `␣` | 0.939| 0.951| 0.811| 0.945| 0.870| 6885| 8074| 0.853 |
| `'` | 1.000| 1.000| 0.637| 1.000| 0.778| 1943| 3050| 0.637 |
| `⏎␣⁻␣⁻` | 0.959| 0.799| 0.607| 0.872| 0.744| 877| 1154| 0.760 |
| `⏎` | 0.918| 0.810| 0.315| 0.861| 0.469| 842| 2165| 0.389 |
| `⏎␣⁺␣⁺` | 0.876| 0.916| 0.539| 0.896| 0.667| 694| 1181| 0.588 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 32| 741| 0.043 |
| `micro avg` | 0.961| 0.961| 0.757| 0.961| 0.847| 28606| 36287| 0.788 |
| `weighted avg` | 0.959| 0.961| 0.757| 0.960| 0.830| 28606| 36287| 0.788 |
| `macro avg` | 0.809| 0.779| 0.537| 0.793| 0.634| 28606| 36287| 0.788 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|2589 |16970 |351 |0 |0 |0 |12 |0 |
|1189 |190 |6546 |0 |41 |90 |18 |0 |
|1107 |0 |0 |1943 |0 |0 |0 |0 |
|1323 |124 |36 |0 |682 |0 |0 |0 |
|487 |38 |20 |0 |0 |636 |0 |0 |
|277 |167 |6 |0 |3 |0 |701 |0 |
|709 |0 |15 |0 |17 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/pageComponents/morph/morphResults.test.js | 86 |
| src/services/morphService.js | 85 |
| src/services/derivService.js | 72 |
| src/pageComponents/frequen/frequenResults.js | 70 |
| src/services/genService.js | 57 |
| src/services/frequenService.js | 50 |
| src/pages/morph/index.js | 40 |
| src/pages/frequen/index.js | 39 |
| src/pageComponents/deriv/derivResults.js | 35 |
| src/pageComponents/deriv/derivForm.js | 31 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1943}, "macro avg": {"f1-score": 0.792508729290047, "precision": 0.8088353125531507, "recall": 0.7793626323260632, "support": 28606}, "micro avg": {"f1-score": 0.9605677130671887, "precision": 0.9605677130671887, "recall": 0.9605677130671887, "support": 28606}, "weighted avg": {"f1-score": 0.9596536439311737, "precision": 0.9594469242811423, "recall": 0.9605677130671887, "support": 28606}, "\u2205": {"f1-score": 0.9746711848831198, "precision": 0.9703242037852364, "recall": 0.9790572895632609, "support": 17333}, "\u23ce": {"f1-score": 0.8605678233438486, "precision": 0.917900403768506, "recall": 0.8099762470308789, "support": 842}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 32}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.895774647887324, "precision": 0.8760330578512396, "recall": 0.9164265129682997, "support": 694}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8718905472636815, "precision": 0.9589603283173734, "recall": 0.7993158494868872, "support": 877}, "\u2423": {"f1-score": 0.9446569016523559, "precision": 0.9386291941496989, "recall": 0.9507625272331155, "support": 6885}},
  "cl_report_full": {"\u0027": {"f1-score": 0.7782896054476267, "precision": 1.0, "recall": 0.6370491803278688, "support": 3050}, "macro avg": {"f1-score": 0.633622726329642, "precision": 0.8088353125531507, "recall": 0.5372303433186314, "support": 36287}, "micro avg": {"f1-score": 0.8468710030357666, "precision": 0.9605677130671887, "recall": 0.7572408851654863, "support": 36287}, "weighted avg": {"f1-score": 0.8304206181960239, "precision": 0.9393937370334702, "recall": 0.7572408851654863, "support": 36287}, "\u2205": {"f1-score": 0.9072198016626125, "precision": 0.9703242037852364, "recall": 0.8518221062142355, "support": 19922}, "\u23ce": {"f1-score": 0.469050894085282, "precision": 0.917900403768506, "recall": 0.31501154734411085, "support": 2165}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 741}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.6670162558993182, "precision": 0.8760330578512396, "recall": 0.5385266723116003, "support": 1181}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7437665782493368, "precision": 0.9589603283173734, "recall": 0.6074523396880416, "support": 1154}, "\u2423": {"f1-score": 0.8700159489633174, "precision": 0.9386291941496989, "recall": 0.8107505573445628, "support": 8074}},
  "ppcr": 0.7883263978835395
}
```
</details>
