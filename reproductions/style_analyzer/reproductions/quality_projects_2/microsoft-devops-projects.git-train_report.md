# Train report for javascript / file:///tmp/top-repos-quality-repos-aqntce2w/microsoft-devops-projects.git HEAD 2d2ad0079acaee10c8a39de951209174b593caca

### Classification report

PPCR: 0.766

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.945| 0.986| 0.922| 0.965| 0.933| 48285| 51624| 0.935 |
| `␣` | 0.952| 0.901| 0.695| 0.926| 0.803| 23514| 30471| 0.772 |
| `'` | 0.970| 1.000| 0.806| 0.985| 0.881| 9065| 11240| 0.806 |
| `⏎` | 0.912| 0.899| 0.474| 0.905| 0.624| 4490| 8512| 0.527 |
| `"` | 1.000| 0.646| 0.237| 0.785| 0.383| 780| 2125| 0.367 |
| `⏎⏎` | 0.935| 0.480| 0.122| 0.635| 0.215| 604| 2383| 0.253 |
| `⏎␣⁻␣⁻` | 0.953| 0.990| 0.477| 0.971| 0.636| 577| 1197| 0.482 |
| `⏎␣⁺␣⁺` | 0.972| 1.000| 0.400| 0.986| 0.567| 424| 1059| 0.400 |
| `⇥` | 0.000| 0.000| 0.000| 0.000| 0.000| 120| 560| 0.214 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 82| 1315| 0.062 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 80| 1202| 0.067 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 77| 1357| 0.057 |
| `⇥⇥` | 0.000| 0.000| 0.000| 0.000| 0.000| 40| 200| 0.200 |
| `⏎⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 34| 108| 0.315 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 20| 1004| 0.020 |
| `⇥⇥⇥` | 0.000| 0.000| 0.000| 0.000| 0.000| 20| 180| 0.111 |
| `␣␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 14| 110| 0.127 |
| `⏎⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 274| 0.022 |
| `⏎⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 136| 0.015 |
| `⏎⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 160| 0.000 |
| `weighted avg` | 0.943| 0.948| 0.726| 0.944| 0.786| 88234| 115217| 0.766 |
| `micro avg` | 0.948| 0.948| 0.726| 0.948| 0.822| 88234| 115217| 0.766 |
| `macro avg` | 0.382| 0.345| 0.207| 0.358| 0.252| 88234| 115217| 0.766 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁻␣⁻| ⏎⇥⁺| ⏎␣⁺␣⁺| ⏎⇥⁻| ⇥| ⏎⏎␣⁺␣⁺| ⇥⇥| ⇥⇥⇥| ⏎⏎⇥⁻| ⏎⏎␣⁻␣⁻| ␣␣| ⏎⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3339 |47591 |661 |0 |25 |0 |0 |0 |0 |4 |0 |4 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|6957 |2256 |21176 |0 |82 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2175 |0 |0 |9065 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|4022 |240 |201 |0 |4035 |0 |14 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1345 |0 |0 |276 |0 |504 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1779 |99 |0 |0 |215 |0 |290 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1280 |14 |49 |0 |6 |0 |0 |0 |0 |0 |0 |8 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1233 |58 |0 |0 |0 |0 |0 |0 |0 |24 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|620 |0 |6 |0 |0 |0 |0 |0 |0 |571 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1122 |20 |40 |0 |20 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|635 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |424 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|984 |0 |0 |0 |20 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|440 |80 |40 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|268 |0 |0 |0 |0 |0 |6 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|160 |0 |40 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|160 |0 |20 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|160 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|134 |2 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|96 |0 |8 |0 |6 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|74 |20 |0 |0 |14 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| python/flask/container/Application/python_webapp_flask/static/scripts/respond.js | 109 |
| python/django/webapp/Application/app/static/app/scripts/respond.js | 109 |
| python/django/webapp/Application/static/app/scripts/respond.js | 109 |
| dotnet/aspnet46/vm-webapp/Application/SampleWebApplication/Scripts/respond.js | 109 |
| python/bottle/webappWithTests/Application/static/scripts/respond.js | 109 |
| python/flask/webapp/Application/python_webapp_flask/static/scripts/respond.js | 109 |
| dotnet/aspnet/mssqldb/Application/SampleWebApplication/Scripts/respond.js | 109 |
| python/bottle/containerWithTests/Application/static/scripts/respond.js | 109 |
| python/django/webappWithTests/Application/static/app/scripts/respond.js | 109 |
| dotnet/aspnet46/vm-mssqldb/Application/SampleWebApplication/Scripts/respond.js | 109 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.7850467289719627, "precision": 1.0, "recall": 0.6461538461538462, "support": 780}, "\u0027": {"f1-score": 0.9850048897098772, "precision": 0.970452842308104, "recall": 1.0, "support": 9065}, "macro avg": {"f1-score": 0.3578751631686029, "precision": 0.3820351065667974, "recall": 0.34503741298275514, "support": 88234}, "micro avg": {"f1-score": 0.9481152390235058, "precision": 0.9481152390235057, "recall": 0.9481152390235057, "support": 88234}, "weighted avg": {"f1-score": 0.9442398635675741, "precision": 0.942954973412764, "recall": 0.9481152390235057, "support": 88234}, "\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 120}, "\u21e5\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 40}, "\u21e5\u21e5\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 20}, "\u2205": {"f1-score": 0.9646987280190544, "precision": 0.9446407304485908, "recall": 0.9856270063166614, "support": 48285}, "\u23ce": {"f1-score": 0.9054190508246382, "precision": 0.9122767352475695, "recall": 0.8986636971046771, "support": 4490}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 80}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 20}, "\u23ce\u23ce": {"f1-score": 0.6345733041575493, "precision": 0.9354838709677419, "recall": 0.48013245033112584, "support": 604}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 34}, "\u23ce\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.986046511627907, "precision": 0.9724770642201835, "recall": 1.0, "support": 424}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 77}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9710884353741497, "precision": 0.9532554257095158, "recall": 0.9896013864818024, "support": 577}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 82}, "\u2423": {"f1-score": 0.9256256146869194, "precision": 0.952115462434243, "recall": 0.9005698732669899, "support": 23514}, "\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}},
  "cl_report_full": {"\"": {"f1-score": 0.3834157474324838, "precision": 1.0, "recall": 0.2371764705882353, "support": 2125}, "\u0027": {"f1-score": 0.8809095767941304, "precision": 0.970452842308104, "recall": 0.8064946619217082, "support": 11240}, "macro avg": {"f1-score": 0.2521624655922717, "precision": 0.3820351065667974, "recall": 0.2066820011289626, "support": 115217}, "micro avg": {"f1-score": 0.8223700055541628, "precision": 0.9481152390235057, "recall": 0.7260734093059183, "support": 115217}, "weighted avg": {"f1-score": 0.7859552712542452, "precision": 0.8937603900690165, "recall": 0.7260734093059183, "support": 115217}, "\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 560}, "\u21e5\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 200}, "\u21e5\u21e5\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 180}, "\u2205": {"f1-score": 0.9331202697933414, "precision": 0.9446407304485908, "recall": 0.9218774213544088, "support": 51624}, "\u23ce": {"f1-score": 0.6238886741399304, "precision": 0.9122767352475695, "recall": 0.47403665413533835, "support": 8512}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1202}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1004}, "\u23ce\u23ce": {"f1-score": 0.21537318975120684, "precision": 0.9354838709677419, "recall": 0.12169534200587495, "support": 2383}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 160}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 108}, "\u23ce\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 274}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 136}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.5672240802675586, "precision": 0.9724770642201835, "recall": 0.4003777148253069, "support": 1059}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1357}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.6358574610244989, "precision": 0.9532554257095158, "recall": 0.47702589807852963, "support": 1197}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1315}, "\u2423": {"f1-score": 0.8034603126422826, "precision": 0.952115462434243, "recall": 0.6949558596698501, "support": 30471}, "\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 110}},
  "ppcr": 0.7658071291562877
}
```
</details>
