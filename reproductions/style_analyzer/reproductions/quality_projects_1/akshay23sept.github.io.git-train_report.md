# Train report for javascript / file:///tmp/top-repos-quality-repos-3kibs5bn/akshay23sept.github.io.git HEAD a5dedc1a3efe295a5c8b01433a6d41f32c82221b

### Classification report

PPCR: 0.142

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `⏎` | 0.984| 1.000| 0.926| 0.992| 0.955| 378| 408| 0.926 |
| `∅` | 1.000| 0.979| 0.080| 0.989| 0.149| 190| 2316| 0.082 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 938| 0.002 |
| `␣⇥` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 124| 0.000 |
| `␣⇥⇥` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 112| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 104| 0.000 |
| `macro avg` | 0.331| 0.330| 0.168| 0.330| 0.184| 570| 4002| 0.142 |
| `micro avg` | 0.989| 0.989| 0.141| 0.989| 0.247| 570| 4002| 0.142 |
| `weighted avg` | 0.986| 0.989| 0.141| 0.988| 0.183| 570| 4002| 0.142 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| ␣⇥| ␣⇥⇥| '| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|2126 |186 |0 |4 |0 |0 |0 |
|936 |0 |0 |2 |0 |0 |0 |
|30 |0 |0 |378 |0 |0 |0 |
|124 |0 |0 |0 |0 |0 |0 |
|112 |0 |0 |0 |0 |0 |0 |
|104 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| BackUp/index_files/dataRefs.bundle.js | 3 |
| backUp/Index_Files1/dataRefs.bundle.js | 3 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.33024794772993804, "precision": 0.3307291666666667, "recall": 0.32982456140350874, "support": 570}, "micro avg": {"f1-score": 0.9894736842105263, "precision": 0.9894736842105263, "recall": 0.9894736842105263, "support": 570}, "weighted avg": {"f1-score": 0.987723413072806, "precision": 0.9861293859649123, "recall": 0.9894736842105263, "support": 570}, "\u2205": {"f1-score": 0.9893617021276596, "precision": 1.0, "recall": 0.9789473684210527, "support": 190}, "\u23ce": {"f1-score": 0.9921259842519685, "precision": 0.984375, "recall": 1.0, "support": 378}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u2423\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423\u21e5\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 104}, "macro avg": {"f1-score": 0.18387108495022164, "precision": 0.3307291666666667, "recall": 0.1677969115107183, "support": 4002}, "micro avg": {"f1-score": 0.24671916010498687, "precision": 0.9894736842105263, "recall": 0.1409295352323838, "support": 4002}, "weighted avg": {"f1-score": 0.18335828815481076, "precision": 0.6790667166416792, "recall": 0.1409295352323838, "support": 4002}, "\u2205": {"f1-score": 0.1486810551558753, "precision": 1.0, "recall": 0.08031088082901554, "support": 2316}, "\u23ce": {"f1-score": 0.9545454545454545, "precision": 0.984375, "recall": 0.9264705882352942, "support": 408}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 938}, "\u2423\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 124}, "\u2423\u21e5\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 112}},
  "ppcr": 0.1424287856071964
}
```
</details>
