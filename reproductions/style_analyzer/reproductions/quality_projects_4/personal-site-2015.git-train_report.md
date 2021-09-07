# Train report for javascript / file:///tmp/top-repos-quality-repos-eobax_tg/personal-site-2015.git HEAD dc9a2ee578e028bda8507665b04682115737b186

### Classification report

PPCR: 0.383

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.979| 0.999| 0.601| 0.989| 0.745| 1816| 3021| 0.601 |
| `'` | 1.000| 1.000| 0.500| 1.000| 0.667| 359| 718| 0.500 |
| `␣` | 0.946| 0.861| 0.136| 0.902| 0.238| 245| 1548| 0.158 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 472| 0.013 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 238| 0.017 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 108| 0.028 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 247| 0.008 |
| `macro avg` | 0.418| 0.409| 0.177| 0.413| 0.236| 2435| 6352| 0.383 |
| `weighted avg` | 0.973| 0.979| 0.375| 0.976| 0.488| 2435| 6352| 0.383 |
| `micro avg` | 0.979| 0.979| 0.375| 0.979| 0.543| 2435| 6352| 0.383 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1205 |1815 |1 |0 |0 |0 |0 |0 |
|1303 |34 |211 |0 |0 |0 |0 |0 |
|359 |0 |0 |359 |0 |0 |0 |0 |
|466 |0 |6 |0 |0 |0 |0 |0 |
|245 |2 |0 |0 |0 |0 |0 |0 |
|234 |2 |2 |0 |0 |0 |0 |0 |
|105 |0 |3 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/phsh.js | 10 |
| src/kml_new.js | 10 |
| src/github_tracker.js | 9 |
| src/filesys.js | 7 |
| src/term.js | 5 |
| src/visual.js | 4 |
| gulpfile.js | 3 |
| src/modules.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 359}, "macro avg": {"f1-score": 0.4130114003376473, "precision": 0.41795443647488206, "recall": 0.4086676898575667, "support": 2435}, "micro avg": {"f1-score": 0.9794661190965093, "precision": 0.9794661190965093, "recall": 0.9794661190965093, "support": 2435}, "weighted avg": {"f1-score": 0.976022772487352, "precision": 0.9731313811337285, "recall": 0.9794661190965093, "support": 2435}, "\u2205": {"f1-score": 0.9893704006541293, "precision": 0.9794927145169995, "recall": 0.9994493392070485, "support": 1816}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u2423": {"f1-score": 0.9017094017094017, "precision": 0.9461883408071748, "recall": 0.8612244897959184, "support": 245}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 718}, "macro avg": {"f1-score": 0.23567403998745864, "precision": 0.41795443647488206, "recall": 0.17672847835546154, "support": 6352}, "micro avg": {"f1-score": 0.5428473881870945, "precision": 0.9794661190965093, "recall": 0.37547229219143574, "support": 6352}, "weighted avg": {"f1-score": 0.48763760391570105, "precision": 0.8094689927779223, "recall": 0.37547229219143574, "support": 6352}, "\u2205": {"f1-score": 0.7447681575707839, "precision": 0.9794927145169995, "recall": 0.6007944389275075, "support": 3021}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 472}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 108}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 247}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 238}, "\u2423": {"f1-score": 0.23828345567476006, "precision": 0.9461883408071748, "recall": 0.13630490956072353, "support": 1548}},
  "ppcr": 0.38334382871536526
}
```
</details>
