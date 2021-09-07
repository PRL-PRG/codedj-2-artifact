# Train report for javascript / file:///tmp/top-repos-quality-repos-59u_dqk9/remix-live-beta.git HEAD b74e67604b13c5f5eb29bfa1ccc02582addc20a6

### Classification report

PPCR: 0.885

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.981| 0.988| 0.972| 0.985| 0.976| 42678| 43418| 0.983 |
| `␣` | 0.968| 0.981| 0.909| 0.974| 0.938| 19979| 21558| 0.927 |
| `'` | 1.000| 1.000| 0.979| 1.000| 0.989| 6845| 6995| 0.979 |
| `⏎␣⁻␣⁻` | 0.957| 0.868| 0.709| 0.910| 0.814| 2036| 2492| 0.817 |
| `⏎` | 0.853| 0.705| 0.166| 0.772| 0.278| 993| 4216| 0.236 |
| `⏎⏎` | 0.979| 0.821| 0.216| 0.893| 0.354| 346| 1313| 0.264 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 147| 2551| 0.058 |
| `macro avg` | 0.820| 0.766| 0.564| 0.791| 0.621| 73024| 82543| 0.885 |
| `micro avg` | 0.977| 0.977| 0.865| 0.977| 0.918| 73024| 82543| 0.885 |
| `weighted avg` | 0.975| 0.977| 0.865| 0.976| 0.887| 73024| 82543| 0.885 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|740 |42185 |407 |0 |66 |0 |20 |0 |
|1579 |323 |19595 |0 |0 |0 |60 |1 |
|150 |0 |0 |6845 |0 |0 |0 |0 |
|3223 |132 |156 |0 |700 |0 |0 |5 |
|2404 |79 |64 |0 |4 |0 |0 |0 |
|456 |254 |11 |0 |4 |0 |1767 |0 |
|967 |10 |5 |0 |47 |0 |0 |284 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/app/files/file-explorer.js | 94 |
| src/app/panels/terminal.js | 77 |
| test-browser/tests/transactionExecution.js | 50 |
| src/app.js | 47 |
| src/app/tabs/runTab/settings.js | 45 |
| src/app/editor/editor.js | 45 |
| src/app/tabs/runTab/model/dropdownlogic.js | 43 |
| src/app/ui/modal-dialog-custom.js | 40 |
| src/app/tabs/debugger/debuggerUI/ButtonNavigator.js | 37 |
| src/app/tabs/runTab/model/recorder.js | 36 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 6845}, "macro avg": {"f1-score": 0.7906243405175347, "precision": 0.8197540325032646, "recall": 0.7661214546557737, "support": 73024}, "micro avg": {"f1-score": 0.9774320771253286, "precision": 0.9774320771253286, "recall": 0.9774320771253286, "support": 73024}, "weighted avg": {"f1-score": 0.976076230963519, "precision": 0.975134160750476, "recall": 0.9774320771253286, "support": 73024}, "\u2205": {"f1-score": 0.9849289641727274, "precision": 0.9814345206244329, "recall": 0.9884483808988237, "support": 42678}, "\u23ce": {"f1-score": 0.7717750826901876, "precision": 0.8526187576126675, "recall": 0.7049345417925479, "support": 993}, "\u23ce\u23ce": {"f1-score": 0.8930817610062893, "precision": 0.9793103448275862, "recall": 0.8208092485549133, "support": 346}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 147}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9101210404326552, "precision": 0.9566865186789388, "recall": 0.8678781925343811, "support": 2036}, "\u2423": {"f1-score": 0.9744635353208843, "precision": 0.9682280857792271, "recall": 0.9807798188097503, "support": 19979}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9891618497109826, "precision": 1.0, "recall": 0.9785561115082202, "support": 6995}, "macro avg": {"f1-score": 0.6214367050029306, "precision": 0.8197540325032646, "recall": 0.5643575423548947, "support": 82543}, "micro avg": {"f1-score": 0.9176239176689144, "precision": 0.9774320771253286, "recall": 0.8647129374992428, "support": 82543}, "weighted avg": {"f1-score": 0.8867758875099718, "precision": 0.9418669427946857, "recall": 0.8647129374992428, "support": 82543}, "\u2205": {"f1-score": 0.9764933276235228, "precision": 0.9814345206244329, "recall": 0.9716016398728637, "support": 43418}, "\u23ce": {"f1-score": 0.2779432201707365, "precision": 0.8526187576126675, "recall": 0.16603415559772297, "support": 4216}, "\u23ce\u23ce": {"f1-score": 0.3543356207111666, "precision": 0.9793103448275862, "recall": 0.2162985529322163, "support": 1313}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2551}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8144733809633556, "precision": 0.9566865186789388, "recall": 0.7090690208667737, "support": 2492}, "\u2423": {"f1-score": 0.9376495358407503, "precision": 0.9682280857792271, "recall": 0.9089433157064662, "support": 21558}},
  "ppcr": 0.8846782888918503
}
```
</details>
