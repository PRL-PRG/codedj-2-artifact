# Train report for javascript / file:///tmp/top-repos-quality-repos-2ovvvp80/ra16-homeworks.git HEAD fd001024467295eef79c44013c65eaa7838f2d21

### Classification report

PPCR: 0.658

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.971| 0.997| 0.891| 0.984| 0.929| 16447| 18416| 0.893 |
| `␣` | 0.987| 0.976| 0.475| 0.981| 0.642| 4100| 8419| 0.487 |
| `'` | 0.996| 1.000| 0.859| 0.998| 0.923| 1945| 2263| 0.859 |
| `⏎` | 0.991| 0.708| 0.118| 0.826| 0.211| 312| 1867| 0.167 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 159| 1176| 0.135 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 61| 287| 0.213 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 43| 934| 0.046 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 37| 392| 0.094 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 713| 0.011 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 665| 0.002 |
| `macro avg` | 0.395| 0.368| 0.234| 0.379| 0.270| 23113| 35132| 0.658 |
| `weighted avg` | 0.963| 0.976| 0.642| 0.969| 0.711| 23113| 35132| 0.658 |
| `micro avg` | 0.976| 0.976| 0.642| 0.976| 0.775| 23113| 35132| 0.658 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| ⏎⏎| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1969 |16400 |47 |0 |0 |0 |0 |0 |0 |0 |0 |
|4319 |98 |4001 |0 |1 |0 |0 |0 |0 |0 |0 |
|318 |0 |0 |1945 |0 |0 |0 |0 |0 |0 |0 |
|1555 |90 |1 |0 |221 |0 |0 |0 |0 |0 |0 |
|1017 |158 |1 |0 |0 |0 |0 |0 |0 |0 |0 |
|891 |43 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|705 |0 |0 |8 |0 |0 |0 |0 |0 |0 |0 |
|664 |0 |0 |0 |1 |0 |0 |0 |0 |0 |0 |
|355 |37 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|226 |58 |3 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| lifecycle-http/crud/frontend/src/Crud.js | 19 |
| hoc/highlight/src/App.js | 17 |
| hooks-context/use-effect/src/App.js | 13 |
| events-state/layouts/src/components/Store.js | 12 |
| redux/api/frontend/src/serviceWorker.js | 10 |
| events-state/filter/src/serviceWorker.js | 10 |
| components/store-func/store-func/src/serviceWorker.js | 10 |
| composition/decomposition/src/serviceWorker.js | 10 |
| props/films/src/serviceWorker.js | 10 |
| events-state/layouts/src/serviceWorker.js | 10 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u0027": {"f1-score": 0.9979476654694716, "precision": 0.9959037378392217, "recall": 1.0, "support": 1945}, "macro avg": {"f1-score": 0.37896639846971836, "precision": 0.3945438932661145, "recall": 0.3681329327858245, "support": 23113}, "micro avg": {"f1-score": 0.9763769307316229, "precision": 0.9763769307316229, "recall": 0.9763769307316229, "support": 23113}, "weighted avg": {"f1-score": 0.969489828469837, "precision": 0.9634906191310817, "recall": 0.9763769307316229, "support": 23113}, "\u2205": {"f1-score": 0.9840688848219374, "precision": 0.9713338071547026, "recall": 0.9971423359883261, "support": 16447}, "\u23ce": {"f1-score": 0.8261682242990654, "precision": 0.9910313901345291, "recall": 0.7083333333333334, "support": 312}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 159}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 61}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 43}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 37}, "\u2423": {"f1-score": 0.9814792101067091, "precision": 0.9871699975326919, "recall": 0.9758536585365853, "support": 4100}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 713}, "\u0027": {"f1-score": 0.9226755218216319, "precision": 0.9959037378392217, "recall": 0.859478568272205, "support": 2263}, "macro avg": {"f1-score": 0.2704934423343105, "precision": 0.3945438932661145, "recall": 0.23436148499746762, "support": 35132}, "micro avg": {"f1-score": 0.7748991329727875, "precision": 0.9763769307316229, "recall": 0.642348855744051, "support": 35132}, "weighted avg": {"f1-score": 0.7114941161168873, "precision": 0.8625484847404092, "recall": 0.642348855744051, "support": 35132}, "\u2205": {"f1-score": 0.9291784702549576, "precision": 0.9713338071547026, "recall": 0.8905299739357081, "support": 18416}, "\u23ce": {"f1-score": 0.21148325358851675, "precision": 0.9910313901345291, "recall": 0.11837171933583289, "support": 1867}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 665}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1176}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 287}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 934}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 392}, "\u2423": {"f1-score": 0.6415971776779987, "precision": 0.9871699975326919, "recall": 0.47523458843093, "support": 8419}},
  "ppcr": 0.6578902425139473
}
```
</details>
