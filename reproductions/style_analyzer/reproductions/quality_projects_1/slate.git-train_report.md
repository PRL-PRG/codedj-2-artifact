# Train report for javascript / file:///tmp/top-repos-quality-repos-yah1jnco/slate.git HEAD 4f329c6890e72bea03641665569db9a93a8e0f1e

### Classification report

PPCR: 0.371

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.996| 0.994| 0.549| 0.995| 0.708| 2784| 5035| 0.553 |
| `␣` | 0.948| 0.948| 0.163| 0.948| 0.279| 466| 2707| 0.172 |
| `⏎␣⁺␣⁺` | 0.929| 0.972| 0.860| 0.950| 0.893| 215| 243| 0.885 |
| `⏎` | 0.962| 0.974| 0.256| 0.968| 0.405| 154| 585| 0.263 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 242| 0.017 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 498| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 280| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 167| 0.000 |
| `macro avg` | 0.479| 0.486| 0.229| 0.483| 0.286| 3623| 9757| 0.371 |
| `micro avg` | 0.985| 0.985| 0.366| 0.985| 0.533| 3623| 9757| 0.371 |
| `weighted avg` | 0.984| 0.985| 0.366| 0.984| 0.489| 3623| 9757| 0.371 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| '| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2251 |2766 |18 |0 |0 |0 |0 |0 |0 |
|2241 |2 |442 |0 |6 |0 |16 |0 |0 |
|498 |0 |0 |0 |0 |0 |0 |0 |0 |
|431 |4 |0 |0 |150 |0 |0 |0 |0 |
|280 |0 |0 |0 |0 |0 |0 |0 |0 |
|28 |0 |6 |0 |0 |0 |209 |0 |0 |
|238 |4 |0 |0 |0 |0 |0 |0 |0 |
|167 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| source/javascripts/lib/_lunr.js | 20 |
| source/javascripts/app/_toc.js | 14 |
| source/javascripts/app/_search.js | 7 |
| source/javascripts/app/_lang.js | 6 |
| source/javascripts/lib/_jquery.highlight.js | 5 |
| source/javascripts/lib/_energize.js | 3 |
| source/javascripts/all_nosearch.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.48265047729226285, "precision": 0.4794153623786374, "recall": 0.4860189167647077, "support": 3623}, "micro avg": {"f1-score": 0.9845431962462048, "precision": 0.9845431962462048, "recall": 0.9845431962462048, "support": 3623}, "weighted avg": {"f1-score": 0.9840635148163467, "precision": 0.983648693277458, "recall": 0.9845431962462048, "support": 3623}, "\u2205": {"f1-score": 0.9949640287769784, "precision": 0.9963976945244957, "recall": 0.9935344827586207, "support": 2784}, "\u23ce": {"f1-score": 0.967741935483871, "precision": 0.9615384615384616, "recall": 0.974025974025974, "support": 154}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9500000000000001, "precision": 0.9288888888888889, "recall": 0.9720930232558139, "support": 215}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u2423": {"f1-score": 0.9484978540772532, "precision": 0.9484978540772532, "recall": 0.9484978540772532, "support": 466}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 498}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 167}, "macro avg": {"f1-score": 0.2856066708309925, "precision": 0.4794153623786374, "recall": 0.2286409329371931, "support": 9757}, "micro avg": {"f1-score": 0.5331838565022422, "precision": 0.9845431962462048, "recall": 0.3655836835092754, "support": 9757}, "weighted avg": {"f1-score": 0.48928980892904383, "precision": 0.8581188975010721, "recall": 0.36558368350927534, "support": 9757}, "\u2205": {"f1-score": 0.7082319805402636, "precision": 0.9963976945244957, "recall": 0.5493545183714001, "support": 5035}, "\u23ce": {"f1-score": 0.40485829959514164, "precision": 0.9615384615384616, "recall": 0.2564102564102564, "support": 585}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 280}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8931623931623931, "precision": 0.9288888888888889, "recall": 0.8600823045267489, "support": 243}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 242}, "\u2423": {"f1-score": 0.2786006933501418, "precision": 0.9484978540772532, "recall": 0.16328038418913926, "support": 2707}},
  "ppcr": 0.3713231526083837
}
```
</details>
