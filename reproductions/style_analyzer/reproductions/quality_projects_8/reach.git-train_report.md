# Train report for javascript / file:///tmp/top-repos-quality-repos-b3hsmgrz/reach.git HEAD 036fde6c2a83f5b014b17b423c72d79730e43a8c

### Classification report

PPCR: 0.495

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.946| 0.996| 0.852| 0.970| 0.896| 2076| 2427| 0.855 |
| `␣` | 0.979| 0.898| 0.220| 0.936| 0.360| 509| 2075| 0.245 |
| `'` | 1.000| 1.000| 0.475| 1.000| 0.644| 150| 316| 0.475 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 41| 202| 0.203 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 25| 291| 0.086 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 241| 0.008 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 107| 0.000 |
| `weighted avg` | 0.932| 0.954| 0.473| 0.942| 0.552| 2803| 5659| 0.495 |
| `micro avg` | 0.954| 0.954| 0.473| 0.954| 0.632| 2803| 5659| 0.495 |
| `macro avg` | 0.418| 0.413| 0.221| 0.415| 0.271| 2803| 5659| 0.495 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|351 |2067 |9 |0 |0 |0 |0 |0 |
|1566 |52 |457 |0 |0 |0 |0 |0 |
|166 |0 |0 |150 |0 |0 |0 |0 |
|266 |24 |1 |0 |0 |0 |0 |0 |
|239 |2 |0 |0 |0 |0 |0 |0 |
|161 |41 |0 |0 |0 |0 |0 |0 |
|107 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/serviceWorker.js | 30 |
| src/store/project/reducers.js | 22 |
| src/store/project/actions.js | 17 |
| src/store/resource/actions.js | 9 |
| src/utils/ScheduleConfig.js | 8 |
| src/store/attachment/actions.js | 7 |
| src/store/resource/reducers.js | 5 |
| src/store/account/actions.js | 5 |
| src/store/account/reducers.js | 4 |
| src/store/index.js | 4 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 150}, "macro avg": {"f1-score": 0.41520608020115674, "precision": 0.41773562790213425, "recall": 0.4133576628125613, "support": 2803}, "micro avg": {"f1-score": 0.953977880841955, "precision": 0.953977880841955, "recall": 0.953977880841955, "support": 2803}, "weighted avg": {"f1-score": 0.9419613950303587, "precision": 0.9315336241627559, "recall": 0.953977880841955, "support": 2803}, "\u2205": {"f1-score": 0.9699671515720318, "precision": 0.9455626715462031, "recall": 0.9956647398843931, "support": 2076}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 25}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 41}, "\u2423": {"f1-score": 0.9364754098360655, "precision": 0.9785867237687366, "recall": 0.8978388998035364, "support": 509}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6437768240343348, "precision": 1.0, "recall": 0.47468354430379744, "support": 316}, "macro avg": {"f1-score": 0.2713570348055772, "precision": 0.41773562790213425, "recall": 0.22094189071177966, "support": 5659}, "micro avg": {"f1-score": 0.632001890805956, "precision": 0.953977880841955, "recall": 0.47252164693408727, "support": 5659}, "weighted avg": {"f1-score": 0.5521305671064513, "precision": 0.8201887357594563, "recall": 0.47252164693408727, "support": 5659}, "\u2205": {"f1-score": 0.8961630175590722, "precision": 0.9455626715462031, "recall": 0.8516687268232386, "support": 2427}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 291}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 107}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 241}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 202}, "\u2423": {"f1-score": 0.3595594020456334, "precision": 0.9785867237687366, "recall": 0.2202409638554217, "support": 2075}},
  "ppcr": 0.4953171938505036
}
```
</details>
