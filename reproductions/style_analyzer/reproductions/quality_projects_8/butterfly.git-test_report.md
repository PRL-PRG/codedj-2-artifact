# Test report for javascript / file:///tmp/top-repos-quality-repos-j7qjpcaa/butterfly.git HEAD 5fa9f561ce4cae64997e2a90be2284b239c72fe4

### Classification report

PPCR: 0.754

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.844| 0.993| 0.916| 0.913| 0.879| 1066| 1156| 0.922 |
| `␣` | 0.787| 0.463| 0.212| 0.583| 0.334| 160| 349| 0.458 |
| `"` | 1.000| 1.000| 0.500| 1.000| 0.667| 96| 192| 0.500 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 59| 103| 0.573 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 35| 41| 0.854 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 17| 51| 0.333 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 11| 23| 0.478 |
| `weighted avg` | 0.777| 0.851| 0.642| 0.805| 0.658| 1444| 1915| 0.754 |
| `micro avg` | 0.851| 0.851| 0.642| 0.851| 0.732| 1444| 1915| 0.754 |
| `macro avg` | 0.376| 0.351| 0.233| 0.357| 0.269| 1444| 1915| 0.754 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|90 |1059 |7 |0 |0 |0 |0 |0 |
|189 |86 |74 |0 |0 |0 |0 |0 |
|96 |0 |0 |96 |0 |0 |0 |0 |
|44 |48 |11 |0 |0 |0 |0 |0 |
|34 |15 |2 |0 |0 |0 |0 |0 |
|6 |35 |0 |0 |0 |0 |0 |0 |
|12 |11 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 96}, "macro avg": {"f1-score": 0.3565154571195842, "precision": 0.3759616643155277, "recall": 0.3508476279817743, "support": 1444}, "micro avg": {"f1-score": 0.8511080332409973, "precision": 0.8511080332409973, "recall": 0.8511080332409973, "support": 1444}, "weighted avg": {"f1-score": 0.8049950340826272, "precision": 0.7771412026103021, "recall": 0.8511080332409973, "support": 1444}, "\u2205": {"f1-score": 0.9129310344827587, "precision": 0.8444976076555024, "recall": 0.9934333958724203, "support": 1066}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 59}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 17}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 35}, "\u2423": {"f1-score": 0.5826771653543308, "precision": 0.7872340425531915, "recall": 0.4625, "support": 160}},
  "cl_report_full": {"\"": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 192}, "macro avg": {"f1-score": 0.2685129456745092, "precision": 0.3759616643155277, "recall": 0.23258919276458265, "support": 1915}, "micro avg": {"f1-score": 0.7317654063709437, "precision": 0.8511080332409973, "recall": 0.64177545691906, "support": 1915}, "weighted avg": {"f1-score": 0.6582417056162734, "precision": 0.7535164048568276, "recall": 0.64177545691906, "support": 1915}, "\u2205": {"f1-score": 0.878838174273859, "precision": 0.8444976076555024, "recall": 0.9160899653979239, "support": 1156}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 103}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 23}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 51}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 41}, "\u2423": {"f1-score": 0.3340857787810384, "precision": 0.7872340425531915, "recall": 0.21203438395415472, "support": 349}},
  "ppcr": 0.7540469973890339
}
```
</details>
