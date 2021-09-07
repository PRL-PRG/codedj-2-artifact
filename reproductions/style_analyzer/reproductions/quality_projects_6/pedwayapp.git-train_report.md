# Train report for javascript / file:///tmp/top-repos-quality-repos-nbv4mtwe/pedwayapp.git HEAD c444e47e59396d7c68f58277ec70f494e97ad7f5

### Classification report

PPCR: 0.798

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.960| 0.993| 0.924| 0.976| 0.942| 18354| 19723| 0.931 |
| `␣` | 0.961| 0.899| 0.575| 0.929| 0.719| 3441| 5380| 0.640 |
| `'` | 1.000| 1.000| 0.810| 1.000| 0.895| 2094| 2584| 0.810 |
| `⏎␣⁺␣⁺` | 0.932| 0.906| 0.573| 0.919| 0.710| 670| 1060| 0.632 |
| `⏎` | 0.947| 0.717| 0.226| 0.816| 0.365| 600| 1900| 0.316 |
| `⏎␣⁻␣⁻` | 0.994| 0.720| 0.321| 0.835| 0.485| 429| 963| 0.445 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 123| 156| 0.788 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 457| 0.002 |
| `micro avg` | 0.963| 0.963| 0.768| 0.963| 0.855| 25712| 32223| 0.798 |
| `weighted avg` | 0.958| 0.963| 0.768| 0.960| 0.828| 25712| 32223| 0.798 |
| `macro avg` | 0.724| 0.654| 0.429| 0.684| 0.515| 25712| 32223| 0.798 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1369 |18224 |94 |0 |0 |34 |2 |0 |0 |
|1939 |316 |3094 |0 |22 |9 |0 |0 |0 |
|490 |0 |0 |2094 |0 |0 |0 |0 |0 |
|1300 |161 |9 |0 |430 |0 |0 |0 |0 |
|390 |42 |19 |0 |2 |607 |0 |0 |0 |
|534 |119 |0 |0 |0 |1 |309 |0 |0 |
|456 |0 |1 |0 |0 |0 |0 |0 |0 |
|33 |119 |4 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| pedwayApp/components/GroundMapView/GroundMapView.js | 120 |
| backend/test/authController.test.js | 87 |
| backend/test/userController.test.js | 57 |
| backend/test/feedbackController.test.js | 52 |
| backend/test/sectionController.test.js | 46 |
| backend/test/orsController.test.js | 45 |
| backend/test/entranceController.test.js | 44 |
| backend/api/controllers/authController.js | 43 |
| pedwayApp/components/SlidingUpDetailView/SlidingUpDetailView.js | 41 |
| pedwayApp/App.js | 38 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 2094}, "macro avg": {"f1-score": 0.6843959372823825, "precision": 0.7242258292463043, "recall": 0.6543738541543653, "support": 25712}, "micro avg": {"f1-score": 0.9628967019290604, "precision": 0.9628967019290604, "recall": 0.9628967019290604, "support": 25712}, "weighted avg": {"f1-score": 0.9595398514543793, "precision": 0.9583295970347308, "recall": 0.9628967019290604, "support": 25712}, "\u2205": {"f1-score": 0.9762421320476764, "precision": 0.9601180127495917, "recall": 0.992917075296938, "support": 18354}, "\u23ce": {"f1-score": 0.8159392789373814, "precision": 0.947136563876652, "recall": 0.7166666666666667, "support": 600}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.919000757002271, "precision": 0.9324116743471582, "recall": 0.9059701492537313, "support": 670}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 123}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8351351351351352, "precision": 0.9935691318327974, "recall": 0.7202797202797203, "support": 429}, "\u2423": {"f1-score": 0.9288501951365956, "precision": 0.9605712511642347, "recall": 0.8991572217378669, "support": 3441}},
  "cl_report_full": {"\u0027": {"f1-score": 0.8952543822146216, "precision": 1.0, "recall": 0.8103715170278638, "support": 2584}, "macro avg": {"f1-score": 0.5145456992093174, "precision": 0.7242258292463043, "recall": 0.4286614237957554, "support": 32223}, "micro avg": {"f1-score": 0.8546819711745922, "precision": 0.9628967019290604, "recall": 0.768333178164665, "support": 32223}, "weighted avg": {"f1-score": 0.8276924508862723, "precision": 0.9444497351845698, "recall": 0.768333178164665, "support": 32223}, "\u2205": {"f1-score": 0.9417114510128153, "precision": 0.9601180127495917, "recall": 0.923997363484257, "support": 19723}, "\u23ce": {"f1-score": 0.3653355989804588, "precision": 0.947136563876652, "recall": 0.22631578947368422, "support": 1900}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 457}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7095265926358855, "precision": 0.9324116743471582, "recall": 0.5726415094339623, "support": 1060}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 156}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.48508634222919933, "precision": 0.9935691318327974, "recall": 0.32087227414330216, "support": 963}, "\u2423": {"f1-score": 0.719451226601558, "precision": 0.9605712511642347, "recall": 0.575092936802974, "support": 5380}},
  "ppcr": 0.7979393600844118
}
```
</details>
