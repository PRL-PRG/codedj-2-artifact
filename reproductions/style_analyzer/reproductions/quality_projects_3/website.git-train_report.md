# Train report for javascript / file:///tmp/top-repos-quality-repos-uxe5t4qv/website.git HEAD f2e7abc3af248dc6ef69b0dc0868643db2adb8e1

### Classification report

PPCR: 0.491

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.944| 1.000| 0.785| 0.971| 0.858| 4552| 5796| 0.785 |
| `␣` | 0.995| 0.550| 0.077| 0.708| 0.142| 360| 2580| 0.140 |
| `'` | 1.000| 1.000| 0.282| 1.000| 0.440| 164| 581| 0.282 |
| `"` | 1.000| 1.000| 0.464| 1.000| 0.634| 130| 280| 0.464 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 39| 348| 0.112 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 35| 358| 0.098 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 33| 643| 0.051 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 241| 0.000 |
| `macro avg` | 0.492| 0.444| 0.201| 0.460| 0.259| 5313| 10827| 0.491 |
| `weighted avg` | 0.932| 0.949| 0.466| 0.936| 0.533| 5313| 10827| 0.491 |
| `micro avg` | 0.949| 0.949| 0.466| 0.949| 0.625| 5313| 10827| 0.491 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1244 |4552 |0 |0 |0 |0 |0 |0 |0 |
|2220 |162 |198 |0 |0 |0 |0 |0 |0 |
|610 |32 |1 |0 |0 |0 |0 |0 |0 |
|417 |0 |0 |0 |164 |0 |0 |0 |0 |
|323 |35 |0 |0 |0 |0 |0 |0 |0 |
|309 |39 |0 |0 |0 |0 |0 |0 |0 |
|150 |0 |0 |0 |0 |0 |0 |130 |0 |
|241 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/components/HorizontalCard/HorizontalCard.js | 56 |
| src/components/Navbar.js | 25 |
| src/templates/staff-page.js | 16 |
| gatsby-config.js | 15 |
| src/templates/index-page.js | 13 |
| src/components/Map.js | 11 |
| src/components/FloatingCard.js | 10 |
| src/templates/ensembles.js | 9 |
| src/templates/concerts-page.js | 9 |
| src/components/HorizontalCard/index.js | 8 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 130}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 164}, "macro avg": {"f1-score": 0.45997650673379464, "precision": 0.4924216518276027, "recall": 0.44375, "support": 5313}, "micro avg": {"f1-score": 0.9493694711086016, "precision": 0.9493694711086016, "recall": 0.9493694711086016, "support": 5313}, "weighted avg": {"f1-score": 0.9356029876065246, "precision": 0.9318825897961881, "recall": 0.9493694711086016, "support": 5313}, "\u2205": {"f1-score": 0.9714041826717883, "precision": 0.9443983402489626, "recall": 1.0, "support": 4552}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 33}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 35}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 39}, "\u2423": {"f1-score": 0.708407871198569, "precision": 0.9949748743718593, "recall": 0.55, "support": 360}},
  "cl_report_full": {"\"": {"f1-score": 0.6341463414634146, "precision": 1.0, "recall": 0.4642857142857143, "support": 280}, "\u0027": {"f1-score": 0.44026845637583895, "precision": 1.0, "recall": 0.2822719449225473, "support": 581}, "macro avg": {"f1-score": 0.2593106966285298, "precision": 0.4924216518276027, "recall": 0.20108388317582526, "support": 10827}, "micro avg": {"f1-score": 0.6250309789343247, "precision": 0.9493694711086016, "recall": 0.4658723561466704, "support": 10827}, "weighted avg": {"f1-score": 0.5330651003177747, "precision": 0.822182317905457, "recall": 0.4658723561466704, "support": 10827}, "\u2205": {"f1-score": 0.8575734740015072, "precision": 0.9443983402489626, "recall": 0.7853692201518289, "support": 5796}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 643}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 241}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 358}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 348}, "\u2423": {"f1-score": 0.14249730118747753, "precision": 0.9949748743718593, "recall": 0.07674418604651163, "support": 2580}},
  "ppcr": 0.49071765031864784
}
```
</details>
