# Train report for javascript / file:///tmp/top-repos-quality-repos-mqpjgnld/odooapps.git HEAD 9ef6c85d8b47e8ca6a1d4527e41a7f134445cd2a

### Classification report

PPCR: 0.683

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.965| 0.998| 0.947| 0.981| 0.956| 8940| 9421| 0.949 |
| `␣` | 0.988| 0.904| 0.309| 0.944| 0.471| 1814| 5308| 0.342 |
| `'` | 1.000| 1.000| 0.870| 1.000| 0.931| 1100| 1264| 0.870 |
| `⏎` | 0.969| 0.783| 0.294| 0.866| 0.451| 397| 1058| 0.375 |
| `⏎⏎` | 0.892| 0.956| 0.618| 0.923| 0.730| 225| 348| 0.647 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 36| 362| 0.099 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 27| 400| 0.068 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 17| 130| 0.131 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 11| 121| 0.091 |
| `weighted avg` | 0.963| 0.970| 0.662| 0.966| 0.728| 12567| 18412| 0.683 |
| `micro avg` | 0.970| 0.970| 0.662| 0.970| 0.787| 12567| 18412| 0.683 |
| `macro avg` | 0.535| 0.516| 0.338| 0.524| 0.393| 12567| 18412| 0.683 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|481 |8920 |20 |0 |0 |0 |0 |0 |0 |0 |
|3494 |171 |1639 |0 |0 |0 |0 |4 |0 |0 |
|164 |0 |0 |1100 |0 |0 |0 |0 |0 |0 |
|661 |65 |0 |0 |311 |0 |0 |21 |0 |0 |
|373 |27 |0 |0 |0 |0 |0 |0 |0 |0 |
|326 |36 |0 |0 |0 |0 |0 |0 |0 |0 |
|123 |0 |0 |0 |10 |0 |0 |215 |0 |0 |
|113 |17 |0 |0 |0 |0 |0 |0 |0 |0 |
|110 |10 |0 |0 |0 |0 |0 |1 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| static/apps/www/lib/underscore/underscore.js | 85 |
| static/apps/www/app/detail/detail.controller.js | 54 |
| static/apps/www/lib/buche/gulp/build.js | 53 |
| static/apps/www/lib/odoo/gulp/build.js | 24 |
| static/apps/www/lib/odoo/src/components/odoo/jsonRpc-service.js | 22 |
| static/apps/www/lib/buche/src/components/register/register.directive.js | 22 |
| static/apps/www/app/print/print.controller.js | 15 |
| static/apps/www/app/list/list.controller.js | 15 |
| static/apps/www/lib/buche/src/components/login/login.directive.js | 12 |
| static/apps/www/app/index.js | 11 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1100}, "macro avg": {"f1-score": 0.5237632220729029, "precision": 0.5348499543718002, "recall": 0.5155802054017131, "support": 12567}, "micro avg": {"f1-score": 0.969602928304289, "precision": 0.969602928304289, "recall": 0.969602928304289, "support": 12567}, "weighted avg": {"f1-score": 0.9655125917891567, "precision": 0.9630205333239095, "recall": 0.969602928304289, "support": 12567}, "\u2205": {"f1-score": 0.9809743758935443, "precision": 0.9647415098420938, "recall": 0.9977628635346756, "support": 8940}, "\u23ce": {"f1-score": 0.8662952646239553, "precision": 0.9688473520249221, "recall": 0.783375314861461, "support": 397}, "\u23ce\u23ce": {"f1-score": 0.9227467811158798, "precision": 0.8921161825726142, "recall": 0.9555555555555556, "support": 225}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 27}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 17}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 36}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u2423": {"f1-score": 0.9438525770227469, "precision": 0.9879445449065702, "recall": 0.9035281146637266, "support": 1814}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9306260575296108, "precision": 1.0, "recall": 0.870253164556962, "support": 1264}, "macro avg": {"f1-score": 0.3931032779153816, "precision": 0.5348499543718002, "recall": 0.3375133600376502, "support": 18412}, "micro avg": {"f1-score": 0.7866619322767037, "precision": 0.969602928304289, "recall": 0.6617966543558549, "support": 18412}, "weighted avg": {"f1-score": 0.728255443784279, "precision": 0.9196359080254224, "recall": 0.6617966543558549, "support": 18412}, "\u2205": {"f1-score": 0.9556972196925055, "precision": 0.9647415098420938, "recall": 0.9468209319605138, "support": 9421}, "\u23ce": {"f1-score": 0.4510514865844815, "precision": 0.9688473520249221, "recall": 0.2939508506616257, "support": 1058}, "\u23ce\u23ce": {"f1-score": 0.7300509337860782, "precision": 0.8921161825726142, "recall": 0.617816091954023, "support": 348}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 400}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 130}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 362}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 121}, "\u2423": {"f1-score": 0.4705038036457586, "precision": 0.9879445449065702, "recall": 0.3087792012057272, "support": 5308}},
  "ppcr": 0.6825439930480122
}
```
</details>
