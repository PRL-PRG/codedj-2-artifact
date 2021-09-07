# Train report for javascript / file:///tmp/top-repos-quality-repos-j1zadpqs/certificates-for-everyone-netlify.git HEAD 04db893536e992b028c2f4cfe2e79bfbcd3e9b1b

### Classification report

PPCR: 0.585

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.953| 1.000| 0.810| 0.976| 0.876| 4720| 5826| 0.810 |
| `␣` | 0.919| 0.632| 0.126| 0.749| 0.221| 467| 2343| 0.199 |
| `"` | 1.000| 1.000| 0.500| 1.000| 0.667| 335| 670| 0.500 |
| `⏎` | 0.952| 0.706| 0.120| 0.811| 0.214| 85| 499| 0.170 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 27| 92| 0.293 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 27| 177| 0.153 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 12| 96| 0.125 |
| `weighted avg` | 0.942| 0.954| 0.558| 0.945| 0.636| 5673| 9703| 0.585 |
| `micro avg` | 0.954| 0.954| 0.558| 0.954| 0.704| 5673| 9703| 0.585 |
| `macro avg` | 0.546| 0.477| 0.222| 0.505| 0.282| 5673| 9703| 0.585 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1106 |4720 |0 |0 |0 |0 |0 |0 |
|1876 |172 |295 |0 |0 |0 |0 |0 |
|335 |0 |0 |335 |0 |0 |0 |0 |
|414 |25 |0 |0 |60 |0 |0 |0 |
|65 |27 |0 |0 |0 |0 |0 |0 |
|84 |10 |2 |0 |0 |0 |0 |0 |
|150 |0 |24 |0 |3 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/components/list-events/index.js | 74 |
| src/serviceWorker.js | 31 |
| src/components/list-presents/index.js | 26 |
| src/components/form-login/index.js | 24 |
| src/components/profile-card/index.js | 19 |
| src/lambda/async-dadjoke.js | 17 |
| src/components/form-register/index.js | 10 |
| src/components/info-event/index.js | 9 |
| src/pages/home/index.js | 7 |
| src/routes.js | 6 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 335}, "macro avg": {"f1-score": 0.5050504612368586, "precision": 0.5463070728161095, "recall": 0.4767962859662066, "support": 5673}, "micro avg": {"f1-score": 0.9536400493566014, "precision": 0.9536400493566014, "recall": 0.9536400493566014, "support": 5673}, "weighted avg": {"f1-score": 0.9447217238285789, "precision": 0.941685126447622, "recall": 0.9536400493566014, "support": 5673}, "\u2205": {"f1-score": 0.9758114533801943, "precision": 0.9527654420670165, "recall": 1.0, "support": 4720}, "\u23ce": {"f1-score": 0.810810810810811, "precision": 0.9523809523809523, "recall": 0.7058823529411765, "support": 85}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 27}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 27}, "\u2423": {"f1-score": 0.7487309644670049, "precision": 0.9190031152647975, "recall": 0.6316916488222698, "support": 467}},
  "cl_report_full": {"\"": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 670}, "macro avg": {"f1-score": 0.2824795718070466, "precision": 0.5463070728161095, "recall": 0.22232982622093184, "support": 9703}, "micro avg": {"f1-score": 0.70369406867846, "precision": 0.9536400493566014, "recall": 0.5575595176749459, "support": 9703}, "weighted avg": {"f1-score": 0.6362903953979774, "precision": 0.9120142079548545, "recall": 0.5575595176749459, "support": 9703}, "\u2205": {"f1-score": 0.8756957328385898, "precision": 0.9527654420670165, "recall": 0.8101613456917267, "support": 5826}, "\u23ce": {"f1-score": 0.21352313167259787, "precision": 0.9523809523809523, "recall": 0.12024048096192384, "support": 499}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 177}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 96}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 92}, "\u2423": {"f1-score": 0.22147147147147145, "precision": 0.9190031152647975, "recall": 0.12590695689287237, "support": 2343}},
  "ppcr": 0.5846645367412141
}
```
</details>
