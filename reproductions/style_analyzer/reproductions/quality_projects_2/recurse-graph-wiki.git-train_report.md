# Train report for javascript / file:///tmp/top-repos-quality-repos-smy0n1pm/recurse-graph-wiki.git HEAD 849622735ce0aa7ee4ad2e8fc570bc579152edc6

### Classification report

PPCR: 0.480

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.995| 0.996| 0.691| 0.996| 0.815| 3577| 5161| 0.693 |
| `'` | 1.000| 1.000| 0.498| 1.000| 0.665| 952| 1910| 0.498 |
| `␣` | 0.972| 0.989| 0.247| 0.980| 0.394| 701| 2802| 0.250 |
| `⏎` | 0.932| 0.965| 0.221| 0.948| 0.357| 229| 1000| 0.229 |
| `⏎␣⁻␣⁻` | 0.985| 0.951| 0.547| 0.968| 0.704| 206| 358| 0.575 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 240| 0.042 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 361| 0.025 |
| `weighted avg` | 0.987| 0.990| 0.475| 0.988| 0.608| 5684| 11832| 0.480 |
| `micro avg` | 0.990| 0.990| 0.475| 0.990| 0.642| 5684| 11832| 0.480 |
| `macro avg` | 0.698| 0.700| 0.315| 0.699| 0.419| 5684| 11832| 0.480 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1584 |3564 |13 |0 |0 |0 |0 |0 |
|2101 |2 |693 |0 |3 |0 |3 |0 |
|958 |0 |0 |952 |0 |0 |0 |0 |
|771 |1 |7 |0 |221 |0 |0 |0 |
|352 |6 |0 |0 |3 |0 |0 |0 |
|152 |10 |0 |0 |0 |0 |196 |0 |
|230 |0 |0 |0 |10 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| app/tailwind/config/colors.js | 10 |
| public/sw.js | 7 |
| app/lib/ordts/sequence.js | 7 |
| app/lib/idb.js | 5 |
| testem.js | 4 |
| app/controllers/page.js | 4 |
| app/tailwind/config/fonts.js | 3 |
| app/lib/decorators.js | 3 |
| app/lib/ordts/graph.js | 3 |
| app/services/sw.js | 3 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 952}, "macro avg": {"f1-score": 0.6988754049577312, "precision": 0.6977229664059871, "recall": 0.7002107448900298, "support": 5684}, "micro avg": {"f1-score": 0.9897959183673469, "precision": 0.9897959183673469, "recall": 0.9897959183673469, "support": 5684}, "weighted avg": {"f1-score": 0.988164300247554, "precision": 0.9865944721241727, "recall": 0.9897959183673469, "support": 5684}, "\u2205": {"f1-score": 0.9955307262569832, "precision": 0.9946971811331287, "recall": 0.9963656695554934, "support": 3577}, "\u23ce": {"f1-score": 0.9484978540772533, "precision": 0.9324894514767933, "recall": 0.9650655021834061, "support": 229}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9679012345679012, "precision": 0.9849246231155779, "recall": 0.9514563106796117, "support": 206}, "\u2423": {"f1-score": 0.9801980198019802, "precision": 0.9719495091164095, "recall": 0.9885877318116976, "support": 701}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6652690426275332, "precision": 1.0, "recall": 0.49842931937172774, "support": 1910}, "macro avg": {"f1-score": 0.4194075691681996, "precision": 0.6977229664059871, "recall": 0.31497179108265855, "support": 11832}, "micro avg": {"f1-score": 0.6423841059602649, "precision": 0.9897959183673469, "recall": 0.47549019607843135, "support": 11832}, "weighted avg": {"f1-score": 0.6078405655043686, "precision": 0.9340878247907731, "recall": 0.47549019607843135, "support": 11832}, "\u2205": {"f1-score": 0.8151875571820677, "precision": 0.9946971811331287, "recall": 0.6905638442162372, "support": 5161}, "\u23ce": {"f1-score": 0.35731608730800324, "precision": 0.9324894514767933, "recall": 0.221, "support": 1000}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 240}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 361}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.703770197486535, "precision": 0.9849246231155779, "recall": 0.547486033519553, "support": 358}, "\u2423": {"f1-score": 0.3943100995732575, "precision": 0.9719495091164095, "recall": 0.24732334047109208, "support": 2802}},
  "ppcr": 0.4803921568627451
}
```
</details>
