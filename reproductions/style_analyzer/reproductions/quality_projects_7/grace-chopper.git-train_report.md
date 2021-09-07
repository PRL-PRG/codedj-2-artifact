# Train report for javascript / file:///tmp/top-repos-quality-repos-6ewo8n2p/grace-chopper.git HEAD 99cde130c3d8fdf411065add3365c3ffe7712d5c

### Classification report

PPCR: 0.562

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.961| 0.986| 0.784| 0.974| 0.864| 4794| 6032| 0.795 |
| `␣` | 0.974| 0.928| 0.446| 0.950| 0.612| 1410| 2935| 0.480 |
| `⏎␣⁻␣⁻` | 0.937| 0.878| 0.728| 0.906| 0.819| 353| 426| 0.829 |
| `⏎␣⁺␣⁺` | 0.858| 0.819| 0.408| 0.838| 0.553| 221| 444| 0.498 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 18| 764| 0.024 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 248| 0.028 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 989| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 271| 0.000 |
| `micro avg` | 0.959| 0.959| 0.539| 0.959| 0.690| 6803| 12109| 0.562 |
| `weighted avg` | 0.956| 0.959| 0.539| 0.957| 0.627| 6803| 12109| 0.562 |
| `macro avg` | 0.466| 0.451| 0.296| 0.459| 0.356| 6803| 12109| 0.562 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1238 |4728 |15 |0 |0 |30 |21 |0 |0 |
|1525 |102 |1308 |0 |0 |0 |0 |0 |0 |
|989 |0 |0 |0 |0 |0 |0 |0 |0 |
|746 |18 |0 |0 |0 |0 |0 |0 |0 |
|223 |21 |19 |0 |0 |181 |0 |0 |0 |
|73 |42 |1 |0 |0 |0 |310 |0 |0 |
|271 |0 |0 |0 |0 |0 |0 |0 |0 |
|241 |7 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| script/seed.js | 24 |
| client/components/navbar.js | 18 |
| client/routes.js | 16 |
| client/components/SingleProduct.js | 15 |
| server/db/models/user.js | 14 |
| client/components/CreateAccount.js | 14 |
| server/auth/google.js | 12 |
| client/components/Cart.js | 9 |
| client/utils/changeQuantity.js | 9 |
| script/encrypt-heroku-auth-token.js | 9 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.4585340842799053, "precision": 0.4662101435283761, "recall": 0.4513854823977782, "support": 6803}, "micro avg": {"f1-score": 0.9594296633838013, "precision": 0.9594296633838013, "recall": 0.9594296633838013, "support": 6803}, "weighted avg": {"f1-score": 0.957317029470568, "precision": 0.9557881674391324, "recall": 0.9594296633838013, "support": 6803}, "\u2205": {"f1-score": 0.9736408566721582, "precision": 0.9613664091093941, "recall": 0.986232790988736, "support": 4794}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 18}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.837962962962963, "precision": 0.8578199052132701, "recall": 0.8190045248868778, "support": 221}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9064327485380116, "precision": 0.9365558912386707, "recall": 0.8781869688385269, "support": 353}, "\u2423": {"f1-score": 0.9502361060661098, "precision": 0.9739389426656738, "recall": 0.9276595744680851, "support": 1410}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 271}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 989}, "macro avg": {"f1-score": 0.3558445697363063, "precision": 0.4662101435283761, "recall": 0.29560408677046546, "support": 12109}, "micro avg": {"f1-score": 0.6902495769881557, "precision": 0.9594296633838013, "recall": 0.5390205632174415, "support": 12109}, "weighted avg": {"f1-score": 0.6274711553451062, "precision": 0.7793639296435696, "recall": 0.5390205632174415, "support": 12109}, "\u2205": {"f1-score": 0.8635616438356165, "precision": 0.9613664091093941, "recall": 0.7838196286472149, "support": 6032}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 764}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 248}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.5526717557251909, "precision": 0.8578199052132701, "recall": 0.40765765765765766, "support": 444}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8190224570673711, "precision": 0.9365558912386707, "recall": 0.7276995305164319, "support": 426}, "\u2423": {"f1-score": 0.611500701262272, "precision": 0.9739389426656738, "recall": 0.44565587734241907, "support": 2935}},
  "ppcr": 0.561813527128582
}
```
</details>
