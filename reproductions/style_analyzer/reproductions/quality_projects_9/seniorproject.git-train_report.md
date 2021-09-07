# Train report for javascript / file:///tmp/top-repos-quality-repos-6irnugou/seniorproject.git HEAD 30f18fe29fad80e91915a04d8157b4d1496c8d9b

### Classification report

PPCR: 0.667

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.894| 0.997| 0.943| 0.942| 0.918| 10366| 10960| 0.946 |
| `␣` | 0.960| 0.598| 0.304| 0.737| 0.462| 2471| 4857| 0.509 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 108| 336| 0.321 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 77| 1218| 0.063 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 57| 357| 0.160 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 22| 232| 0.095 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1525| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 164| 0.000 |
| `weighted avg` | 0.888| 0.902| 0.601| 0.885| 0.626| 13101| 19649| 0.667 |
| `macro avg` | 0.232| 0.199| 0.156| 0.210| 0.172| 13101| 19649| 0.667 |
| `micro avg` | 0.902| 0.902| 0.601| 0.902| 0.721| 13101| 19649| 0.667 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|594 |10334 |32 |0 |0 |0 |0 |0 |0 |
|2386 |994 |1477 |0 |0 |0 |0 |0 |0 |
|1525 |0 |0 |0 |0 |0 |0 |0 |0 |
|1141 |60 |17 |0 |0 |0 |0 |0 |0 |
|300 |46 |11 |0 |0 |0 |0 |0 |0 |
|228 |107 |1 |0 |0 |0 |0 |0 |0 |
|210 |22 |0 |0 |0 |0 |0 |0 |0 |
|164 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| React-Native-App/App-Andy-Old.js | 107 |
| React-Native-App/src/screens/DocumentsScreen.js | 105 |
| React-Native-App/src/components/ImagePick.js | 104 |
| React-Native-App/src/screens/ProfileScreen.js | 90 |
| React-Native-App/src/screens/ClinicsScreen.js | 79 |
| React-Native-App/src/components/MyPicker.js | 75 |
| React-Native-App/src/context/AuthContext.js | 73 |
| React-Native-App/src/components/SignUp.js | 65 |
| React-Native-App/src/screens/ClassesScreen.js | 54 |
| React-Native-App/src/screens/SexEdScreen.js | 51 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.2099172928902127, "precision": 0.23175635071823977, "recall": 0.1993308369757526, "support": 13101}, "micro avg": {"f1-score": 0.9015342340279368, "precision": 0.9015342340279368, "recall": 0.9015342340279368, "support": 13101}, "weighted avg": {"f1-score": 0.8847151243223361, "precision": 0.8882697001426572, "recall": 0.9015342340279368, "support": 13101}, "\u2205": {"f1-score": 0.9424962378585435, "precision": 0.8937127043154891, "recall": 0.9969129847578623, "support": 10366}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 77}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 57}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 108}, "\u2423": {"f1-score": 0.736842105263158, "precision": 0.9603381014304291, "recall": 0.5977337110481586, "support": 2471}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 164}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1525}, "macro avg": {"f1-score": 0.17244539045952056, "precision": 0.23175635071823977, "recall": 0.1558725488759545, "support": 19649}, "micro avg": {"f1-score": 0.7212824427480916, "precision": 0.9015342340279368, "recall": 0.6010992925848644, "support": 19649}, "weighted avg": {"f1-score": 0.6260315263759624, "precision": 0.7358874954422798, "recall": 0.6010992925848644, "support": 19649}, "\u2205": {"f1-score": 0.9176397460373841, "precision": 0.8937127043154891, "recall": 0.9428832116788322, "support": 10960}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1218}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 232}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 357}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 336}, "\u2423": {"f1-score": 0.46192337763878033, "precision": 0.9603381014304291, "recall": 0.3040971793288038, "support": 4857}},
  "ppcr": 0.6667514886253754
}
```
</details>
