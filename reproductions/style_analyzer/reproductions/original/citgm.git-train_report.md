# Train report for javascript / file:///tmp/top-repos-quality-repos-yk1pyslq/citgm.git HEAD 5fd10b4af59f91c2640dc9982811385fc991e2cc

### Classification report

PPCR: 0.835

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.960| 0.987| 0.963| 0.973| 0.961| 14137| 14486| 0.976 |
| `␣` | 0.946| 0.972| 0.749| 0.959| 0.836| 5244| 6805| 0.771 |
| `'` | 1.000| 1.000| 0.971| 1.000| 0.985| 3669| 3780| 0.971 |
| `⏎␣⁺␣⁺` | 0.963| 0.839| 0.800| 0.897| 0.874| 958| 1005| 0.953 |
| `⏎␣⁻␣⁻` | 1.000| 0.646| 0.492| 0.785| 0.660| 785| 1030| 0.762 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 141| 2147| 0.066 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 600| 0.002 |
| `weighted avg` | 0.959| 0.964| 0.805| 0.960| 0.834| 24935| 29853| 0.835 |
| `macro avg` | 0.695| 0.635| 0.568| 0.659| 0.617| 24935| 29853| 0.835 |
| `micro avg` | 0.964| 0.964| 0.805| 0.964| 0.877| 24935| 29853| 0.835 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|349 |13949 |188 |0 |0 |0 |0 |0 |
|1561 |115 |5098 |0 |0 |31 |0 |0 |
|111 |0 |0 |3669 |0 |0 |0 |0 |
|2006 |54 |87 |0 |0 |0 |0 |0 |
|47 |136 |18 |0 |0 |804 |0 |0 |
|245 |278 |0 |0 |0 |0 |507 |0 |
|599 |1 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| test/test-lookup.js | 98 |
| test/reporter/test-reporter-util.js | 52 |
| lib/lookup.js | 47 |
| bin/citgm-all.js | 42 |
| lib/common-args.js | 39 |
| test/test-check-tags.js | 38 |
| test/npm/test-npm-test.js | 35 |
| test/yarn/test-yarn-test.js | 33 |
| test/test-grab-project.js | 27 |
| test/test-grab-module-data.js | 27 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 3669}, "macro avg": {"f1-score": 0.659063541982302, "precision": 0.6954771430382761, "recall": 0.6348526468052126, "support": 24935}, "micro avg": {"f1-score": 0.9635853218367756, "precision": 0.9635853218367756, "recall": 0.9635853218367756, "support": 24935}, "weighted avg": {"f1-score": 0.9596194657543928, "precision": 0.958665970935822, "recall": 0.9635853218367756, "support": 24935}, "\u2205": {"f1-score": 0.9730728985001744, "precision": 0.9598155921007363, "recall": 0.986701563273679, "support": 14137}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 141}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8968209704406023, "precision": 0.962874251497006, "recall": 0.8392484342379958, "support": 958}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7848297213622291, "precision": 1.0, "recall": 0.6458598726114649, "support": 785}, "\u2423": {"f1-score": 0.9587212035731076, "precision": 0.945650157670191, "recall": 0.9721586575133486, "support": 5244}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9850986709625452, "precision": 1.0, "recall": 0.9706349206349206, "support": 3780}, "macro avg": {"f1-score": 0.6165886284001421, "precision": 0.6954771430382761, "recall": 0.5678503840942214, "support": 29853}, "micro avg": {"f1-score": 0.8770898736949696, "precision": 0.9635853218367756, "recall": 0.8048437342980604, "support": 29853}, "weighted avg": {"f1-score": 0.8339846004418171, "precision": 0.8748442907872376, "recall": 0.8048437342980604, "support": 29853}, "\u2205": {"f1-score": 0.9613701368069197, "precision": 0.9598155921007363, "recall": 0.9629297252519674, "support": 14486}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2147}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 600}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8739130434782609, "precision": 0.962874251497006, "recall": 0.8, "support": 1005}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.6597267404033832, "precision": 1.0, "recall": 0.49223300970873785, "support": 1030}, "\u2423": {"f1-score": 0.8360118071498851, "precision": 0.945650157670191, "recall": 0.7491550330639236, "support": 6805}},
  "ppcr": 0.8352594379124376
}
```
</details>
