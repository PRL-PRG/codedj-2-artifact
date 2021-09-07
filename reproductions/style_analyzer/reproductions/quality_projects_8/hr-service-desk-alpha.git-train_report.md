# Train report for javascript / file:///tmp/top-repos-quality-repos-ar1ujtb6/hr-service-desk-alpha.git HEAD 784cd966b08dfff7c4f97bec6bdb43338255af43

### Classification report

PPCR: 0.503

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.973| 0.999| 0.641| 0.986| 0.773| 2199| 3425| 0.642 |
| `␣` | 0.995| 0.957| 0.323| 0.975| 0.487| 644| 1910| 0.337 |
| `'` | 1.000| 1.000| 0.823| 1.000| 0.903| 521| 633| 0.823 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 21| 170| 0.124 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 324| 0.022 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 183| 0.022 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 100| 0.000 |
| `weighted avg` | 0.972| 0.981| 0.494| 0.977| 0.615| 3396| 6745| 0.503 |
| `micro avg` | 0.981| 0.981| 0.494| 0.981| 0.657| 3396| 6745| 0.503 |
| `macro avg` | 0.424| 0.422| 0.255| 0.423| 0.309| 3396| 6745| 0.503 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1226 |2196 |3 |0 |0 |0 |0 |0 |
|1266 |28 |616 |0 |0 |0 |0 |0 |
|112 |0 |0 |521 |0 |0 |0 |0 |
|317 |7 |0 |0 |0 |0 |0 |0 |
|179 |4 |0 |0 |0 |0 |0 |0 |
|149 |21 |0 |0 |0 |0 |0 |0 |
|100 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| gatsby-node.js | 16 |
| src/templates/detailPage.js | 11 |
| src/templates/search.js | 10 |
| src/html.js | 7 |
| src/pages/index.js | 5 |
| src/templates/secondaryPage.js | 5 |
| gatsby-config.js | 4 |
| src/templates/downloads.js | 4 |
| src/pages/404.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 521}, "macro avg": {"f1-score": 0.4230448358714392, "precision": 0.42407967552332176, "recall": 0.42216535466431665, "support": 3396}, "micro avg": {"f1-score": 0.9814487632508834, "precision": 0.9814487632508834, "recall": 0.9814487632508834, "support": 3396}, "weighted avg": {"f1-score": 0.9767656717074643, "precision": 0.9724366296467641, "recall": 0.9814487632508834, "support": 3396}, "\u2205": {"f1-score": 0.9858585858585859, "precision": 0.973404255319149, "recall": 0.9986357435197817, "support": 2199}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 21}, "\u2423": {"f1-score": 0.9754552652414885, "precision": 0.9951534733441034, "recall": 0.9565217391304348, "support": 644}},
  "cl_report_full": {"\u0027": {"f1-score": 0.902946273830156, "precision": 1.0, "recall": 0.8230647709320695, "support": 633}, "macro avg": {"f1-score": 0.30902838164117, "precision": 0.42407967552332176, "recall": 0.25524939187842627, "support": 6745}, "micro avg": {"f1-score": 0.6573316241001873, "precision": 0.9814487632508834, "recall": 0.49414381022979986, "support": 6745}, "weighted avg": {"f1-score": 0.615255539078299, "precision": 0.8699262725804777, "recall": 0.49414381022979986, "support": 6745}, "\u2205": {"f1-score": 0.7731033268790706, "precision": 0.973404255319149, "recall": 0.6411678832116788, "support": 3425}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 324}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 100}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 183}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 170}, "\u2423": {"f1-score": 0.48714907077896397, "precision": 0.9951534733441034, "recall": 0.3225130890052356, "support": 1910}},
  "ppcr": 0.503484062268347
}
```
</details>
