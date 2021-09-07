# Train report for javascript / file:///tmp/top-repos-quality-repos-sor36cv6/udemy_nodejs.git HEAD 33d78358af80afa88c4c619ff57d59447af09015

### Classification report

PPCR: 0.465

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.983| 1.000| 0.583| 0.991| 0.732| 2937| 5032| 0.584 |
| `'` | 1.000| 1.000| 0.990| 1.000| 0.995| 807| 815| 0.990 |
| `␣` | 1.000| 0.931| 0.127| 0.964| 0.226| 347| 2536| 0.137 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.961| 0.967| 0.864| 0.964| 0.910| 329| 368| 0.894 |
| `⏎` | 0.932| 0.854| 0.320| 0.891| 0.476| 192| 513| 0.374 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 383| 0.026 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 303| 0.003 |
| `micro avg` | 0.984| 0.984| 0.457| 0.984| 0.624| 4623| 9950| 0.465 |
| `macro avg` | 0.697| 0.679| 0.412| 0.687| 0.477| 4623| 9950| 0.465 |
| `weighted avg` | 0.981| 0.984| 0.457| 0.982| 0.568| 4623| 9950| 0.465 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|2095 |2936 |0 |0 |0 |0 |1 |0 |
|2189 |1 |323 |0 |12 |0 |11 |0 |
|8 |0 |0 |807 |0 |0 |0 |0 |
|321 |28 |0 |0 |164 |0 |0 |0 |
|373 |10 |0 |0 |0 |0 |0 |0 |
|39 |11 |0 |0 |0 |0 |318 |0 |
|302 |0 |0 |0 |0 |0 |1 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| resources/node-course-v3-code-master/task-manager/tests/user.test.js | 31 |
| resources/node-course-v3-code-master/task-manager/tests/task.test.js | 13 |
| resources/node-course-v3-code-master/task-manager/src/routers/task.js | 6 |
| resources/node-course-v3-code-master/task-manager/src/routers/user.js | 3 |
| weather-app/utils/forecast.js | 3 |
| resources/node-course-v3-code-master/web-server/src/utils/geocode.js | 2 |
| resources/node-course-v3-code-master/weather-app/utils/forecast.js | 2 |
| resources/node-course-v3-code-master/playground/8-promises.js | 2 |
| resources/node-course-v3-code-master/task-manager/src/middleware/auth.js | 2 |
| resources/node-course-v3-code-master/web-server/src/utils/forecast.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 807}, "macro avg": {"f1-score": 0.6872156163578541, "precision": 0.6965426354625295, "recall": 0.6787467525135007, "support": 4623}, "micro avg": {"f1-score": 0.9837767683322518, "precision": 0.9837767683322518, "recall": 0.9837767683322518, "support": 4623}, "weighted avg": {"f1-score": 0.9823594862552159, "precision": 0.9813558590535113, "recall": 0.9837767683322518, "support": 4623}, "\u2205": {"f1-score": 0.9913894985649163, "precision": 0.9832551908908238, "recall": 0.9996595165134491, "support": 2937}, "\u23ce": {"f1-score": 0.8913043478260869, "precision": 0.9318181818181818, "recall": 0.8541666666666666, "support": 192}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9636363636363636, "precision": 0.9607250755287009, "recall": 0.9665653495440729, "support": 329}, "\u2423": {"f1-score": 0.9641791044776119, "precision": 1.0, "recall": 0.930835734870317, "support": 347}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9950678175092478, "precision": 1.0, "recall": 0.9901840490797545, "support": 815}, "macro avg": {"f1-score": 0.4770423785399777, "precision": 0.6965426354625295, "recall": 0.412119191769066, "support": 9950}, "micro avg": {"f1-score": 0.6241679818843067, "precision": 0.9837767683322518, "recall": 0.4570854271356784, "support": 9950}, "weighted avg": {"f1-score": 0.5676624261834874, "precision": 0.9176190628773785, "recall": 0.4570854271356784, "support": 9950}, "\u2205": {"f1-score": 0.7323522075330506, "precision": 0.9832551908908238, "recall": 0.5834658187599364, "support": 5032}, "\u23ce": {"f1-score": 0.47605224963715526, "precision": 0.9318181818181818, "recall": 0.31968810916179335, "support": 513}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 303}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 383}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9098712446351931, "precision": 0.9607250755287009, "recall": 0.8641304347826086, "support": 368}, "\u2423": {"f1-score": 0.22595313046519766, "precision": 1.0, "recall": 0.1273659305993691, "support": 2536}},
  "ppcr": 0.46462311557788943
}
```
</details>
