# Train report for javascript / file:///tmp/top-repos-quality-repos-6blnn9ka/desig-system-ta.git HEAD 52e2c65d1a684d17142cf48c65a2b8dfd24a0f3d

### Classification report

PPCR: 0.696

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.985| 0.999| 0.948| 0.992| 0.966| 3403| 3587| 0.949 |
| `"` | 1.000| 1.000| 0.882| 1.000| 0.937| 970| 1100| 0.882 |
| `␣` | 1.000| 0.836| 0.148| 0.911| 0.258| 311| 1754| 0.177 |
| `⏎␣⁻␣⁻` | 0.948| 0.965| 0.887| 0.956| 0.916| 284| 309| 0.919 |
| `⏎␣⁺␣⁺` | 0.956| 0.960| 0.830| 0.958| 0.889| 274| 317| 0.864 |
| `⏎` | 0.985| 0.974| 0.383| 0.979| 0.551| 267| 679| 0.393 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 170| 0.000 |
| `micro avg` | 0.985| 0.985| 0.686| 0.985| 0.808| 5509| 7916| 0.696 |
| `macro avg` | 0.839| 0.819| 0.582| 0.828| 0.645| 5509| 7916| 0.696 |
| `weighted avg` | 0.985| 0.985| 0.686| 0.985| 0.744| 5509| 7916| 0.696 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|184 |3400 |0 |0 |0 |0 |3 |0 |
|1443 |23 |260 |0 |4 |12 |12 |0 |
|130 |0 |0 |970 |0 |0 |0 |0 |
|412 |7 |0 |0 |260 |0 |0 |0 |
|43 |11 |0 |0 |0 |263 |0 |0 |
|25 |10 |0 |0 |0 |0 |274 |0 |
|170 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| build/webpack.dev.conf.js | 16 |
| build/webpack.prod.conf.js | 13 |
| build/check-versions.js | 8 |
| build/webpack.system.conf.js | 7 |
| docs/components/Preview.js | 7 |
| build/utils.js | 6 |
| build/build-system.js | 4 |
| build/build.js | 4 |
| build/webpack.base.conf.js | 4 |
| test/unit/specs/docs/statusLabels.spec.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 970}, "macro avg": {"f1-score": 0.8280806285031781, "precision": 0.8392186688431595, "recall": 0.8190795435977157, "support": 5509}, "micro avg": {"f1-score": 0.9851152659284806, "precision": 0.9851152659284806, "recall": 0.9851152659284806, "support": 5509}, "weighted avg": {"f1-score": 0.9847541209387036, "precision": 0.9852908072236389, "recall": 0.9851152659284806, "support": 5509}, "\u2205": {"f1-score": 0.9921213889699445, "precision": 0.9852216748768473, "recall": 0.9991184249191889, "support": 3403}, "\u23ce": {"f1-score": 0.9792843691148776, "precision": 0.9848484848484849, "recall": 0.9737827715355806, "support": 267}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9581056466302368, "precision": 0.9563636363636364, "recall": 0.9598540145985401, "support": 274}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.956369982547993, "precision": 0.9480968858131488, "recall": 0.9647887323943662, "support": 284}, "\u2423": {"f1-score": 0.9106830122591943, "precision": 1.0, "recall": 0.8360128617363344, "support": 311}},
  "cl_report_full": {"\"": {"f1-score": 0.9371980676328503, "precision": 1.0, "recall": 0.8818181818181818, "support": 1100}, "macro avg": {"f1-score": 0.645415338372038, "precision": 0.8392186688431595, "recall": 0.5824597904315959, "support": 7916}, "micro avg": {"f1-score": 0.8084916201117318, "precision": 0.9851152659284806, "recall": 0.6855735219807984, "support": 7916}, "weighted avg": {"f1-score": 0.7439026009045021, "precision": 0.966754860970049, "recall": 0.6855735219807984, "support": 7916}, "\u2205": {"f1-score": 0.966183574879227, "precision": 0.9852216748768473, "recall": 0.9478672985781991, "support": 3587}, "\u23ce": {"f1-score": 0.5514316012725345, "precision": 0.9848484848484849, "recall": 0.38291605301914583, "support": 679}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 170}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8885135135135135, "precision": 0.9563636363636364, "recall": 0.8296529968454258, "support": 317}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9163879598662208, "precision": 0.9480968858131488, "recall": 0.8867313915857605, "support": 309}, "\u2423": {"f1-score": 0.2581926514399206, "precision": 1.0, "recall": 0.14823261117445838, "support": 1754}},
  "ppcr": 0.6959322890348661
}
```
</details>
