# Train report for javascript / file:///tmp/top-repos-quality-repos-bubz1k53/mattermost-sourcecode-edited.git HEAD 84908b440cb8c4a9fcb40968fdcb1bab8d2a47ad

### Classification report

PPCR: 0.902

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.991| 0.996| 0.965| 0.993| 0.978| 94817| 97789| 0.970 |
| `␣` | 0.974| 0.992| 0.948| 0.982| 0.961| 37170| 38874| 0.956 |
| `'` | 1.000| 1.000| 0.994| 1.000| 0.997| 19638| 19748| 0.994 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.992| 0.958| 0.851| 0.975| 0.916| 4932| 5553| 0.888 |
| `⏎` | 0.977| 0.890| 0.353| 0.931| 0.518| 4340| 10951| 0.396 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.950| 0.826| 0.529| 0.884| 0.680| 3628| 5661| 0.641 |
| `⏎⏎` | 0.962| 0.944| 0.314| 0.953| 0.474| 1967| 5907| 0.333 |
| `⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 40| 64| 0.625 |
| `weighted avg` | 0.987| 0.987| 0.891| 0.987| 0.922| 166532| 184547| 0.902 |
| `micro avg` | 0.987| 0.987| 0.891| 0.987| 0.936| 166532| 184547| 0.902 |
| `macro avg` | 0.856| 0.826| 0.619| 0.840| 0.690| 166532| 184547| 0.902 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2972 |94405 |308 |0 |0 |0 |104 |0 |0 |
|1704 |235 |36859 |0 |34 |39 |3 |0 |0 |
|110 |0 |0 |19638 |0 |0 |0 |0 |0 |
|6611 |148 |247 |0 |3861 |34 |50 |0 |0 |
|3940 |10 |56 |0 |45 |1856 |0 |0 |0 |
|2033 |305 |318 |0 |9 |0 |2996 |0 |0 |
|621 |132 |73 |0 |1 |0 |0 |4726 |0 |
|24 |1 |1 |0 |0 |0 |0 |38 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| utils/a11y_controller.js | 119 |
| components/channel_header/channel_header.js | 114 |
| reducers/views/rhs.js | 86 |
| webpack.config.js | 65 |
| e2e/cypress/support/api_commands.js | 58 |
| reducers/storage.test.js | 58 |
| components/channel_header_dropdown/channel_header_dropdown_items.js | 51 |
| reducers/views/rhs.test.js | 44 |
| e2e/cypress/integration/search/date_filter_spec.js | 41 |
| e2e/cypress/integration/enterprise/elasticsearch/users_spec.js | 41 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 19638}, "macro avg": {"f1-score": 0.839835430487575, "precision": 0.8558297427510255, "recall": 0.8255649176418812, "support": 166532}, "micro avg": {"f1-score": 0.9868433694425094, "precision": 0.9868433694425095, "recall": 0.9868433694425095, "support": 166532}, "weighted avg": {"f1-score": 0.9865036356828469, "precision": 0.9865237328298254, "recall": 0.9868433694425095, "support": 166532}, "\u2205": {"f1-score": 0.9934597191309793, "precision": 0.9912743080347768, "recall": 0.9956547876435661, "support": 94817}, "\u23ce": {"f1-score": 0.9314837153196622, "precision": 0.9774683544303797, "recall": 0.8896313364055299, "support": 4340}, "\u23ce\u23ce": {"f1-score": 0.9527720739219714, "precision": 0.96215655780197, "recall": 0.9435688866293849, "support": 1967}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.8836454800176965, "precision": 0.9502061528702823, "recall": 0.8257993384785005, "support": 3628}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9748349834983498, "precision": 0.9920235096557515, "recall": 0.9582319545823196, "support": 4932}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 40}, "\u2423": {"f1-score": 0.9824874720119416, "precision": 0.9735090592150442, "recall": 0.9916330373957493, "support": 37170}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9972071294368557, "precision": 1.0, "recall": 0.994429815677537, "support": 19748}, "macro avg": {"f1-score": 0.6904947007133815, "precision": 0.8558297427510255, "recall": 0.619383900619862, "support": 184547}, "micro avg": {"f1-score": 0.9362052415553194, "precision": 0.9868433694425095, "recall": 0.8905102765149258, "support": 184547}, "weighted avg": {"f1-score": 0.9217204319728408, "precision": 0.9851336125385881, "recall": 0.8905102765149258, "support": 184547}, "\u2205": {"f1-score": 0.9781634503302681, "precision": 0.9912743080347768, "recall": 0.9653948808148156, "support": 97789}, "\u23ce": {"f1-score": 0.5182202536742501, "precision": 0.9774683544303797, "recall": 0.35257054150305905, "support": 10951}, "\u23ce\u23ce": {"f1-score": 0.47371107708014293, "precision": 0.96215655780197, "recall": 0.3142034873878449, "support": 5907}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.6798275470841844, "precision": 0.9502061528702823, "recall": 0.5292351174704116, "support": 5661}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9161577978094407, "precision": 0.9920235096557515, "recall": 0.8510714928867279, "support": 5553}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 64}, "\u2423": {"f1-score": 0.9606703502919098, "precision": 0.9735090592150442, "recall": 0.9481658692185008, "support": 38874}},
  "ppcr": 0.9023825908847068
}
```
</details>
