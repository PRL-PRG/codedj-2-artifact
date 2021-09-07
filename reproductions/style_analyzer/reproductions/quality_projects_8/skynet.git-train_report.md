# Train report for javascript / file:///tmp/top-repos-quality-repos-kquqhe21/skynet.git HEAD 4c1123d693048023ee6c8349e064d74f687c2afe

### Classification report

PPCR: 0.650

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.953| 0.991| 0.813| 0.971| 0.878| 2853| 3475| 0.821 |
| `␣` | 0.961| 0.959| 0.433| 0.960| 0.597| 738| 1636| 0.451 |
| `⏎␣⁺␣⁺` | 0.952| 0.811| 0.687| 0.876| 0.798| 270| 319| 0.846 |
| `'` | 1.000| 1.000| 0.502| 1.000| 0.668| 245| 488| 0.502 |
| `⏎␣⁻␣⁻` | 0.885| 0.757| 0.684| 0.816| 0.771| 243| 269| 0.903 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 35| 454| 0.077 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 106| 0.009 |
| `weighted avg` | 0.945| 0.954| 0.620| 0.949| 0.714| 4385| 6747| 0.650 |
| `micro avg` | 0.954| 0.954| 0.620| 0.954| 0.751| 4385| 6747| 0.650 |
| `macro avg` | 0.679| 0.645| 0.446| 0.660| 0.530| 4385| 6747| 0.650 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|622 |2826 |13 |0 |0 |0 |14 |0 |
|898 |9 |708 |0 |0 |11 |10 |0 |
|243 |0 |0 |245 |0 |0 |0 |0 |
|419 |28 |7 |0 |0 |0 |0 |0 |
|49 |43 |8 |0 |0 |219 |0 |0 |
|26 |59 |0 |0 |0 |0 |184 |0 |
|105 |0 |1 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| commands/Command.js | 84 |
| server/server.js | 34 |
| commands/vote.js | 30 |
| events/botAnnounce.js | 23 |
| commands/wafflehouse.js | 9 |
| server/oauth.js | 6 |
| bot.js | 5 |
| logger.js | 5 |
| events/twitter.js | 3 |
| events/botUpdate.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 245}, "macro avg": {"f1-score": 0.6604903402360243, "precision": 0.6786514738362656, "recall": 0.6454569469000149, "support": 4385}, "micro avg": {"f1-score": 0.9537058152793615, "precision": 0.9537058152793615, "recall": 0.9537058152793615, "support": 4385}, "weighted avg": {"f1-score": 0.948660702186699, "precision": 0.945327191902283, "recall": 0.9537058152793615, "support": 4385}, "\u2205": {"f1-score": 0.9714678583705739, "precision": 0.9531197301854974, "recall": 0.9905362776025236, "support": 2853}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 35}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8760000000000001, "precision": 0.9521739130434783, "recall": 0.8111111111111111, "support": 270}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8159645232815963, "precision": 0.8846153846153846, "recall": 0.757201646090535, "support": 243}, "\u2423": {"f1-score": 0.96, "precision": 0.9606512890094979, "recall": 0.959349593495935, "support": 738}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6684856753069577, "precision": 1.0, "recall": 0.5020491803278688, "support": 488}, "macro avg": {"f1-score": 0.5303058750862123, "precision": 0.6786514738362656, "recall": 0.44551209609280784, "support": 6747}, "micro avg": {"f1-score": 0.7513474667624864, "precision": 0.9537058152793615, "recall": 0.6198310360160071, "support": 6747}, "weighted avg": {"f1-score": 0.7135430235541134, "precision": 0.8764519916906107, "recall": 0.6198310360160071, "support": 6747}, "\u2205": {"f1-score": 0.877639751552795, "precision": 0.9531197301854974, "recall": 0.8132374100719425, "support": 3475}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 454}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 106}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7978142076502732, "precision": 0.9521739130434783, "recall": 0.6865203761755486, "support": 319}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7714884696016772, "precision": 0.8846153846153846, "recall": 0.6840148698884758, "support": 269}, "\u2423": {"f1-score": 0.5967130214917825, "precision": 0.9606512890094979, "recall": 0.43276283618581907, "support": 1636}},
  "ppcr": 0.6499184822884245
}
```
</details>
