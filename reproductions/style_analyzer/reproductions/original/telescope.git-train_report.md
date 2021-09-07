# Train report for javascript / file:///tmp/top-repos-quality-repos-oknrmilz/telescope.git HEAD e82daf606bf4ac64c445b5bd58f80784dd16f97a

### Classification report

PPCR: 0.633

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.973| 1.000| 0.858| 0.987| 0.912| 695| 810| 0.858 |
| `'` | 1.000| 1.000| 0.861| 1.000| 0.925| 359| 417| 0.861 |
| `␣` | 0.990| 0.912| 0.210| 0.949| 0.347| 113| 490| 0.231 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.987| 0.975| 0.975| 0.981| 0.981| 79| 79| 1.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.986| 0.948| 0.924| 0.967| 0.954| 77| 79| 0.975 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 152| 0.039 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 71| 0.000 |
| `weighted avg` | 0.979| 0.983| 0.623| 0.981| 0.690| 1329| 2098| 0.633 |
| `macro avg` | 0.705| 0.691| 0.547| 0.698| 0.588| 1329| 2098| 0.633 |
| `micro avg` | 0.983| 0.983| 0.623| 0.983| 0.763| 1329| 2098| 0.633 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|115 |695 |0 |0 |0 |0 |0 |0 |
|377 |8 |103 |0 |0 |1 |1 |0 |
|58 |0 |0 |359 |0 |0 |0 |0 |
|146 |6 |0 |0 |0 |0 |0 |0 |
|0 |1 |1 |0 |0 |77 |0 |0 |
|2 |4 |0 |0 |0 |0 |73 |0 |
|71 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| webpack.mix.js | 13 |
| resources/js/base.js | 3 |
| resources/js/routes.js | 3 |
| resources/js/app.js | 3 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 359}, "macro avg": {"f1-score": 0.6976575931104437, "precision": 0.7053485635418408, "recall": 0.6906057024477866, "support": 1329}, "micro avg": {"f1-score": 0.9834462001504891, "precision": 0.9834462001504891, "recall": 0.9834462001504891, "support": 1329}, "weighted avg": {"f1-score": 0.9810690529914649, "precision": 0.9792066988156484, "recall": 0.9834462001504891, "support": 1329}, "\u2205": {"f1-score": 0.9865152590489709, "precision": 0.9733893557422969, "recall": 1.0, "support": 695}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.980891719745223, "precision": 0.9871794871794872, "recall": 0.9746835443037974, "support": 79}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9668874172185431, "precision": 0.9864864864864865, "recall": 0.948051948051948, "support": 77}, "\u2423": {"f1-score": 0.9493087557603687, "precision": 0.9903846153846154, "recall": 0.911504424778761, "support": 113}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9252577319587628, "precision": 1.0, "recall": 0.8609112709832134, "support": 417}, "macro avg": {"f1-score": 0.5884675221902933, "precision": 0.7053485635418408, "recall": 0.546839174455583, "support": 2098}, "micro avg": {"f1-score": 0.762766267872775, "precision": 0.9834462001504891, "recall": 0.622974261201144, "support": 2098}, "weighted avg": {"f1-score": 0.689905018347676, "precision": 0.8801970694038771, "recall": 0.622974261201144, "support": 2098}, "\u2205": {"f1-score": 0.9120734908136484, "precision": 0.9733893557422969, "recall": 0.8580246913580247, "support": 810}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 152}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 71}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.980891719745223, "precision": 0.9871794871794872, "recall": 0.9746835443037974, "support": 79}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.954248366013072, "precision": 0.9864864864864865, "recall": 0.9240506329113924, "support": 79}, "\u2423": {"f1-score": 0.3468013468013468, "precision": 0.9903846153846154, "recall": 0.21020408163265306, "support": 490}},
  "ppcr": 0.6334604385128694
}
```
</details>
