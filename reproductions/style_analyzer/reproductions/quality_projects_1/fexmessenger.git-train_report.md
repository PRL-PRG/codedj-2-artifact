# Train report for javascript / file:///tmp/top-repos-quality-repos-in1yetyr/fexmessenger.git HEAD ee60badcc320ef78fe7e3eb6766a2efb70c9fa4f

### Classification report

PPCR: 0.945

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.973| 0.992| 0.985| 0.982| 0.979| 118195| 119070| 0.993 |
| `␣` | 0.964| 0.961| 0.900| 0.963| 0.931| 60409| 64465| 0.937 |
| `'` | 1.000| 1.000| 0.989| 1.000| 0.995| 29639| 29955| 0.989 |
| `⏎` | 0.935| 0.924| 0.720| 0.929| 0.813| 16262| 20873| 0.779 |
| `⏎⇥⁻` | 0.954| 0.869| 0.763| 0.910| 0.848| 6496| 7391| 0.879 |
| `⏎⇥⁺` | 0.954| 0.815| 0.678| 0.879| 0.793| 6221| 7482| 0.831 |
| `⏎⏎` | 0.962| 0.772| 0.381| 0.856| 0.546| 1643| 3324| 0.494 |
| `⏎⇥⁻⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 65| 130| 0.500 |
| `⏎⇥⁺⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 28| 130| 0.215 |
| `macro avg` | 0.749| 0.704| 0.602| 0.724| 0.656| 238958| 252820| 0.945 |
| `micro avg` | 0.971| 0.971| 0.917| 0.971| 0.943| 238958| 252820| 0.945 |
| `weighted avg` | 0.970| 0.971| 0.917| 0.970| 0.939| 238958| 252820| 0.945 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⇥⁺| ⏎⇥⁻| ⏎⏎| ⏎⇥⁻⇥⁻| ⏎⇥⁺⇥⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|875 |117247 |904 |0 |1 |18 |25 |0 |0 |0 |
|4056 |1268 |58030 |0 |685 |222 |199 |5 |0 |0 |
|316 |0 |0 |29639 |0 |0 |0 |0 |0 |0 |
|4611 |517 |659 |0 |15030 |4 |7 |45 |0 |0 |
|1261 |865 |277 |0 |7 |5072 |0 |0 |0 |0 |
|895 |576 |272 |0 |5 |0 |5643 |0 |0 |0 |
|1681 |8 |21 |0 |346 |0 |0 |1268 |0 |0 |
|65 |23 |3 |0 |0 |0 |39 |0 |0 |0 |
|102 |22 |0 |0 |6 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| app/lib/database/schema/app.js | 210 |
| app/presentation/ImageViewer/index.android.js | 174 |
| app/lib/rocketchat.js | 168 |
| app/containers/MessageBox/index.js | 150 |
| app/views/RoomView/index.js | 143 |
| app/views/RoomsListView/index.js | 137 |
| app/views/RoomActionsView/index.js | 136 |
| storybook/stories/UiKitMessage.js | 135 |
| app/views/RoomInfoEditView/index.js | 132 |
| app/views/ProfileView/index.js | 125 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 29639}, "macro avg": {"f1-score": 0.7243906415971778, "precision": 0.7491662695721085, "recall": 0.7036209749923977, "support": 238958}, "micro avg": {"f1-score": 0.9705847889587291, "precision": 0.9705847889587291, "recall": 0.9705847889587291, "support": 238958}, "weighted avg": {"f1-score": 0.9699940018912182, "precision": 0.97003831759594, "recall": 0.9705847889587291, "support": 238958}, "\u2205": {"f1-score": 0.9822931371768718, "precision": 0.9727942518626687, "recall": 0.9919793561487372, "support": 118195}, "\u23ce": {"f1-score": 0.9294415929750788, "precision": 0.9347014925373134, "recall": 0.9242405608166278, "support": 16262}, "\u23ce\u21e5\u207a": {"f1-score": 0.8792580393516513, "precision": 0.9541008276899925, "recall": 0.8153030059475969, "support": 6221}, "\u23ce\u21e5\u207a\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 28}, "\u23ce\u21e5\u207b": {"f1-score": 0.9095011685067289, "precision": 0.954337899543379, "recall": 0.8686884236453202, "support": 6496}, "\u23ce\u21e5\u207b\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 65}, "\u23ce\u23ce": {"f1-score": 0.8564674096588991, "precision": 0.9620637329286799, "recall": 0.7717589774802192, "support": 1643}, "\u2423": {"f1-score": 0.9625544267053701, "precision": 0.9644982215869428, "recall": 0.9606184508930788, "support": 60409}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9946974527637009, "precision": 1.0, "recall": 0.9894508429310632, "support": 29955}, "macro avg": {"f1-score": 0.6561504891470489, "precision": 0.7491662695721085, "recall": 0.6019161963661159, "support": 252820}, "micro avg": {"f1-score": 0.9432264151710732, "precision": 0.9705847889587291, "recall": 0.9173680879677241, "support": 252820}, "weighted avg": {"f1-score": 0.938842197252215, "precision": 0.9685230888939883, "recall": 0.9173680879677241, "support": 252820}, "\u2205": {"f1-score": 0.9787058214661347, "precision": 0.9727942518626687, "recall": 0.9846896783404719, "support": 119070}, "\u23ce": {"f1-score": 0.8134657537953617, "precision": 0.9347014925373134, "recall": 0.7200689886456187, "support": 20873}, "\u23ce\u21e5\u207a": {"f1-score": 0.7926238474761681, "precision": 0.9541008276899925, "recall": 0.6778936113338679, "support": 7482}, "\u23ce\u21e5\u207a\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 130}, "\u23ce\u21e5\u207b": {"f1-score": 0.8483162958508719, "precision": 0.954337899543379, "recall": 0.7634961439588689, "support": 7391}, "\u23ce\u21e5\u207b\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 130}, "\u23ce\u23ce": {"f1-score": 0.5463162429987074, "precision": 0.9620637329286799, "recall": 0.381468110709988, "support": 3324}, "\u2423": {"f1-score": 0.9312289879724949, "precision": 0.9644982215869428, "recall": 0.9001783913751649, "support": 64465}},
  "ppcr": 0.9451704770192232
}
```
</details>
