# Train report for javascript / file:///tmp/top-repos-quality-repos-ru2mv3a9/frozen-island.git HEAD f227b26797ba5667def2ad30a8b84359967d9564

### Classification report

PPCR: 0.493

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.971| 1.000| 0.826| 0.985| 0.893| 1948| 2359| 0.826 |
| `␣` | 0.987| 0.936| 0.254| 0.961| 0.404| 314| 1157| 0.271 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 18| 161| 0.112 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 14| 153| 0.092 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 269| 0.037 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 422| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 124| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 32| 0.000 |
| `weighted avg` | 0.955| 0.973| 0.479| 0.964| 0.550| 2304| 4677| 0.493 |
| `macro avg` | 0.245| 0.242| 0.135| 0.243| 0.162| 2304| 4677| 0.493 |
| `micro avg` | 0.973| 0.973| 0.479| 0.973| 0.642| 2304| 4677| 0.493 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|411 |1948 |0 |0 |0 |0 |0 |0 |0 |
|843 |20 |294 |0 |0 |0 |0 |0 |0 |
|422 |0 |0 |0 |0 |0 |0 |0 |0 |
|259 |10 |0 |0 |0 |0 |0 |0 |0 |
|143 |14 |4 |0 |0 |0 |0 |0 |0 |
|139 |14 |0 |0 |0 |0 |0 |0 |0 |
|124 |0 |0 |0 |0 |0 |0 |0 |0 |
|32 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| client/components/auth-form.js | 7 |
| client/components/navbar.js | 7 |
| server/auth/google.js | 6 |
| client/routes.js | 6 |
| client/app.js | 5 |
| server/db/models/user.js | 4 |
| server/db/db.js | 3 |
| server/index.js | 3 |
| client/index.js | 3 |
| client/store/index.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.2432644529739058, "precision": 0.24470799012358896, "recall": 0.24203821656050956, "support": 2304}, "micro avg": {"f1-score": 0.9730902777777778, "precision": 0.9730902777777778, "recall": 0.9730902777777778, "support": 2304}, "weighted avg": {"f1-score": 0.9640241608150595, "precision": 0.9554957482604388, "recall": 0.9730902777777778, "support": 2304}, "\u2205": {"f1-score": 0.9853313100657561, "precision": 0.9710867397806581, "recall": 1.0, "support": 1948}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 18}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u2423": {"f1-score": 0.9607843137254902, "precision": 0.9865771812080537, "recall": 0.9363057324840764, "support": 314}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 32}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 422}, "macro avg": {"f1-score": 0.1620847651775487, "precision": 0.24470799012358896, "recall": 0.13498488475149695, "support": 4677}, "micro avg": {"f1-score": 0.6423148546053574, "precision": 0.9730902777777778, "recall": 0.4793671156724396, "support": 4677}, "weighted avg": {"f1-score": 0.5501618531964445, "precision": 0.7338600422921298, "recall": 0.4793671156724396, "support": 4677}, "\u2205": {"f1-score": 0.8925544100801833, "precision": 0.9710867397806581, "recall": 0.8257736328952946, "support": 2359}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 269}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 124}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 161}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 153}, "\u2423": {"f1-score": 0.4041237113402062, "precision": 0.9865771812080537, "recall": 0.2541054451166811, "support": 1157}},
  "ppcr": 0.4926234765875561
}
```
</details>
