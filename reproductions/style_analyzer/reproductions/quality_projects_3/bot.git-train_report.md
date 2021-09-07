# Train report for javascript / file:///tmp/top-repos-quality-repos-nu_6hocy/bot.git HEAD 010b1cd4cb62eef8e907f69a36c1752c5cb0bae8

### Classification report

PPCR: 0.894

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.992| 0.992| 0.964| 0.992| 0.978| 56682| 58293| 0.972 |
| `␣` | 0.972| 0.986| 0.904| 0.979| 0.937| 23941| 26112| 0.917 |
| `"` | 1.000| 1.000| 0.896| 1.000| 0.945| 5711| 6373| 0.896 |
| `⏎⇥⁻` | 0.948| 0.942| 0.933| 0.945| 0.940| 3128| 3160| 0.990 |
| `⏎⇥⁺` | 0.993| 0.940| 0.793| 0.966| 0.882| 2733| 3240| 0.844 |
| `⏎` | 0.990| 0.768| 0.091| 0.865| 0.167| 634| 5352| 0.118 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 13| 1354| 0.010 |
| `macro avg` | 0.842| 0.804| 0.654| 0.821| 0.693| 92842| 103884| 0.894 |
| `weighted avg` | 0.986| 0.986| 0.881| 0.986| 0.907| 92842| 103884| 0.894 |
| `micro avg` | 0.986| 0.986| 0.881| 0.986| 0.931| 92842| 103884| 0.894 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎⇥⁺| ⏎⇥⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1611 |56218 |389 |0 |1 |0 |74 |0 |
|2171 |227 |23610 |0 |0 |16 |88 |0 |
|662 |0 |0 |5711 |0 |0 |0 |0 |
|4718 |36 |110 |0 |487 |1 |0 |0 |
|507 |48 |116 |0 |1 |2568 |0 |0 |
|32 |119 |61 |0 |0 |0 |2948 |0 |
|1341 |0 |10 |0 |3 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| Web/controllers/dashboard/administration.js | 78 |
| Database/Schemas/Schema.js | 61 |
| Internals/Events/ready/GAB.Ready.js | 55 |
| Web/controllers/maintainer.js | 48 |
| Web/helpers.js | 43 |
| Web/parsers.js | 41 |
| Web/controllers/activity.js | 40 |
| Database/Document.js | 35 |
| master.js | 35 |
| Modules/MessageUtils/PaginatedEmbed.js | 26 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 5711}, "macro avg": {"f1-score": 0.8209983366024611, "precision": 0.8421921610998174, "recall": 0.804029878999257, "support": 92842}, "micro avg": {"f1-score": 0.985997716550699, "precision": 0.985997716550699, "recall": 0.985997716550699, "support": 92842}, "weighted avg": {"f1-score": 0.9858307433254879, "precision": 0.9859267503724879, "recall": 0.985997716550699, "support": 92842}, "\u2205": {"f1-score": 0.9921115326921379, "precision": 0.9924092642282164, "recall": 0.9918139797466567, "support": 56682}, "\u23ce": {"f1-score": 0.8650088809946714, "precision": 0.9898373983739838, "recall": 0.7681388012618297, "support": 634}, "\u23ce\u21e5\u207a": {"f1-score": 0.9657766077472734, "precision": 0.9934235976789169, "recall": 0.9396267837541163, "support": 2733}, "\u23ce\u21e5\u207b": {"f1-score": 0.9451747354921449, "precision": 0.9479099678456592, "recall": 0.9424552429667519, "support": 3128}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 13}, "\u2423": {"f1-score": 0.9789165992910007, "precision": 0.971764899571946, "recall": 0.9861743452654442, "support": 23941}},
  "cl_report_full": {"\"": {"f1-score": 0.9452168156239656, "precision": 1.0, "recall": 0.8961242742821277, "support": 6373}, "macro avg": {"f1-score": 0.6927019111840914, "precision": 0.8421921610998174, "recall": 0.6544583178299522, "support": 103884}, "micro avg": {"f1-score": 0.9306548193934712, "precision": 0.985997716550699, "recall": 0.8811944091486659, "support": 103884}, "weighted avg": {"f1-score": 0.9070433068565469, "precision": 0.9732965231243274, "recall": 0.8811944091486659, "support": 103884}, "\u2205": {"f1-score": 0.9782062101425949, "precision": 0.9924092642282164, "recall": 0.9644039593090079, "support": 58293}, "\u23ce": {"f1-score": 0.16666666666666663, "precision": 0.9898373983739838, "recall": 0.09099402092675635, "support": 5352}, "\u23ce\u21e5\u207a": {"f1-score": 0.8817167381974249, "precision": 0.9934235976789169, "recall": 0.7925925925925926, "support": 3240}, "\u23ce\u21e5\u207b": {"f1-score": 0.9403508771929824, "precision": 0.9479099678456592, "recall": 0.9329113924050633, "support": 3160}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1354}, "\u2423": {"f1-score": 0.9367560704650055, "precision": 0.971764899571946, "recall": 0.9041819852941176, "support": 26112}},
  "ppcr": 0.8937083670247584
}
```
</details>
