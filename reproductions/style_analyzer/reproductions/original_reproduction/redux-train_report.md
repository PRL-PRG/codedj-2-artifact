# Train report for javascript / file:///tmp/top-repos-quality-repos-t6zjqnfc/redux HEAD 902484ed735d38aec06683c847810a7218d8dba2

### Classification report

PPCR: 0.915

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.959| 0.988| 0.960| 0.973| 0.959| 18711| 19263| 0.971 |
| `␣` | 0.944| 0.940| 0.841| 0.942| 0.889| 10030| 11213| 0.894 |
| `'` | 1.000| 1.000| 0.998| 1.000| 0.999| 2928| 2934| 0.998 |
| `⏎␣⁺␣⁺` | 0.877| 0.726| 0.606| 0.795| 0.717| 1361| 1631| 0.834 |
| `⏎` | 0.894| 0.795| 0.495| 0.842| 0.637| 1315| 2113| 0.622 |
| `⏎␣⁻␣⁻` | 0.886| 0.792| 0.699| 0.836| 0.782| 1292| 1463| 0.883 |
| `⏎⏎` | 0.862| 0.918| 0.564| 0.889| 0.682| 625| 1017| 0.615 |
| `"` | 1.000| 1.000| 1.000| 1.000| 1.000| 90| 90| 1.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 58| 77| 0.753 |
| `weighted avg` | 0.947| 0.949| 0.868| 0.948| 0.900| 36410| 39801| 0.915 |
| `macro avg` | 0.825| 0.796| 0.685| 0.809| 0.741| 36410| 39801| 0.915 |
| `micro avg` | 0.949| 0.949| 0.868| 0.949| 0.907| 36410| 39801| 0.915 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|552 |18491 |186 |0 |5 |4 |23 |2 |0 |0 |
|1183 |284 |9426 |0 |83 |134 |66 |37 |0 |0 |
|6 |0 |0 |2928 |0 |0 |0 |0 |0 |0 |
|798 |35 |180 |0 |1046 |0 |1 |53 |0 |0 |
|270 |214 |157 |0 |2 |988 |0 |0 |0 |0 |
|171 |247 |18 |0 |4 |0 |1023 |0 |0 |0 |
|392 |2 |20 |0 |29 |0 |0 |574 |0 |0 |
|0 |0 |0 |0 |0 |0 |0 |0 |90 |0 |
|19 |16 |0 |0 |1 |0 |41 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| test/createStore.spec.js | 156 |
| test/combineReducers.spec.js | 96 |
| examples/todomvc/src/reducers/todos.spec.js | 79 |
| src/combineReducers.js | 70 |
| test/helpers/actionCreators.js | 46 |
| test/helpers/reducers.js | 37 |
| examples/todos-flow/src/__tests__/reducers/todos.test.js | 37 |
| src/createStore.js | 35 |
| test/bindActionCreators.spec.js | 35 |
| examples/real-world/src/actions/index.js | 34 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 90}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 2928}, "macro avg": {"f1-score": 0.8085659469450385, "precision": 0.824695475275138, "recall": 0.795510289738967, "support": 36410}, "micro avg": {"f1-score": 0.9493545729195276, "precision": 0.9493545729195276, "recall": 0.9493545729195276, "support": 36410}, "weighted avg": {"f1-score": 0.9475108350003268, "precision": 0.9468642243024116, "recall": 0.9493545729195276, "support": 36410}, "\u2205": {"f1-score": 0.9732105263157895, "precision": 0.9586292705687179, "recall": 0.9882422104644327, "support": 18711}, "\u23ce": {"f1-score": 0.841851106639839, "precision": 0.8940170940170941, "recall": 0.7954372623574144, "support": 1315}, "\u23ce\u23ce": {"f1-score": 0.8892331525948877, "precision": 0.8618618618618619, "recall": 0.9184, "support": 625}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7945315641334942, "precision": 0.8774422735346359, "recall": 0.7259368111682586, "support": 1361}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8364677023712184, "precision": 0.8864818024263431, "recall": 0.791795665634675, "support": 1292}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 58}, "\u2423": {"f1-score": 0.9417994704501175, "precision": 0.9438269750675878, "recall": 0.9397806580259223, "support": 10030}},
  "cl_report_full": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 90}, "\u0027": {"f1-score": 0.9989764585465711, "precision": 1.0, "recall": 0.9979550102249489, "support": 2934}, "macro avg": {"f1-score": 0.7405963506824763, "precision": 0.824695475275138, "recall": 0.684772991074444, "support": 39801}, "micro avg": {"f1-score": 0.9071131463961895, "precision": 0.9493545729195276, "recall": 0.8684706414411698, "support": 39801}, "weighted avg": {"f1-score": 0.9000667580563356, "precision": 0.9438659925291936, "recall": 0.8684706414411698, "support": 39801}, "\u2205": {"f1-score": 0.9592757833575432, "precision": 0.9586292705687179, "recall": 0.959923168769143, "support": 19263}, "\u23ce": {"f1-score": 0.6372220530003047, "precision": 0.8940170940170941, "recall": 0.49503076194983436, "support": 2113}, "\u23ce\u23ce": {"f1-score": 0.6821152703505645, "precision": 0.8618618618618619, "recall": 0.5644051130776795, "support": 1017}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.716721073630758, "precision": 0.8774422735346359, "recall": 0.6057633353770693, "support": 1631}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7818112342376768, "precision": 0.8864818024263431, "recall": 0.6992481203007519, "support": 1463}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 77}, "\u2423": {"f1-score": 0.8892452830188678, "precision": 0.9438269750675878, "recall": 0.8406314099705698, "support": 11213}},
  "ppcr": 0.9148011356498581
}
```
</details>
