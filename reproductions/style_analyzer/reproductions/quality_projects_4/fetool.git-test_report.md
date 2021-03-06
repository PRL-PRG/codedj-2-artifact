# Test report for javascript / file:///tmp/top-repos-quality-repos-xbezziop/fetool.git HEAD 5d553810ecfdf3218461a63d942aa1b5dc1073b3

### Classification report

PPCR: 0.807

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.981| 0.976| 0.952| 0.978| 0.966| 11657| 11957| 0.975 |
| `␣` | 0.927| 0.971| 0.859| 0.949| 0.892| 6003| 6788| 0.884 |
| `⏎` | 0.827| 0.897| 0.577| 0.861| 0.680| 837| 1301| 0.643 |
| `"` | 0.952| 0.846| 0.270| 0.896| 0.420| 680| 2132| 0.319 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 66| 489| 0.135 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 53| 190| 0.279 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 53| 474| 0.112 |
| `'` | 0.220| 0.225| 0.019| 0.222| 0.034| 40| 484| 0.083 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 28| 90| 0.311 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 21| 59| 0.356 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 83| 0.072 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 56| 0.089 |
| `⇥` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 7| 0.000 |
| `⇥⇥⇥` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 4| 0.000 |
| `⇥⇥⇥⇥` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `␣␣␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `␣␣␣␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `macro avg` | 0.230| 0.230| 0.157| 0.230| 0.176| 19449| 24114| 0.807 |
| `weighted avg` | 0.943| 0.953| 0.769| 0.948| 0.805| 19449| 24114| 0.807 |
| `micro avg` | 0.953| 0.953| 0.769| 0.953| 0.851| 19449| 24114| 0.807 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| ⏎⇥⁺| ⇥⇥⇥⇥| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎⇥⁻| ⇥| ␣␣␣␣| ⇥⇥⇥| ␣␣␣| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|300 |11378 |278 |0 |0 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|785 |129 |5831 |0 |42 |0 |0 |0 |0 |0 |0 |0 |0 |0 |1 |0 |0 |
|1452 |44 |24 |575 |0 |31 |0 |0 |0 |0 |5 |0 |0 |0 |0 |1 |0 |
|464 |8 |77 |1 |751 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|444 |3 |1 |27 |0 |9 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|423 |12 |39 |0 |15 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|421 |20 |7 |0 |26 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|137 |1 |2 |0 |50 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|62 |3 |10 |0 |15 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|51 |1 |0 |0 |4 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|38 |1 |19 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|77 |1 |0 |0 |5 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|7 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|4 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.895638629283489, "precision": 0.9519867549668874, "recall": 0.8455882352941176, "support": 680}, "\u0027": {"f1-score": 0.22222222222222224, "precision": 0.21951219512195122, "recall": 0.225, "support": 40}, "macro avg": {"f1-score": 0.2297556593580919, "precision": 0.2298053448960641, "recall": 0.23030905110344177, "support": 19449}, "micro avg": {"f1-score": 0.9534680446295438, "precision": 0.9534680446295439, "recall": 0.9534680446295439, "support": 19449}, "weighted avg": {"f1-score": 0.9480978399783847, "precision": 0.9433927048862796, "recall": 0.9534680446295439, "support": 19449}, "\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u21e5\u21e5\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u21e5\u21e5\u21e5\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2205": {"f1-score": 0.978416028893284, "precision": 0.980777519179381, "recall": 0.9760658831603328, "support": 11657}, "\u23ce": {"f1-score": 0.8607449856733524, "precision": 0.8270925110132159, "recall": 0.8972520908004779, "support": 837}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 28}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 53}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 66}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 21}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 53}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u2423": {"f1-score": 0.9488243430152143, "precision": 0.927321882951654, "recall": 0.9713476595035816, "support": 6003}, "\u2423\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423\u2423\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\"": {"f1-score": 0.4203216374269006, "precision": 0.9519867549668874, "recall": 0.2696998123827392, "support": 2132}, "\u0027": {"f1-score": 0.03428571428571429, "precision": 0.21951219512195122, "recall": 0.01859504132231405, "support": 484}, "macro avg": {"f1-score": 0.17602190273767582, "precision": 0.2298053448960641, "recall": 0.15741973629950473, "support": 24114}, "micro avg": {"f1-score": 0.8513646902187637, "precision": 0.9534680446295439, "recall": 0.7690138508750104, "support": 24114}, "weighted avg": {"f1-score": 0.8045631298652436, "precision": 0.8805567205424363, "recall": 0.7690138508750104, "support": 24114}, "\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u21e5\u21e5\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u21e5\u21e5\u21e5\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2205": {"f1-score": 0.965956363018932, "precision": 0.980777519179381, "recall": 0.9515764823952496, "support": 11957}, "\u23ce": {"f1-score": 0.6799456767768222, "precision": 0.8270925110132159, "recall": 0.5772482705611068, "support": 1301}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 90}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 83}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 190}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 489}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 59}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 474}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 56}, "\u2423": {"f1-score": 0.89186295503212, "precision": 0.927321882951654, "recall": 0.8590159104301709, "support": 6788}, "\u2423\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423\u2423\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "ppcr": 0.8065439163971138
}
```
</details>
