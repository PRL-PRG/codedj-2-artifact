# Train report for javascript / file:///tmp/top-repos-quality-repos-ndxw65pu/react-auth-server-admin.git HEAD 703a07fe863fb0eadad6137990df5bf34b5779cb

### Classification report

PPCR: 0.272

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.959| 1.000| 0.519| 0.979| 0.673| 1074| 2070| 0.519 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 16| 110| 0.145 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 14| 270| 0.052 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 12| 1043| 0.012 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 115| 0.035 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 304| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 112| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 92| 0.000 |
| `weighted avg` | 0.920| 0.959| 0.261| 0.939| 0.339| 1120| 4116| 0.272 |
| `micro avg` | 0.959| 0.959| 0.261| 0.959| 0.410| 1120| 4116| 0.272 |
| `macro avg` | 0.120| 0.125| 0.065| 0.122| 0.084| 1120| 4116| 0.272 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|996 |1074 |0 |0 |0 |0 |0 |0 |0 |
|1031 |12 |0 |0 |0 |0 |0 |0 |0 |
|304 |0 |0 |0 |0 |0 |0 |0 |0 |
|256 |14 |0 |0 |0 |0 |0 |0 |0 |
|112 |0 |0 |0 |0 |0 |0 |0 |0 |
|111 |4 |0 |0 |0 |0 |0 |0 |0 |
|94 |16 |0 |0 |0 |0 |0 |0 |0 |
|92 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/components/SignUp/SignUpForm.js | 20 |
| src/components/Users/UserList.js | 16 |
| src/serviceWorker.js | 7 |
| src/actions/signup.js | 2 |
| src/index.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.1223792160437557, "precision": 0.11986607142857143, "recall": 0.125, "support": 1120}, "micro avg": {"f1-score": 0.9589285714285715, "precision": 0.9589285714285715, "recall": 0.9589285714285715, "support": 1120}, "weighted avg": {"f1-score": 0.9388234145070974, "precision": 0.9195440051020408, "recall": 0.9589285714285715, "support": 1120}, "\u2205": {"f1-score": 0.9790337283500457, "precision": 0.9589285714285715, "recall": 1.0, "support": 1074}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 112}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 304}, "macro avg": {"f1-score": 0.0841692789968652, "precision": 0.11986607142857143, "recall": 0.06485507246376812, "support": 4116}, "micro avg": {"f1-score": 0.41023682200152783, "precision": 0.9589285714285715, "recall": 0.260932944606414, "support": 4116}, "weighted avg": {"f1-score": 0.3386402478591078, "precision": 0.48225999583506873, "recall": 0.260932944606414, "support": 4116}, "\u2205": {"f1-score": 0.6733542319749216, "precision": 0.9589285714285715, "recall": 0.518840579710145, "support": 2070}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 270}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 92}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 115}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 110}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1043}},
  "ppcr": 0.272108843537415
}
```
</details>
