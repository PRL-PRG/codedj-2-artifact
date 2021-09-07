# Train report for javascript / file:///tmp/top-repos-quality-repos-r7nesld2/fetch.git HEAD ed6e4dae93c33eef6203f02c86a730e8a053a2c0

### Classification report

PPCR: 0.783

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.932| 0.994| 0.945| 0.962| 0.939| 12167| 12792| 0.951 |
| `␣` | 0.952| 0.886| 0.512| 0.918| 0.666| 3212| 5558| 0.578 |
| `'` | 1.000| 1.000| 0.999| 1.000| 1.000| 1870| 1871| 0.999 |
| `⏎` | 0.940| 0.786| 0.274| 0.856| 0.424| 618| 1775| 0.348 |
| `"` | 1.000| 1.000| 1.000| 1.000| 1.000| 560| 560| 1.000 |
| `⏎␣⁺␣⁺` | 0.949| 0.589| 0.351| 0.727| 0.512| 438| 735| 0.596 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 250| 723| 0.346 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 67| 487| 0.138 |
| `weighted avg` | 0.929| 0.944| 0.739| 0.935| 0.786| 19182| 24501| 0.783 |
| `micro avg` | 0.944| 0.944| 0.739| 0.944| 0.829| 19182| 24501| 0.783 |
| `macro avg` | 0.722| 0.657| 0.510| 0.683| 0.568| 19182| 24501| 0.783 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|625 |12093 |55 |0 |5 |14 |0 |0 |0 |
|2346 |341 |2846 |0 |25 |0 |0 |0 |0 |
|1 |0 |0 |1870 |0 |0 |0 |0 |0 |
|1157 |120 |12 |0 |486 |0 |0 |0 |0 |
|297 |103 |77 |0 |0 |258 |0 |0 |0 |
|473 |250 |0 |0 |0 |0 |0 |0 |0 |
|0 |0 |0 |0 |0 |0 |0 |560 |0 |
|420 |66 |0 |0 |1 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| client/routes.js | 59 |
| client/components/MenuBar.js | 48 |
| script/seed.js | 47 |
| client/store/job.js | 46 |
| script/encrypt-heroku-auth-token.js | 37 |
| client/store/chat.js | 34 |
| server/db/models/user.js | 32 |
| server/auth/linkedin.js | 28 |
| client/store/users.js | 28 |
| server/api/jobs.js | 28 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 560}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1870}, "macro avg": {"f1-score": 0.6828711495414533, "precision": 0.7215717961691251, "recall": 0.6569273926989037, "support": 19182}, "micro avg": {"f1-score": 0.9442706704201856, "precision": 0.9442706704201856, "recall": 0.9442706704201856, "support": 19182}, "weighted avg": {"f1-score": 0.9347687333545429, "precision": 0.9292764937690052, "recall": 0.9442706704201856, "support": 19182}, "\u2205": {"f1-score": 0.9620525059665871, "precision": 0.9321668079858167, "recall": 0.9939179748500041, "support": 12167}, "\u23ce": {"f1-score": 0.8563876651982378, "precision": 0.9400386847195358, "recall": 0.7864077669902912, "support": 618}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 67}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7267605633802817, "precision": 0.9485294117647058, "recall": 0.589041095890411, "support": 438}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 250}, "\u2423": {"f1-score": 0.9177684617865204, "precision": 0.9518394648829431, "recall": 0.886052303860523, "support": 3212}},
  "cl_report_full": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 560}, "\u0027": {"f1-score": 0.999732691793638, "precision": 1.0, "recall": 0.9994655264564404, "support": 1871}, "macro avg": {"f1-score": 0.5676039547843337, "precision": 0.7215717961691251, "recall": 0.5102124900313001, "support": 24501}, "micro avg": {"f1-score": 0.8292928599226244, "precision": 0.9442706704201856, "recall": 0.739275947920493, "support": 24501}, "weighted avg": {"f1-score": 0.7864543101293158, "precision": 0.8983853449491124, "recall": 0.739275947920493, "support": 24501}, "\u2205": {"f1-score": 0.9387153114690472, "precision": 0.9321668079858167, "recall": 0.9453564727954972, "support": 12792}, "\u23ce": {"f1-score": 0.42408376963350786, "precision": 0.9400386847195358, "recall": 0.27380281690140845, "support": 1775}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 487}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.5124131082423039, "precision": 0.9485294117647058, "recall": 0.3510204081632653, "support": 735}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 723}, "\u2423": {"f1-score": 0.6658867571361723, "precision": 0.9518394648829431, "recall": 0.5120546959337892, "support": 5558}},
  "ppcr": 0.7829068201297906
}
```
</details>
