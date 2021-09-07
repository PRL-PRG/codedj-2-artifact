# Train report for javascript / file:///tmp/top-repos-quality-repos-8z63go5b/notelifymyself.git HEAD 9dd300ec9ff98884e9b5813a386b1f94be871cd7

### Classification report

PPCR: 0.369

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `␣` | 0.973| 0.992| 0.396| 0.982| 0.563| 1343| 3366| 0.399 |
| `∅` | 0.994| 0.975| 0.256| 0.984| 0.407| 1294| 4927| 0.263 |
| `'` | 1.000| 1.000| 0.838| 1.000| 0.912| 1157| 1380| 0.838 |
| `⏎` | 0.917| 0.969| 0.280| 0.943| 0.429| 229| 793| 0.289 |
| `⏎␣⁺␣⁺` | 0.939| 0.933| 0.411| 0.936| 0.572| 164| 372| 0.441 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 14| 169| 0.083 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 368| 0.000 |
| `macro avg` | 0.689| 0.696| 0.312| 0.692| 0.412| 4201| 11375| 0.369 |
| `weighted avg` | 0.979| 0.982| 0.363| 0.981| 0.502| 4201| 11375| 0.369 |
| `micro avg` | 0.982| 0.982| 0.363| 0.982| 0.530| 4201| 11375| 0.369 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|3633 |1262 |27 |0 |5 |0 |0 |0 |
|2023 |0 |1332 |0 |1 |10 |0 |0 |
|223 |0 |0 |1157 |0 |0 |0 |0 |
|564 |2 |5 |0 |222 |0 |0 |0 |
|208 |6 |5 |0 |0 |153 |0 |0 |
|368 |0 |0 |0 |0 |0 |0 |0 |
|155 |0 |0 |0 |14 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/store/actions/card/cardActions.spec.js | 11 |
| src/containers/layout/layout.js | 9 |
| src/store/actions/card/cardActions.js | 6 |
| src/containers/form/form.js | 5 |
| src/store/utils/reducerGen/reducerGen.spec.js | 4 |
| src/components/category/category.spec.js | 3 |
| src/components/formElements/formElements.js | 3 |
| src/store/actions/utils/utils.js | 2 |
| test/setup.js | 2 |
| src/containers/cards/cards.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1157}, "macro avg": {"f1-score": 0.6921650338114602, "precision": 0.6889542055748344, "recall": 0.6956341435419828, "support": 4201}, "micro avg": {"f1-score": 0.9821471078314687, "precision": 0.9821471078314687, "recall": 0.9821471078314687, "support": 4201}, "weighted avg": {"f1-score": 0.9805720976863842, "precision": 0.9791869916858792, "recall": 0.9821471078314687, "support": 4201}, "\u2205": {"f1-score": 0.9843993759750389, "precision": 0.9937007874015747, "recall": 0.9752704791344667, "support": 1294}, "\u23ce": {"f1-score": 0.9426751592356689, "precision": 0.9173553719008265, "recall": 0.9694323144104804, "support": 229}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9357798165137614, "precision": 0.9386503067484663, "recall": 0.9329268292682927, "support": 164}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9823008849557523, "precision": 0.972972972972973, "recall": 0.9918093819806404, "support": 1343}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9121009065825779, "precision": 1.0, "recall": 0.8384057971014492, "support": 1380}, "macro avg": {"f1-score": 0.411851668387673, "precision": 0.6889542055748344, "recall": 0.311643891739894, "support": 11375}, "micro avg": {"f1-score": 0.5297894196199281, "precision": 0.9821471078314687, "recall": 0.3627252747252747, "support": 11375}, "weighted avg": {"f1-score": 0.5021684859714698, "precision": 0.9342972774138347, "recall": 0.3627252747252747, "support": 11375}, "\u2205": {"f1-score": 0.407293851863805, "precision": 0.9937007874015747, "recall": 0.2561396387253907, "support": 4927}, "\u23ce": {"f1-score": 0.4289855072463768, "precision": 0.9173553719008265, "recall": 0.27994955863808324, "support": 793}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 169}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.5719626168224299, "precision": 0.9386503067484663, "recall": 0.4112903225806452, "support": 372}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 368}, "\u2423": {"f1-score": 0.5626187961985216, "precision": 0.972972972972973, "recall": 0.39572192513368987, "support": 3366}},
  "ppcr": 0.36931868131868134
}
```
</details>
