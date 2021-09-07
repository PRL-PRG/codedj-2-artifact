# Test report for javascript / file:///tmp/top-repos-quality-repos-8_pugp2s/matt-peet-illustration.git HEAD aab58c9d78342452aac4819e0b72474ce8b45e0a

### Classification report

PPCR: 0.543

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.736| 1.000| 0.880| 0.848| 0.801| 117| 133| 0.880 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 33| 118| 0.280 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 10| 0.400 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 7| 0.429 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 18| 0.056 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 7| 0.143 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `macro avg` | 0.105| 0.143| 0.126| 0.121| 0.114| 159| 293| 0.543 |
| `micro avg` | 0.736| 0.736| 0.399| 0.736| 0.518| 159| 293| 0.543 |
| `weighted avg` | 0.541| 0.736| 0.399| 0.624| 0.364| 159| 293| 0.543 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| '| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|16 |117 |0 |0 |0 |0 |0 |
|85 |33 |0 |0 |0 |0 |0 |
|6 |4 |0 |0 |0 |0 |0 |
|17 |1 |0 |0 |0 |0 |0 |
|6 |1 |0 |0 |0 |0 |0 |
|4 |3 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "macro avg": {"f1-score": 0.12111801242236023, "precision": 0.10512129380053907, "recall": 0.14285714285714285, "support": 159}, "micro avg": {"f1-score": 0.7358490566037735, "precision": 0.7358490566037735, "recall": 0.7358490566037735, "support": 159}, "weighted avg": {"f1-score": 0.6238720262510253, "precision": 0.5414738341046635, "recall": 0.7358490566037735, "support": 159}, "\u2205": {"f1-score": 0.8478260869565216, "precision": 0.7358490566037735, "recall": 1.0, "support": 117}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 33}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 18}, "macro avg": {"f1-score": 0.11448140900195694, "precision": 0.10512129380053907, "recall": 0.12567132116004295, "support": 293}, "micro avg": {"f1-score": 0.5176991150442478, "precision": 0.7358490566037735, "recall": 0.3993174061433447, "support": 293}, "weighted avg": {"f1-score": 0.36376174669222494, "precision": 0.33402022023311223, "recall": 0.3993174061433447, "support": 293}, "\u2205": {"f1-score": 0.8013698630136986, "precision": 0.7358490566037735, "recall": 0.8796992481203008, "support": 133}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 118}},
  "ppcr": 0.5426621160409556
}
```
</details>
