# Train report for javascript / file:///tmp/top-repos-quality-repos-w9_53pmg/fabric-samples.git HEAD f0f756e0a68040c740a9d6986315a214e7017a1d

### Classification report

PPCR: 0.843

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.963| 0.980| 0.957| 0.972| 0.960| 14282| 14629| 0.976 |
| `␣` | 0.944| 0.927| 0.821| 0.935| 0.878| 6131| 6924| 0.885 |
| `'` | 1.000| 1.000| 0.982| 1.000| 0.991| 2689| 2739| 0.982 |
| `⏎` | 0.973| 0.868| 0.263| 0.917| 0.414| 653| 2157| 0.303 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 23| 692| 0.033 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 21| 290| 0.072 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 14| 271| 0.052 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 11| 280| 0.039 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 282| 0.035 |
| `weighted avg` | 0.959| 0.962| 0.812| 0.961| 0.840| 23834| 28264| 0.843 |
| `micro avg` | 0.962| 0.962| 0.812| 0.962| 0.881| 23834| 28264| 0.843 |
| `macro avg` | 0.431| 0.419| 0.336| 0.425| 0.360| 23834| 28264| 0.843 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎⇥⁺| ⏎⇥⁻| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|347 |14002 |280 |0 |0 |0 |0 |0 |0 |0 |
|793 |449 |5682 |0 |0 |0 |0 |0 |0 |0 |
|50 |0 |0 |2689 |0 |0 |0 |0 |0 |0 |
|1504 |50 |36 |0 |567 |0 |0 |0 |0 |0 |
|669 |1 |7 |0 |15 |0 |0 |0 |0 |0 |
|272 |0 |10 |0 |0 |0 |0 |0 |0 |0 |
|269 |10 |0 |0 |1 |0 |0 |0 |0 |0 |
|269 |21 |0 |0 |0 |0 |0 |0 |0 |0 |
|257 |9 |5 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| balance-transfer/app.js | 84 |
| chaincode/marbles02/node/marbles_chaincode.js | 73 |
| off_chain_data/blockProcessing.js | 65 |
| balance-transfer/app/query.js | 54 |
| balance-transfer/app/instantiate-chaincode.js | 40 |
| chaincode/fabcar/javascript/lib/fabcar.js | 35 |
| off_chain_data/couchdbutil.js | 30 |
| fabcar/javascript-low-level/invoke.js | 29 |
| chaincode/fabcar/javascript-low-level/fabcar.js | 29 |
| off_chain_data/blockEventListener.js | 28 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 2689}, "macro avg": {"f1-score": 0.4249176534671683, "precision": 0.43103063860628654, "recall": 0.4194956303520534, "support": 23834}, "micro avg": {"f1-score": 0.9624905597046236, "precision": 0.9624905597046236, "recall": 0.9624905597046236, "support": 23834}, "weighted avg": {"f1-score": 0.9607164267448011, "precision": 0.9592389640567913, "recall": 0.9624905597046236, "support": 23834}, "\u2205": {"f1-score": 0.9715514848737162, "precision": 0.9628661807179205, "recall": 0.9803949026746954, "support": 14282}, "\u23ce": {"f1-score": 0.9174757281553397, "precision": 0.9725557461406518, "recall": 0.8683001531393568, "support": 653}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 23}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 21}, "\u2423": {"f1-score": 0.9352316681754589, "precision": 0.9438538205980066, "recall": 0.9267656173544283, "support": 6131}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9907885040530582, "precision": 1.0, "recall": 0.981745162468054, "support": 2739}, "macro avg": {"f1-score": 0.36028748393414606, "precision": 0.43103063860628654, "recall": 0.33581934413596326, "support": 28264}, "micro avg": {"f1-score": 0.8806480095205189, "precision": 0.9624905597046236, "recall": 0.8116331729408435, "support": 28264}, "weighted avg": {"f1-score": 0.8395510550310045, "precision": 0.900715254598374, "recall": 0.8116331729408435, "support": 28264}, "\u2205": {"f1-score": 0.9599945151006136, "precision": 0.9628661807179205, "recall": 0.9571399275411853, "support": 14629}, "\u23ce": {"f1-score": 0.4138686131386861, "precision": 0.9725557461406518, "recall": 0.26286509040333794, "support": 2157}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 282}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 280}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 692}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 271}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 290}, "\u2423": {"f1-score": 0.8779357231149566, "precision": 0.9438538205980066, "recall": 0.8206239168110918, "support": 6924}},
  "ppcr": 0.8432635154259835
}
```
</details>
