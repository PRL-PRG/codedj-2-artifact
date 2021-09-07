# Train report for javascript / file:///tmp/top-repos-quality-repos-3rdli07t/robot-management-application.git HEAD 62d8414fac3f513a13d7bc31981ad83035cf0848

### Classification report

PPCR: 0.692

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.932| 0.986| 0.928| 0.958| 0.930| 63831| 67768| 0.942 |
| `␣` | 0.939| 0.822| 0.473| 0.877| 0.629| 22029| 38261| 0.576 |
| `'` | 1.000| 1.000| 0.262| 1.000| 0.415| 1115| 4255| 0.262 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 306| 7197| 0.043 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 301| 2680| 0.112 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 158| 1943| 0.081 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 95| 555| 0.171 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 36| 543| 0.066 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 24| 316| 0.076 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 1963| 0.005 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 249| 0.016 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1359| 0.000 |
| `macro avg` | 0.239| 0.234| 0.139| 0.236| 0.165| 87908| 127089| 0.692 |
| `micro avg` | 0.934| 0.934| 0.646| 0.934| 0.764| 87908| 127089| 0.692 |
| `weighted avg` | 0.925| 0.934| 0.646| 0.928| 0.699| 87908| 127089| 0.692 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⇥⁺| ⏎⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3937 |62908 |923 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|16232 |3925 |18104 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|6891 |230 |76 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3140 |0 |0 |0 |1115 |0 |0 |0 |0 |0 |0 |0 |0 |
|1359 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1954 |9 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1785 |52 |106 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2379 |296 |5 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|460 |39 |56 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|507 |36 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|292 |19 |5 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|245 |4 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| client/src/assets/ColladaLoader.js | 2735 |
| client/src/assets/ColladaLoader2.js | 687 |
| client/src/assets/ros3d.js | 496 |
| client/src/assets/roslib.js | 365 |
| client/src/assets/roslibjs/roslib.js | 365 |
| client/src/assets/ros2d.js | 198 |
| client/src/assets/ros2djs/ros2d.js | 198 |
| client/src/assets/STLLoader.js | 118 |
| client/src/app/home/husky/husky.component.js | 46 |
| client/src/app/home/jackal/jackal.component.js | 43 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1115}, "macro avg": {"f1-score": 0.23620814185771696, "precision": 0.23924747011787875, "recall": 0.23394714304353345, "support": 87908}, "micro avg": {"f1-score": 0.9342380670701188, "precision": 0.9342380670701188, "recall": 0.9342380670701188, "support": 87908}, "weighted avg": {"f1-score": 0.9278821611208928, "precision": 0.9245851294814439, "recall": 0.9342380670701188, "support": 87908}, "\u2205": {"f1-score": 0.9578755833694965, "precision": 0.931721911194052, "recall": 0.9855399414077799, "support": 63831}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 306}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 24}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 301}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 158}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 95}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 36}, "\u2423": {"f1-score": 0.8766221189231068, "precision": 0.9392477302204929, "recall": 0.8218257751146216, "support": 22029}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1359}, "\u0027": {"f1-score": 0.415270018621974, "precision": 1.0, "recall": 0.2620446533490012, "support": 4255}, "macro avg": {"f1-score": 0.16454837594037644, "precision": 0.23924747011787875, "recall": 0.13862504197475364, "support": 127089}, "micro avg": {"f1-score": 0.7639827532477198, "precision": 0.9342380670701188, "recall": 0.6462164310050437, "support": 127089}, "weighted avg": {"f1-score": 0.699267970093117, "precision": 0.8130718463735239, "recall": 0.6462164310050437, "support": 127089}, "\u2205": {"f1-score": 0.9300001478349571, "precision": 0.931721911194052, "recall": 0.9282847361586589, "support": 67768}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7197}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 316}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 249}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2680}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1943}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 555}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1963}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 543}, "\u2423": {"f1-score": 0.6293103448275862, "precision": 0.9392477302204929, "recall": 0.47317111418938346, "support": 38261}},
  "ppcr": 0.6917042387618126
}
```
</details>
