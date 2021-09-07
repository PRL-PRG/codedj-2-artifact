# Train report for javascript / file:///tmp/top-repos-quality-repos-vhcdciu9/advancedreact.git HEAD 636dd6fdc1019e76ca672f40c5b10ee4b257b8cf

### Classification report

PPCR: 0.810

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.988| 0.994| 0.913| 0.991| 0.949| 30993| 33762| 0.918 |
| `␣` | 0.970| 0.985| 0.796| 0.977| 0.874| 12594| 15578| 0.808 |
| `'` | 1.000| 1.000| 0.734| 1.000| 0.847| 2729| 3718| 0.734 |
| `⏎␣⁻␣⁻` | 1.000| 0.870| 0.679| 0.930| 0.809| 1358| 1739| 0.781 |
| `⏎` | 0.947| 0.920| 0.294| 0.933| 0.448| 1240| 3884| 0.319 |
| `"` | 1.000| 1.000| 0.979| 1.000| 0.990| 758| 774| 0.979 |
| `⏎␣⁺␣⁺` | 0.933| 0.504| 0.103| 0.655| 0.186| 359| 1757| 0.204 |
| `⏎⏎` | 1.000| 0.854| 0.199| 0.921| 0.333| 185| 792| 0.234 |
| `micro avg` | 0.983| 0.983| 0.796| 0.983| 0.880| 50216| 62004| 0.810 |
| `weighted avg` | 0.983| 0.983| 0.796| 0.983| 0.860| 50216| 62004| 0.810 |
| `macro avg` | 0.980| 0.891| 0.587| 0.926| 0.679| 50216| 62004| 0.810 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2769 |30821 |172 |0 |0 |0 |0 |0 |0 |
|2984 |151 |12402 |28 |0 |13 |0 |0 |0 |
|2644 |51 |48 |1141 |0 |0 |0 |0 |0 |
|989 |0 |0 |0 |2729 |0 |0 |0 |0 |
|1398 |64 |114 |0 |0 |181 |0 |0 |0 |
|381 |103 |53 |21 |0 |0 |1181 |0 |0 |
|607 |12 |0 |15 |0 |0 |0 |158 |0 |
|16 |0 |0 |0 |0 |0 |0 |0 |758 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| finished-application/frontend/components/CreateItem.js | 21 |
| stepped-solutions/19/frontend/components/CreateItem.js | 21 |
| finished-application/frontend/__tests__/CreateItem.test.js | 20 |
| stepped-solutions/20/frontend/components/UpdateItem.js | 18 |
| stepped-solutions/52/backend/src/resolvers/Mutation.js | 18 |
| finished-application/backend/src/resolvers/Mutation.js | 18 |
| finished-application/frontend/components/UpdateItem.js | 18 |
| stepped-solutions/18/frontend/components/CreateItem.js | 16 |
| stepped-solutions/51/backend/src/resolvers/Mutation.js | 16 |
| finished-application/frontend/components/Signup.js | 13 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 758}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 2729}, "macro avg": {"f1-score": 0.9259766157547248, "precision": 0.9796758152239708, "recall": 0.8909074860941694, "support": 50216}, "micro avg": {"f1-score": 0.9831726939620838, "precision": 0.9831726939620838, "recall": 0.9831726939620838, "support": 50216}, "weighted avg": {"f1-score": 0.9825007905702448, "precision": 0.9830838218865627, "recall": 0.9831726939620838, "support": 50216}, "\u2205": {"f1-score": 0.9911086100168824, "precision": 0.9877892442792129, "recall": 0.9944503597586551, "support": 30993}, "\u23ce": {"f1-score": 0.9333333333333333, "precision": 0.9468879668049792, "recall": 0.9201612903225806, "support": 1240}, "\u23ce\u23ce": {"f1-score": 0.9212827988338192, "precision": 1.0, "recall": 0.8540540540540541, "support": 185}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.6546112115732369, "precision": 0.9329896907216495, "recall": 0.5041782729805014, "support": 359}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9302875147695944, "precision": 1.0, "recall": 0.8696612665684831, "support": 1358}, "\u2423": {"f1-score": 0.9771894575109324, "precision": 0.9697396199859254, "recall": 0.9847546450690805, "support": 12594}},
  "cl_report_full": {"\"": {"f1-score": 0.989556135770235, "precision": 1.0, "recall": 0.979328165374677, "support": 774}, "\u0027": {"f1-score": 0.8465953156506902, "precision": 1.0, "recall": 0.733996772458311, "support": 3718}, "macro avg": {"f1-score": 0.6793639328063783, "precision": 0.9796758152239708, "recall": 0.5872180757480616, "support": 62004}, "micro avg": {"f1-score": 0.8798966316164676, "precision": 0.9831726939620838, "recall": 0.7962550803173989, "support": 62004}, "weighted avg": {"f1-score": 0.8597548232653045, "precision": 0.9805225471770209, "recall": 0.7962550803173989, "support": 62004}, "\u2205": {"f1-score": 0.9488639862077458, "precision": 0.9877892442792129, "recall": 0.9128902316213494, "support": 33762}, "\u23ce": {"f1-score": 0.4484181568088033, "precision": 0.9468879668049792, "recall": 0.29376930998970135, "support": 3884}, "\u23ce\u23ce": {"f1-score": 0.33263157894736844, "precision": 1.0, "recall": 0.1994949494949495, "support": 792}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.18554587391081498, "precision": 0.9329896907216495, "recall": 0.10301650540694365, "support": 1757}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.808904109589041, "precision": 1.0, "recall": 0.6791259344450834, "support": 1739}, "\u2423": {"f1-score": 0.8743963055663271, "precision": 0.9697396199859254, "recall": 0.796122737193478, "support": 15578}},
  "ppcr": 0.8098832333397845
}
```
</details>
