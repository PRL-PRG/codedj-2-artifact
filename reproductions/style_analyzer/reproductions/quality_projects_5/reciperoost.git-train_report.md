# Train report for javascript / file:///tmp/top-repos-quality-repos-t0igxwbp/reciperoost.git HEAD dd397d9dad2548a66bc84a91c3d0d57d10cda5cb

### Classification report

PPCR: 0.622

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.930| 0.996| 0.793| 0.962| 0.856| 4608| 5784| 0.797 |
| `'` | 1.000| 1.000| 0.968| 1.000| 0.984| 906| 936| 0.968 |
| `␣` | 0.992| 0.829| 0.260| 0.903| 0.413| 755| 2404| 0.314 |
| `⏎␣⁺␣⁺` | 0.911| 0.599| 0.420| 0.723| 0.575| 272| 388| 0.701 |
| `"` | 1.000| 1.000| 1.000| 1.000| 1.000| 200| 200| 1.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 59| 708| 0.083 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 45| 383| 0.117 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 206| 0.019 |
| `micro avg` | 0.947| 0.947| 0.589| 0.947| 0.726| 6849| 11009| 0.622 |
| `macro avg` | 0.604| 0.553| 0.430| 0.573| 0.478| 6849| 11009| 0.622 |
| `weighted avg` | 0.933| 0.947| 0.589| 0.937| 0.662| 6849| 11009| 0.622 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| "| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1176 |4588 |4 |0 |0 |0 |16 |0 |0 |
|1649 |129 |626 |0 |0 |0 |0 |0 |0 |
|30 |0 |0 |906 |0 |0 |0 |0 |0 |
|649 |59 |0 |0 |0 |0 |0 |0 |0 |
|338 |45 |0 |0 |0 |0 |0 |0 |0 |
|116 |108 |1 |0 |0 |0 |163 |0 |0 |
|0 |0 |0 |0 |0 |0 |0 |200 |0 |
|202 |4 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| client/components/EditRecipe.js | 31 |
| client/components/AllRecipes.js | 27 |
| client/store/user.spec.js | 26 |
| server/db/models/user.spec.js | 24 |
| client/components/AddRecipe.js | 21 |
| server/api/users.spec.js | 21 |
| client/components/RecipeDetails.js | 18 |
| script/encrypt-heroku-auth-token.js | 13 |
| client/routes.js | 13 |
| client/components/navbar.js | 12 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 200}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 906}, "macro avg": {"f1-score": 0.5734876365971691, "precision": 0.604094179619272, "recall": 0.5530079376190322, "support": 6849}, "micro avg": {"f1-score": 0.9465615418309242, "precision": 0.9465615418309242, "recall": 0.9465615418309242, "support": 6849}, "weighted avg": {"f1-score": 0.9368279072785053, "precision": 0.932754294029392, "recall": 0.9465615418309242, "support": 6849}, "\u2205": {"f1-score": 0.9617440519861651, "precision": 0.9300628420839245, "recall": 0.9956597222222222, "support": 4608}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 59}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7228381374722838, "precision": 0.9106145251396648, "recall": 0.5992647058823529, "support": 272}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 45}, "\u2423": {"f1-score": 0.9033189033189033, "precision": 0.9920760697305864, "recall": 0.8291390728476821, "support": 755}},
  "cl_report_full": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 200}, "\u0027": {"f1-score": 0.9837133550488599, "precision": 1.0, "recall": 0.967948717948718, "support": 936}, "macro avg": {"f1-score": 0.4784249520766138, "precision": 0.604094179619272, "recall": 0.43020922855487453, "support": 11009}, "micro avg": {"f1-score": 0.7260611490648449, "precision": 0.9465615418309242, "recall": 0.5888818239622128, "support": 11009}, "weighted avg": {"f1-score": 0.6619906759442822, "precision": 0.8405625203015659, "recall": 0.5888818239622128, "support": 11009}, "\u2205": {"f1-score": 0.856209760194084, "precision": 0.9300628420839245, "recall": 0.793222683264177, "support": 5784}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 708}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 206}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.5749559082892416, "precision": 0.9106145251396648, "recall": 0.42010309278350516, "support": 388}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 383}, "\u2423": {"f1-score": 0.41252059308072486, "precision": 0.9920760697305864, "recall": 0.26039933444259566, "support": 2404}},
  "ppcr": 0.6221273503497139
}
```
</details>
