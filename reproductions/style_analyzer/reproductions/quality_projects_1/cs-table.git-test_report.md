# Test report for javascript / file:///tmp/top-repos-quality-repos-7lawyjuc/cs-table.git HEAD 17226957035448b5b3ad84f54d8dff261d473dbb

### Classification report

PPCR: 0.564

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.977| 1.000| 0.425| 0.988| 0.592| 85| 200| 0.425 |
| `'` | 0.928| 1.000| 1.000| 0.962| 0.962| 64| 64| 1.000 |
| `␣` | 1.000| 0.273| 0.164| 0.429| 0.281| 33| 55| 0.600 |
| `⏎␣⁺␣⁺` | 0.857| 0.857| 0.857| 0.857| 0.857| 7| 7| 1.000 |
| `⏎` | 0.286| 1.000| 0.300| 0.444| 0.293| 6| 20| 0.300 |
| `⏎␣⁻␣⁻` | 0.500| 0.750| 0.429| 0.600| 0.462| 4| 7| 0.571 |
| `macro avg` | 0.758| 0.813| 0.529| 0.713| 0.575| 199| 353| 0.564 |
| `micro avg` | 0.869| 0.869| 0.490| 0.869| 0.627| 199| 353| 0.564 |
| `weighted avg` | 0.930| 0.869| 0.490| 0.858| 0.597| 199| 353| 0.564 |

### Confusion matrix

|refusal|  ∅| '| ␣| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|115 |85 |0 |0 |0 |0 |0 |
|0 |0 |64 |0 |0 |0 |0 |
|22 |1 |5 |9 |14 |1 |3 |
|14 |0 |0 |0 |6 |0 |0 |
|0 |1 |0 |0 |0 |6 |0 |
|3 |0 |0 |0 |1 |0 |3 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.9624060150375939, "precision": 0.927536231884058, "recall": 1.0, "support": 64}, "macro avg": {"f1-score": 0.7134894730365966, "precision": 0.7579008114990123, "recall": 0.8133116883116882, "support": 199}, "micro avg": {"f1-score": 0.8693467336683417, "precision": 0.8693467336683417, "recall": 0.8693467336683417, "support": 199}, "weighted avg": {"f1-score": 0.8583675209995304, "precision": 0.9302642289766818, "recall": 0.8693467336683417, "support": 199}, "\u2205": {"f1-score": 0.9883720930232558, "precision": 0.9770114942528736, "recall": 1.0, "support": 85}, "\u23ce": {"f1-score": 0.4444444444444445, "precision": 0.2857142857142857, "recall": 1.0, "support": 6}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8571428571428571, "precision": 0.8571428571428571, "recall": 0.8571428571428571, "support": 7}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.6, "precision": 0.5, "recall": 0.75, "support": 4}, "\u2423": {"f1-score": 0.42857142857142855, "precision": 1.0, "recall": 0.2727272727272727, "support": 33}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9624060150375939, "precision": 0.927536231884058, "recall": 1.0, "support": 64}, "macro avg": {"f1-score": 0.57455912588695, "precision": 0.7579008114990123, "recall": 0.5290584415584415, "support": 353}, "micro avg": {"f1-score": 0.6268115942028986, "precision": 0.8693467336683417, "recall": 0.49008498583569404, "support": 353}, "weighted avg": {"f1-score": 0.5966404013724205, "precision": 0.9206201229615868, "recall": 0.49008498583569404, "support": 353}, "\u2205": {"f1-score": 0.5923344947735192, "precision": 0.9770114942528736, "recall": 0.425, "support": 200}, "\u23ce": {"f1-score": 0.2926829268292683, "precision": 0.2857142857142857, "recall": 0.3, "support": 20}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8571428571428571, "precision": 0.8571428571428571, "recall": 0.8571428571428571, "support": 7}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.4615384615384615, "precision": 0.5, "recall": 0.42857142857142855, "support": 7}, "\u2423": {"f1-score": 0.28125, "precision": 1.0, "recall": 0.16363636363636364, "support": 55}},
  "ppcr": 0.5637393767705382
}
```
</details>
