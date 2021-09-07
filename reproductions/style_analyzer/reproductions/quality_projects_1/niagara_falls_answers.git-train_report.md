# Train report for javascript / file:///tmp/top-repos-quality-repos-y_kfgbkg/niagara_falls_answers.git HEAD 31e233af8283aba82ae72a776cc166ad99181558

### Classification report

PPCR: 0.415

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.962| 1.000| 0.788| 0.981| 0.867| 4079| 5174| 0.788 |
| `␣` | 1.000| 0.651| 0.067| 0.788| 0.126| 315| 3055| 0.103 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 25| 890| 0.028 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 15| 137| 0.109 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 376| 0.016 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 125| 0.032 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 756| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 204| 0.000 |
| `macro avg` | 0.245| 0.206| 0.107| 0.221| 0.124| 4444| 10717| 0.415 |
| `micro avg` | 0.964| 0.964| 0.400| 0.964| 0.565| 4444| 10717| 0.415 |
| `weighted avg` | 0.954| 0.964| 0.400| 0.956| 0.454| 4444| 10717| 0.415 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| '| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1095 |4079 |0 |0 |0 |0 |0 |0 |0 |
|2740 |110 |205 |0 |0 |0 |0 |0 |0 |
|865 |25 |0 |0 |0 |0 |0 |0 |0 |
|756 |0 |0 |0 |0 |0 |0 |0 |0 |
|370 |6 |0 |0 |0 |0 |0 |0 |0 |
|122 |15 |0 |0 |0 |0 |0 |0 |0 |
|204 |0 |0 |0 |0 |0 |0 |0 |0 |
|121 |4 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| app/assets/javascripts/pagedown/Markdown.Converter.js | 48 |
| app/assets/javascripts/jquery-jvert-tabs-1.1.4.js | 33 |
| app/assets/javascripts/Markdown.Extra.js | 32 |
| app/assets/javascripts/mustache.js | 29 |
| app/assets/javascripts/pagedown/Markdown.Sanitizer.js | 11 |
| app/assets/javascripts/pagedown/demo/node/demo.js | 3 |
| app/assets/javascripts/home.js | 2 |
| app/assets/javascripts/pagedown/local/Markdown.local.fr.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.22115326816727393, "precision": 0.24528190610993159, "recall": 0.20634920634920634, "support": 4444}, "micro avg": {"f1-score": 0.963996399639964, "precision": 0.963996399639964, "recall": 0.963996399639964, "support": 4444}, "weighted avg": {"f1-score": 0.9560990585205339, "precision": 0.9541042214624859, "recall": 0.963996399639964, "support": 4444}, "\u2205": {"f1-score": 0.980764606876653, "precision": 0.9622552488794527, "recall": 1.0, "support": 4079}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 25}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 15}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u2423": {"f1-score": 0.7884615384615385, "precision": 1.0, "recall": 0.6507936507936508, "support": 315}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 756}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 204}, "macro avg": {"f1-score": 0.12405507752950984, "precision": 0.24528190610993159, "recall": 0.10693350138581616, "support": 10717}, "micro avg": {"f1-score": 0.5651342259745399, "precision": 0.963996399639964, "recall": 0.3997387328543436, "support": 10717}, "weighted avg": {"f1-score": 0.45426777727922657, "precision": 0.7496229035833059, "recall": 0.3997387328543436, "support": 10717}, "\u2205": {"f1-score": 0.8666737490704345, "precision": 0.9622552488794527, "recall": 0.7883649014302281, "support": 5174}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 890}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 376}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 137}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 125}, "\u2423": {"f1-score": 0.1257668711656442, "precision": 1.0, "recall": 0.06710310965630115, "support": 3055}},
  "ppcr": 0.41466828403471123
}
```
</details>
