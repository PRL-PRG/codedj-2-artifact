# Train report for javascript / file:///tmp/top-repos-quality-repos-1qodi0vh/mainsite.git HEAD dd9498d5b6fe3d52df9009a1e55691d0544da1fa

### Classification report

PPCR: 0.583

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.972| 1.000| 0.835| 0.986| 0.898| 13485| 16146| 0.835 |
| `'` | 1.000| 1.000| 0.952| 1.000| 0.975| 1982| 2083| 0.952 |
| `␣` | 0.993| 0.776| 0.107| 0.871| 0.193| 1111| 8083| 0.137 |
| `⏎` | 0.966| 0.988| 0.271| 0.977| 0.423| 495| 1806| 0.274 |
| `"` | 1.000| 1.000| 0.724| 1.000| 0.840| 466| 644| 0.724 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 110| 549| 0.200 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 31| 555| 0.056 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 17| 469| 0.036 |
| `micro avg` | 0.977| 0.977| 0.570| 0.977| 0.720| 17697| 30335| 0.583 |
| `weighted avg` | 0.968| 0.977| 0.570| 0.971| 0.639| 17697| 30335| 0.583 |
| `macro avg` | 0.616| 0.595| 0.361| 0.604| 0.416| 17697| 30335| 0.583 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2661 |13483 |2 |0 |0 |0 |0 |0 |0 |
|6972 |249 |862 |0 |0 |0 |0 |0 |0 |
|101 |0 |0 |1982 |0 |0 |0 |0 |0 |
|1311 |5 |1 |0 |489 |0 |0 |0 |0 |
|178 |0 |0 |0 |0 |466 |0 |0 |0 |
|524 |28 |3 |0 |0 |0 |0 |0 |0 |
|439 |110 |0 |0 |0 |0 |0 |0 |0 |
|452 |0 |0 |0 |17 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/components/app/app.js | 18 |
| src/components/app/mobile-navigation/mobile-navigation-link/mobile-navigation-link.js | 16 |
| src/components/screens/demo/demo.js | 15 |
| src/components/screens/team/team-member/team-member-detail/team-member-detail.js | 12 |
| src/components/layout/progress-indicator-overlay/progress-indicator-overlay.js | 12 |
| src/store/contact-form/contact-form.js | 11 |
| src/components/screens/tax/tax-process/tax-process.js | 11 |
| src/components/screens/accounting/accounting-process/accounting-process.js | 11 |
| src/components/screens/wealth/wealth-process/wealth-process.js | 11 |
| src/components/layout/grid/column/column.js | 10 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 466}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1982}, "macro avg": {"f1-score": 0.6042211795236565, "precision": 0.6164048084258462, "recall": 0.5954510078371574, "support": 17697}, "micro avg": {"f1-score": 0.9765496976888738, "precision": 0.9765496976888738, "recall": 0.9765496976888738, "support": 17697}, "weighted avg": {"f1-score": 0.9713668335595183, "precision": 0.9681702107754684, "recall": 0.9765496976888738, "support": 17697}, "\u2205": {"f1-score": 0.9855994152046783, "precision": 0.9717477477477477, "recall": 0.999851687059696, "support": 13485}, "\u23ce": {"f1-score": 0.977022977022977, "precision": 0.9664031620553359, "recall": 0.9878787878787879, "support": 495}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 17}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 31}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 110}, "\u2423": {"f1-score": 0.8711470439615968, "precision": 0.9930875576036866, "recall": 0.7758775877587759, "support": 1111}},
  "cl_report_full": {"\"": {"f1-score": 0.8396396396396396, "precision": 1.0, "recall": 0.7236024844720497, "support": 644}, "\u0027": {"f1-score": 0.9751537515375154, "precision": 1.0, "recall": 0.9515122419587134, "support": 2083}, "macro avg": {"f1-score": 0.41608073128008294, "precision": 0.6164048084258462, "recall": 0.36094874099293583, "support": 30335}, "micro avg": {"f1-score": 0.719603597601599, "precision": 0.9765496976888738, "recall": 0.5697049612658646, "support": 30335}, "weighted avg": {"f1-score": 0.6393835058059183, "precision": 0.929266193305346, "recall": 0.5697049612658646, "support": 30335}, "\u2205": {"f1-score": 0.8982379001365711, "precision": 0.9717477477477477, "recall": 0.8350675089805525, "support": 16146}, "\u23ce": {"f1-score": 0.42301038062283736, "precision": 0.9664031620553359, "recall": 0.2707641196013289, "support": 1806}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 469}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 555}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 549}, "\u2423": {"f1-score": 0.1926041783041001, "precision": 0.9930875576036866, "recall": 0.1066435729308425, "support": 8083}},
  "ppcr": 0.5833855282676776
}
```
</details>
