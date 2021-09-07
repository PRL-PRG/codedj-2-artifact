# Train report for javascript / file:///tmp/top-repos-quality-repos-81j8nk6s/apprend.git HEAD 8b2249d46328e6707f929d458d345408193eabdd

### Classification report

PPCR: 0.847

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.976| 0.979| 0.941| 0.977| 0.958| 20686| 21532| 0.961 |
| `␣` | 0.942| 0.960| 0.832| 0.951| 0.883| 9407| 10853| 0.867 |
| `'` | 0.964| 0.968| 0.838| 0.966| 0.897| 2748| 3173| 0.866 |
| `"` | 0.963| 0.958| 0.873| 0.961| 0.916| 2387| 2621| 0.911 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.929| 0.893| 0.818| 0.910| 0.870| 1427| 1557| 0.917 |
| `⏎` | 0.925| 0.885| 0.418| 0.905| 0.575| 1339| 2838| 0.472 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 90| 1613| 0.056 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 54| 861| 0.063 |
| `micro avg` | 0.962| 0.962| 0.815| 0.962| 0.882| 38138| 45048| 0.847 |
| `macro avg` | 0.712| 0.705| 0.590| 0.709| 0.637| 38138| 45048| 0.847 |
| `weighted avg` | 0.958| 0.962| 0.815| 0.960| 0.853| 38138| 45048| 0.847 |

### Confusion matrix

|refusal|  ∅| ␣| "| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|846 |20257 |337 |0 |0 |2 |0 |90 |0 |
|1446 |285 |9029 |0 |0 |88 |0 |5 |0 |
|234 |0 |0 |2287 |100 |0 |0 |0 |0 |
|425 |0 |0 |88 |2660 |0 |0 |0 |0 |
|1499 |47 |104 |0 |0 |1185 |0 |3 |0 |
|1523 |14 |70 |0 |0 |6 |0 |0 |0 |
|130 |144 |9 |0 |0 |0 |0 |1274 |0 |
|807 |14 |40 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| apprend/Server/routes/api/v1/decks/decks.js | 123 |
| apprend/Clients/apprend-web/src/components/shared/actions/actions.js | 105 |
| apprend/Server/seed.js | 102 |
| apprend/Clients/apprend-web/src/__tests__/apiTests/v1/users/users.test.js | 51 |
| apprend/Clients/apprend-web/src/components/playing/actions.js | 46 |
| apprend/Server/routes/api/v1/users/users.js | 45 |
| apprend/Server/routes/api/v1/v1.js | 38 |
| apprend/Clients/apprend-web/src/components/playing/reducers/playing.js | 37 |
| apprend/Server/database/models/deck.js | 37 |
| apprend/Clients/apprend-web/src/components/view-deck/subcomponents/flashcardTable/actions.js | 35 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9605207895842083, "precision": 0.9629473684210527, "recall": 0.9581064097193129, "support": 2387}, "\u0027": {"f1-score": 0.9658678286129266, "precision": 0.9637681159420289, "recall": 0.9679767103347889, "support": 2748}, "macro avg": {"f1-score": 0.7086755492629306, "precision": 0.7122086154234615, "recall": 0.7053665589419866, "support": 38138}, "micro avg": {"f1-score": 0.962085059520688, "precision": 0.962085059520688, "recall": 0.962085059520688, "support": 38138}, "weighted avg": {"f1-score": 0.96019907455968, "precision": 0.9584184944801281, "recall": 0.962085059520688, "support": 38138}, "\u2205": {"f1-score": 0.9774893237146235, "precision": 0.9757237127306007, "recall": 0.9792613361693899, "support": 20686}, "\u23ce": {"f1-score": 0.9045801526717557, "precision": 0.9250585480093677, "recall": 0.8849887976101568, "support": 1339}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 54}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 90}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9103251161128975, "precision": 0.9285714285714286, "recall": 0.8927820602662929, "support": 1427}, "\u2423": {"f1-score": 0.9506211834070332, "precision": 0.941599749713213, "recall": 0.959817157435952, "support": 9407}},
  "cl_report_full": {"\"": {"f1-score": 0.9155324259407527, "precision": 0.9629473684210527, "recall": 0.8725677222434185, "support": 2621}, "\u0027": {"f1-score": 0.8966795887409404, "precision": 0.9637681159420289, "recall": 0.8383233532934131, "support": 3173}, "macro avg": {"f1-score": 0.6373536907253367, "precision": 0.7122086154234615, "recall": 0.5899250659009553, "support": 45048}, "micro avg": {"f1-score": 0.8821676724448826, "precision": 0.962085059520688, "recall": 0.8145089682116853, "support": 45048}, "weighted avg": {"f1-score": 0.853439812517823, "precision": 0.9075095059537547, "recall": 0.8145089682116853, "support": 45048}, "\u2205": {"f1-score": 0.9579363015156173, "precision": 0.9757237127306007, "recall": 0.9407858071707227, "support": 21532}, "\u23ce": {"f1-score": 0.5753823743627093, "precision": 0.9250585480093677, "recall": 0.4175475687103594, "support": 2838}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 861}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1613}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.8699214749061114, "precision": 0.9285714285714286, "recall": 0.8182402055234426, "support": 1557}, "\u2423": {"f1-score": 0.883377360336562, "precision": 0.941599749713213, "recall": 0.8319358702662858, "support": 10853}},
  "ppcr": 0.8466080625110993
}
```
</details>
