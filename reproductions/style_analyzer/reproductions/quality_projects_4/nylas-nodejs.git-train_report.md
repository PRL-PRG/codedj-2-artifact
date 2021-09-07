# Train report for javascript / file:///tmp/top-repos-quality-repos-gulfkp0p/nylas-nodejs.git HEAD 548016d06b8221abc6173bbbf83c150d54f271be

### Classification report

PPCR: 0.909

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.971| 0.995| 0.977| 0.983| 0.974| 17031| 17356| 0.981 |
| `␣` | 0.970| 0.951| 0.787| 0.961| 0.869| 4473| 5409| 0.827 |
| `'` | 1.000| 1.000| 0.993| 1.000| 0.997| 3117| 3138| 0.993 |
| `⏎` | 0.936| 0.900| 0.439| 0.918| 0.597| 972| 1994| 0.487 |
| `⏎␣⁻␣⁻` | 0.987| 0.865| 0.812| 0.922| 0.891| 886| 943| 0.940 |
| `⏎␣⁺␣⁺` | 0.974| 0.756| 0.583| 0.852| 0.730| 808| 1048| 0.771 |
| `⏎⏎` | 0.974| 0.910| 0.579| 0.941| 0.727| 289| 454| 0.637 |
| `micro avg` | 0.973| 0.973| 0.885| 0.973| 0.927| 27576| 30342| 0.909 |
| `weighted avg` | 0.973| 0.973| 0.885| 0.973| 0.918| 27576| 30342| 0.909 |
| `macro avg` | 0.973| 0.911| 0.739| 0.939| 0.826| 27576| 30342| 0.909 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|325 |16954 |71 |0 |6 |0 |0 |0 |
|936 |154 |4255 |0 |53 |10 |1 |0 |
|21 |0 |0 |3117 |0 |0 |0 |0 |
|1022 |77 |0 |0 |875 |6 |9 |5 |
|240 |158 |39 |0 |0 |611 |0 |0 |
|57 |97 |20 |0 |1 |0 |766 |2 |
|165 |26 |0 |0 |0 |0 |0 |263 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| __tests__/restful-model-collection-spec.js | 180 |
| __tests__/management-account-spec.js | 93 |
| __tests__/nylas-api-spec.js | 48 |
| __tests__/delta-spec.js | 34 |
| __tests__/contact-spec.js | 31 |
| __tests__/connect-spec.js | 29 |
| __tests__/thread-spec.js | 29 |
| example/sample-app/bin/www | 28 |
| example/authentication/auth.js | 26 |
| __tests__/webhook-spec.js | 25 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 3117}, "macro avg": {"f1-score": 0.939375172619889, "precision": 0.9732196276629584, "recall": 0.911104324099501, "support": 27576}, "micro avg": {"f1-score": 0.9733463881636205, "precision": 0.9733463881636205, "recall": 0.9733463881636205, "support": 27576}, "weighted avg": {"f1-score": 0.972699663994784, "precision": 0.9733913355124634, "recall": 0.9733463881636205, "support": 27576}, "\u2205": {"f1-score": 0.9829260515407138, "precision": 0.9706859040421391, "recall": 0.9954788327168105, "support": 17031}, "\u23ce": {"f1-score": 0.9176717357105402, "precision": 0.9358288770053476, "recall": 0.9002057613168725, "support": 972}, "\u23ce\u23ce": {"f1-score": 0.9409660107334525, "precision": 0.9740740740740741, "recall": 0.9100346020761245, "support": 289}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8515679442508711, "precision": 0.9744816586921851, "recall": 0.7561881188118812, "support": 808}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9217809867629362, "precision": 0.9871134020618557, "recall": 0.8645598194130926, "support": 886}, "\u2423": {"f1-score": 0.9607134793407088, "precision": 0.9703534777651083, "recall": 0.9512631343617259, "support": 4473}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9966426858513189, "precision": 1.0, "recall": 0.9933078393881453, "support": 3138}, "macro avg": {"f1-score": 0.8262935967152698, "precision": 0.9732196276629584, "recall": 0.7386036763129246, "support": 30342}, "micro avg": {"f1-score": 0.9268621154045373, "precision": 0.9733463881636205, "recall": 0.8846153846153846, "support": 30342}, "weighted avg": {"f1-score": 0.9180005970913361, "precision": 0.9720599709517787, "recall": 0.8846153846153846, "support": 30342}, "\u2205": {"f1-score": 0.9737522256045028, "precision": 0.9706859040421391, "recall": 0.9768379811016363, "support": 17356}, "\u23ce": {"f1-score": 0.597473540457494, "precision": 0.9358288770053476, "recall": 0.43881644934804415, "support": 1994}, "\u23ce\u23ce": {"f1-score": 0.7265193370165745, "precision": 0.9740740740740741, "recall": 0.579295154185022, "support": 454}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7295522388059702, "precision": 0.9744816586921851, "recall": 0.5830152671755725, "support": 1048}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.891215823152996, "precision": 0.9871134020618557, "recall": 0.8123011664899258, "support": 943}, "\u2423": {"f1-score": 0.8688993261180314, "precision": 0.9703534777651083, "recall": 0.7866518765021261, "support": 5409}},
  "ppcr": 0.9088392327466878
}
```
</details>
