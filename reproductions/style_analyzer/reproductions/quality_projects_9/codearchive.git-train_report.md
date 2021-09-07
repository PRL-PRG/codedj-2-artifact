# Train report for javascript / file:///tmp/top-repos-quality-repos-vchuedkx/codearchive.git HEAD 34e5f13f89f383a68790da61ff2f2e3c1094cd45

### Classification report

PPCR: 0.697

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.979| 1.000| 0.918| 0.989| 0.947| 26034| 28365| 0.918 |
| `"` | 0.996| 1.000| 0.989| 0.998| 0.993| 4201| 4248| 0.989 |
| `␣` | 0.994| 0.785| 0.139| 0.877| 0.245| 1374| 7728| 0.178 |
| `⏎` | 0.996| 0.884| 0.089| 0.937| 0.164| 268| 2653| 0.101 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 118| 1270| 0.093 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 99| 1220| 0.081 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 18| 118| 0.153 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 16| 83| 0.193 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 355| 0.025 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 96| 0.062 |
| `weighted avg` | 0.974| 0.982| 0.684| 0.977| 0.724| 32143| 46136| 0.697 |
| `macro avg` | 0.396| 0.367| 0.214| 0.380| 0.235| 32143| 46136| 0.697 |
| `micro avg` | 0.982| 0.982| 0.684| 0.982| 0.806| 32143| 46136| 0.697 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| '| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2331 |26034 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|6354 |296 |1078 |0 |0 |0 |0 |0 |0 |0 |0 |
|47 |0 |0 |4201 |0 |0 |0 |0 |0 |0 |0 |
|2385 |31 |0 |0 |237 |0 |0 |0 |0 |0 |0 |
|1152 |113 |5 |0 |0 |0 |0 |0 |0 |0 |0 |
|1121 |99 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|346 |8 |0 |0 |1 |0 |0 |0 |0 |0 |0 |
|67 |0 |0 |16 |0 |0 |0 |0 |0 |0 |0 |
|100 |18 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|90 |4 |2 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/widgets/archives/records/NoteSegment.js | 39 |
| src/widgets/archives/Archive.js | 36 |
| src/widgets/archives/records/SnippetSegment.js | 29 |
| src/components/home/Home.js | 28 |
| src/components/authentication/Registration.js | 28 |
| src/components/library/Library.js | 27 |
| src/ApplicationViews.js | 26 |
| src/components/library/languagelibrary/LanguageLibrary.js | 23 |
| src/modules/API.js | 21 |
| src/hooks/useSimpleAuth.js | 20 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9980993110002375, "precision": 0.9962058335309462, "recall": 1.0, "support": 4201}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "macro avg": {"f1-score": 0.3800827496227045, "precision": 0.39641639747560375, "recall": 0.36688989550066264, "support": 32143}, "micro avg": {"f1-score": 0.9815511931058084, "precision": 0.9815511931058084, "recall": 0.9815511931058084, "support": 32143}, "weighted avg": {"f1-score": 0.9769261298864751, "precision": 0.9735942579632508, "recall": 0.9815511931058084, "support": 32143}, "\u2205": {"f1-score": 0.9891901134183179, "precision": 0.9786114348005864, "recall": 1.0, "support": 26034}, "\u23ce": {"f1-score": 0.9367588932806324, "precision": 0.9957983193277311, "recall": 0.8843283582089553, "support": 268}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 118}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 99}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 18}, "\u2423": {"f1-score": 0.8767791785278569, "precision": 0.9935483870967742, "recall": 0.784570596797671, "support": 1374}},
  "cl_report_full": {"\"": {"f1-score": 0.9925575900767868, "precision": 0.9962058335309462, "recall": 0.9889359698681732, "support": 4248}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 83}, "macro avg": {"f1-score": 0.23483953321365042, "precision": 0.39641639747560375, "recall": 0.21355828128423315, "support": 46136}, "micro avg": {"f1-score": 0.8060910333550506, "precision": 0.9815511931058084, "recall": 0.6838477544650599, "support": 46136}, "weighted avg": {"f1-score": 0.7241731012135846, "precision": 0.9170753989426571, "recall": 0.6838477544650599, "support": 46136}, "\u2205": {"f1-score": 0.9472420317275505, "precision": 0.9786114348005864, "recall": 0.9178212585933369, "support": 28365}, "\u23ce": {"f1-score": 0.16395710826703563, "precision": 0.9957983193277311, "recall": 0.08933283075763288, "support": 2653}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 355}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1270}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 96}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1220}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 118}, "\u2423": {"f1-score": 0.2446386020651311, "precision": 0.9935483870967742, "recall": 0.13949275362318841, "support": 7728}},
  "ppcr": 0.696701057742327
}
```
</details>
