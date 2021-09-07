# Test report for javascript / file:///tmp/top-repos-quality-repos-a7lhypiv/riftsketchedit.git HEAD e3e45f52f5b044dc8d2ebd3ce5c6bc6f88b6b0cf

### Classification report

PPCR: 0.670

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.915| 0.961| 0.804| 0.938| 0.856| 462| 552| 0.837 |
| `␣` | 0.867| 0.787| 0.404| 0.825| 0.551| 141| 275| 0.513 |
| `⏎␣⁻␣⁻` | 0.870| 0.727| 0.690| 0.792| 0.769| 55| 58| 0.948 |
| `⏎␣⁺␣⁺` | 0.809| 1.000| 0.667| 0.894| 0.731| 38| 57| 0.667 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 77| 0.078 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 20| 0.200 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 14| 0.000 |
| `macro avg` | 0.494| 0.497| 0.366| 0.493| 0.415| 706| 1053| 0.670 |
| `micro avg` | 0.897| 0.897| 0.601| 0.897| 0.720| 706| 1053| 0.670 |
| `weighted avg` | 0.884| 0.897| 0.601| 0.888| 0.675| 706| 1053| 0.670 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|90 |444 |10 |0 |0 |5 |3 |0 |
|134 |24 |111 |0 |0 |3 |3 |0 |
|71 |5 |0 |0 |0 |1 |0 |0 |
|16 |1 |3 |0 |0 |0 |0 |0 |
|19 |0 |0 |0 |0 |38 |0 |0 |
|3 |11 |4 |0 |0 |0 |40 |0 |
|14 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "macro avg": {"f1-score": 0.4927390941503914, "precision": 0.49438961045927854, "recall": 0.49650653298069714, "support": 706}, "micro avg": {"f1-score": 0.896600566572238, "precision": 0.896600566572238, "recall": 0.896600566572238, "support": 706}, "weighted avg": {"f1-score": 0.8882742384764851, "precision": 0.8835230291908613, "recall": 0.896600566572238, "support": 706}, "\u2205": {"f1-score": 0.9376979936642027, "precision": 0.9154639175257732, "recall": 0.961038961038961, "support": 462}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8941176470588235, "precision": 0.8085106382978723, "recall": 1.0, "support": 38}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.792079207920792, "precision": 0.8695652173913043, "recall": 0.7272727272727273, "support": 55}, "\u2423": {"f1-score": 0.825278810408922, "precision": 0.8671875, "recall": 0.7872340425531915, "support": 141}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 20}, "macro avg": {"f1-score": 0.415312111908995, "precision": 0.49438961045927854, "recall": 0.3663294326862543, "support": 1053}, "micro avg": {"f1-score": 0.7197271176805002, "precision": 0.896600566572238, "recall": 0.6011396011396012, "support": 1053}, "weighted avg": {"f1-score": 0.6746862872421505, "precision": 0.7980365944595451, "recall": 0.6011396011396012, "support": 1053}, "\u2205": {"f1-score": 0.8563162970106076, "precision": 0.9154639175257732, "recall": 0.8043478260869565, "support": 552}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 77}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7307692307692306, "precision": 0.8085106382978723, "recall": 0.6666666666666666, "support": 57}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7692307692307693, "precision": 0.8695652173913043, "recall": 0.6896551724137931, "support": 58}, "\u2423": {"f1-score": 0.5508684863523574, "precision": 0.8671875, "recall": 0.4036363636363636, "support": 275}},
  "ppcr": 0.6704653371320038
}
```
</details>
