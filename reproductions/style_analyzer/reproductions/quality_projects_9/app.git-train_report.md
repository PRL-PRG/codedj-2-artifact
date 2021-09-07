# Train report for javascript / file:///tmp/top-repos-quality-repos-2hhfqoh6/app.git HEAD 0954093a6fa73a2f79926a7e5e1a3857fe09e3ea

### Classification report

PPCR: 0.854

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.954| 0.984| 0.961| 0.969| 0.958| 30632| 31364| 0.977 |
| `␣` | 0.927| 0.970| 0.868| 0.948| 0.897| 15318| 17112| 0.895 |
| `'` | 1.000| 1.000| 0.813| 1.000| 0.897| 3932| 4837| 0.813 |
| `⏎` | 0.915| 0.738| 0.344| 0.817| 0.500| 1673| 3588| 0.466 |
| `⏎␣⁻␣⁻` | 0.969| 0.349| 0.202| 0.513| 0.334| 1177| 2037| 0.578 |
| `⏎⏎` | 0.966| 0.927| 0.512| 0.946| 0.669| 668| 1210| 0.552 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 449| 2171| 0.207 |
| `"` | 1.000| 1.000| 0.340| 1.000| 0.507| 272| 801| 0.340 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 76| 189| 0.402 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 22| 155| 0.142 |
| `weighted avg` | 0.940| 0.949| 0.811| 0.941| 0.841| 54219| 63464| 0.854 |
| `macro avg` | 0.673| 0.597| 0.404| 0.619| 0.476| 54219| 63464| 0.854 |
| `micro avg` | 0.949| 0.949| 0.811| 0.949| 0.875| 54219| 63464| 0.854 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|732 |30131 |495 |0 |0 |0 |6 |0 |0 |0 |0 |
|1794 |387 |14859 |0 |70 |0 |2 |0 |0 |0 |0 |
|905 |0 |0 |3932 |0 |0 |0 |0 |0 |0 |0 |
|1915 |205 |213 |0 |1234 |0 |0 |21 |0 |0 |0 |
|1722 |236 |213 |0 |0 |0 |0 |0 |0 |0 |0 |
|860 |541 |224 |0 |1 |0 |411 |0 |0 |0 |0 |
|542 |0 |5 |0 |44 |0 |0 |619 |0 |0 |0 |
|529 |0 |0 |0 |0 |0 |0 |0 |272 |0 |0 |
|113 |67 |3 |0 |0 |0 |5 |1 |0 |0 |0 |
|133 |4 |18 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/api/manager/resolvers.js | 161 |
| src/components/SingleName/DetailsItemEditable.js | 133 |
| src/components/SingleName/NameDetails.js | 131 |
| src/components/SingleName/SubDomains.js | 82 |
| src/components/SingleName/ResolverAndRecords/ResolverAndRecords.js | 77 |
| cypress/integration/nameDetail.spec.js | 75 |
| src/components/SingleName/ResolverAndRecords/AddRecord.js | 70 |
| src/utils/utils.js | 68 |
| src/components/SingleName/ResolverAndRecords/RecordsItem.js | 61 |
| src/components/SingleName/ResolverAndRecords/KeyValueRecord/KeyValueRecord.js | 58 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 272}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 3932}, "macro avg": {"f1-score": 0.6192665705573122, "precision": 0.6731107908544122, "recall": 0.5967116508058924, "support": 54219}, "micro avg": {"f1-score": 0.9490768918644755, "precision": 0.9490768918644755, "recall": 0.9490768918644755, "support": 54219}, "weighted avg": {"f1-score": 0.9407041245282708, "precision": 0.9397850708312769, "recall": 0.9490768918644755, "support": 54219}, "\u2205": {"f1-score": 0.9687957172483642, "precision": 0.9543885211111464, "recall": 0.9836445547140246, "support": 30632}, "\u23ce": {"f1-score": 0.8166776968894772, "precision": 0.9147516679021498, "recall": 0.7375971309025703, "support": 1673}, "\u23ce\u23ce": {"f1-score": 0.9457601222307105, "precision": 0.9656786271450858, "recall": 0.9266467065868264, "support": 668}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 449}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.5134291068082448, "precision": 0.9693396226415094, "recall": 0.3491928632115548, "support": 1177}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 76}, "\u2423": {"f1-score": 0.948003062396325, "precision": 0.9269494697442295, "recall": 0.9700352526439483, "support": 15318}},
  "cl_report_full": {"\"": {"f1-score": 0.5069897483690587, "precision": 1.0, "recall": 0.33957553058676654, "support": 801}, "\u0027": {"f1-score": 0.8967955297069221, "precision": 1.0, "recall": 0.8129005581972297, "support": 4837}, "macro avg": {"f1-score": 0.4760736818665795, "precision": 0.6731107908544122, "recall": 0.4038763254063252, "support": 63464}, "micro avg": {"f1-score": 0.8745188344960614, "precision": 0.9490768918644755, "recall": 0.8108218832724065, "support": 63464}, "weighted avg": {"f1-score": 0.8414723296294963, "precision": 0.9116750573079615, "recall": 0.8108218832724065, "support": 63464}, "\u2205": {"f1-score": 0.9575276078493684, "precision": 0.9543885211111464, "recall": 0.9606874123198572, "support": 31364}, "\u23ce": {"f1-score": 0.49989872392140977, "precision": 0.9147516679021498, "recall": 0.3439241917502787, "support": 3588}, "\u23ce\u23ce": {"f1-score": 0.668827660723933, "precision": 0.9656786271450858, "recall": 0.5115702479338843, "support": 1210}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2171}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 155}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.33401056481105246, "precision": 0.9693396226415094, "recall": 0.20176730486008837, "support": 2037}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 189}, "\u2423": {"f1-score": 0.8966869832840504, "precision": 0.9269494697442295, "recall": 0.8683380084151473, "support": 17112}},
  "ppcr": 0.8543268624732132
}
```
</details>
