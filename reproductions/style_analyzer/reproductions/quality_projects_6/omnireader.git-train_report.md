# Train report for javascript / file:///tmp/top-repos-quality-repos-_z6apyu2/omnireader.git HEAD c34d456c433aa4e0dac8b5f8e9f6d4fd6eaa36db

### Classification report

PPCR: 0.093

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `␣` | 1.000| 1.000| 0.129| 1.000| 0.229| 455| 3522| 0.129 |
| `'` | 1.000| 1.000| 0.214| 1.000| 0.353| 183| 854| 0.214 |
| `⏎` | 0.994| 0.884| 0.220| 0.936| 0.360| 181| 727| 0.249 |
| `⏎⏎` | 0.887| 0.994| 0.524| 0.938| 0.659| 166| 315| 0.527 |
| `⏎␣⁻␣⁻` | 1.000| 1.000| 0.545| 1.000| 0.705| 165| 303| 0.545 |
| `∅` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 6170| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 323| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 193| 0.000 |
| `micro avg` | 0.981| 0.981| 0.091| 0.981| 0.166| 1150| 12407| 0.093 |
| `weighted avg` | 0.983| 0.981| 0.091| 0.981| 0.144| 1150| 12407| 0.093 |
| `macro avg` | 0.610| 0.610| 0.204| 0.609| 0.288| 1150| 12407| 0.093 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎⏎| ⏎␣⁻␣⁻| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|6170 |0 |0 |0 |0 |0 |0 |0 |0 |
|3067 |0 |455 |0 |0 |0 |0 |0 |0 |
|671 |0 |0 |183 |0 |0 |0 |0 |0 |
|546 |0 |0 |0 |160 |0 |21 |0 |0 |
|323 |0 |0 |0 |0 |0 |0 |0 |0 |
|149 |0 |0 |0 |1 |0 |165 |0 |0 |
|138 |0 |0 |0 |0 |0 |0 |165 |0 |
|193 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/services/scraper/scraper.test.js | 3 |
| src/services/refresher.test.js | 3 |
| __tests__/api/posts.js | 3 |
| src/errors/index.js | 2 |
| console.js | 2 |
| src/models/index.js | 2 |
| __tests__/config.js | 1 |
| webapp/src/components/Reader/Reader.js | 1 |
| __tests__/api/stories.js | 1 |
| webapp/src/components/shared/breakpoints.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 183}, "macro avg": {"f1-score": 0.6091465643274854, "precision": 0.6101106992586656, "recall": 0.609744225520868, "support": 1150}, "micro avg": {"f1-score": 0.9808695652173913, "precision": 0.9808695652173913, "recall": 0.9808695652173913, "support": 1150}, "weighted avg": {"f1-score": 0.980853674040173, "precision": 0.9827250790553432, "recall": 0.9808695652173913, "support": 1150}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce": {"f1-score": 0.935672514619883, "precision": 0.9937888198757764, "recall": 0.8839779005524862, "support": 181}, "\u23ce\u23ce": {"f1-score": 0.9375, "precision": 0.8870967741935484, "recall": 0.9939759036144579, "support": 166}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 165}, "\u2423": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 455}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 193}, "\u0027": {"f1-score": 0.35294117647058826, "precision": 1.0, "recall": 0.21428571428571427, "support": 854}, "macro avg": {"f1-score": 0.2882410083635563, "precision": 0.6101106992586656, "recall": 0.2039900232344331, "support": 12407}, "micro avg": {"f1-score": 0.16640849745518918, "precision": 0.9808695652173913, "recall": 0.09091641815104376, "support": 12407}, "weighted avg": {"f1-score": 0.1443073654978879, "precision": 0.4578802253502585, "recall": 0.09091641815104376, "support": 12407}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6170}, "\u23ce": {"f1-score": 0.36036036036036034, "precision": 0.9937888198757764, "recall": 0.2200825309491059, "support": 727}, "\u23ce\u23ce": {"f1-score": 0.6586826347305389, "precision": 0.8870967741935484, "recall": 0.5238095238095238, "support": 315}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 323}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7051282051282052, "precision": 1.0, "recall": 0.5445544554455446, "support": 303}, "\u2423": {"f1-score": 0.22881569021875783, "precision": 1.0, "recall": 0.12918796138557637, "support": 3522}},
  "ppcr": 0.09268961070363504
}
```
</details>
