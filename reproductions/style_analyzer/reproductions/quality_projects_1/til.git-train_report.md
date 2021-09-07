# Train report for javascript / file:///tmp/top-repos-quality-repos-7ykqbg8c/til.git HEAD 50893560b6c8f3366d2d039ab9fb9ac5db678a85

### Classification report

PPCR: 0.666

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.969| 0.990| 0.935| 0.980| 0.952| 18061| 19141| 0.944 |
| `␣` | 0.957| 0.899| 0.469| 0.927| 0.629| 4789| 9184| 0.521 |
| `'` | 1.000| 1.000| 0.518| 1.000| 0.683| 617| 1191| 0.518 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 34| 3044| 0.011 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 23| 220| 0.105 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 17| 213| 0.080 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 12| 205| 0.059 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 207| 0.034 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 1585| 0.003 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 214| 0.014 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 199| 0.015 |
| `macro avg` | 0.266| 0.263| 0.175| 0.264| 0.206| 23570| 35403| 0.666 |
| `micro avg` | 0.968| 0.968| 0.644| 0.968| 0.774| 23570| 35403| 0.666 |
| `weighted avg` | 0.963| 0.968| 0.644| 0.965| 0.701| 23570| 35403| 0.666 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| ⏎⏎| '| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⇥⁺| ⏎␣⁺␣⁺| ⏎⇥⁻| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1080 |17888 |173 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|4395 |483 |4306 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3010 |33 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1581 |2 |2 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|574 |0 |0 |0 |0 |617 |0 |0 |0 |0 |0 |0 |
|197 |15 |8 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|211 |3 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|196 |11 |6 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|200 |4 |3 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|193 |12 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|196 |2 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| react/advanced-react-hooks/src/App.js | 27 |
| react/advanced-react-hooks/src/serviceWorker.js | 20 |
| javascript/toy-problems/arrray_difference.js | 20 |
| javascript/toy-problems/tribonacci_sequence.js | 20 |
| javascript/toy-problems/batches.js | 16 |
| javascript/toy-problems/recursive_nested_even_sum.js | 15 |
| javascript/toy-problems/unique_more_code.js | 14 |
| javascript/toy-problems/nth-smallest-element.js | 14 |
| javascript/toy-problems/unique_sum.js | 13 |
| javascript/toy-problems/reverse-on-diagonals.js | 13 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 617}, "macro avg": {"f1-score": 0.26426424281081634, "precision": 0.26602459647688065, "recall": 0.2626877473856163, "support": 23570}, "micro avg": {"f1-score": 0.9677980483665677, "precision": 0.9677980483665677, "recall": 0.9677980483665677, "support": 23570}, "weighted avg": {"f1-score": 0.9653343234797287, "precision": 0.9634087090931278, "recall": 0.9677980483665677, "support": 23570}, "\u2205": {"f1-score": 0.9797885742454949, "precision": 0.9693816723567984, "recall": 0.9904213498698854, "support": 18061}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 34}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 17}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 23}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u2423": {"f1-score": 0.9271180966734848, "precision": 0.9568888888888889, "recall": 0.8991438713718939, "support": 4789}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6825221238938053, "precision": 1.0, "recall": 0.5180520570948782, "support": 1191}, "macro avg": {"f1-score": 0.20577377155044682, "precision": 0.26602459647688065, "recall": 0.17467721522562957, "support": 35403}, "micro avg": {"f1-score": 0.7736082614077627, "precision": 0.9677980483665677, "recall": 0.6443239273507895, "support": 35403}, "weighted avg": {"f1-score": 0.7007367110649522, "precision": 0.8059769552336535, "recall": 0.6443239273507895, "support": 35403}, "\u2205": {"f1-score": 0.9516412193435123, "precision": 0.9693816723567984, "recall": 0.9345384253696254, "support": 19141}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3044}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 213}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 205}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1585}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 207}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 220}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 199}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 214}, "\u2423": {"f1-score": 0.6293481438175973, "precision": 0.9568888888888889, "recall": 0.4688588850174216, "support": 9184}},
  "ppcr": 0.6657627884642544
}
```
</details>
