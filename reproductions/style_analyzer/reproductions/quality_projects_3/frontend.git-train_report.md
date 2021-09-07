# Train report for javascript / file:///tmp/top-repos-quality-repos-l3up5jui/frontend.git HEAD d5935119454a058ff243d57e85ac3f9d4c39719a

### Classification report

PPCR: 0.357

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.951| 1.000| 0.632| 0.975| 0.759| 4247| 6719| 0.632 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 153| 3125| 0.049 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 48| 207| 0.232 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 11| 727| 0.015 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 211| 0.033 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 874| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 316| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 149| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 171| 0.000 |
| `macro avg` | 0.106| 0.111| 0.070| 0.108| 0.084| 4466| 12499| 0.357 |
| `weighted avg` | 0.904| 0.951| 0.340| 0.927| 0.408| 4466| 12499| 0.357 |
| `micro avg` | 0.951| 0.951| 0.340| 0.951| 0.501| 4466| 12499| 0.357 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| '| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2472 |4247 |0 |0 |0 |0 |0 |0 |0 |0 |
|2972 |153 |0 |0 |0 |0 |0 |0 |0 |0 |
|874 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|716 |11 |0 |0 |0 |0 |0 |0 |0 |0 |
|204 |7 |0 |0 |0 |0 |0 |0 |0 |0 |
|159 |48 |0 |0 |0 |0 |0 |0 |0 |0 |
|316 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|149 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|171 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/containers/EventPage.js | 88 |
| src/components/EventCard.js | 26 |
| src/actions/specificEventActions.js | 22 |
| src/components/UpdateEventForm.js | 13 |
| src/actions/generalEventsActions.js | 9 |
| src/components/AddEventForm.js | 8 |
| src/components/Guests.js | 7 |
| src/components/EventList.js | 6 |
| src/reducers/generalEventsReducer.js | 6 |
| src/containers/SignUpPage.js | 5 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.10831834933751609, "precision": 0.10566253669701946, "recall": 0.1111111111111111, "support": 4466}, "micro avg": {"f1-score": 0.9509628302731751, "precision": 0.9509628302731751, "recall": 0.9509628302731751, "support": 4466}, "weighted avg": {"f1-score": 0.9270605165087051, "precision": 0.9043303045611676, "recall": 0.9509628302731751, "support": 4466}, "\u2205": {"f1-score": 0.9748651440376448, "precision": 0.9509628302731751, "recall": 1.0, "support": 4247}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 48}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 153}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 874}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 316}, "macro avg": {"f1-score": 0.0843788804450405, "precision": 0.10566253669701946, "recall": 0.07023201203882852, "support": 12499}, "micro avg": {"f1-score": 0.5006778661951076, "precision": 0.9509628302731751, "recall": 0.33978718297463795, "support": 12499}, "weighted avg": {"f1-score": 0.4082306808058279, "precision": 0.511202436723375, "recall": 0.33978718297463795, "support": 12499}, "\u2205": {"f1-score": 0.7594099240053644, "precision": 0.9509628302731751, "recall": 0.6320881083494567, "support": 6719}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 727}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 149}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 211}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 171}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 207}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3125}},
  "ppcr": 0.35730858468677495
}
```
</details>
