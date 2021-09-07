# Train report for javascript / file:///tmp/top-repos-quality-repos-rmeh09qw/burd-archive.git HEAD 1d75f33cd35b28b24b57053ad9fec57508ca3b6d

### Classification report

PPCR: 0.734

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.950| 0.992| 0.954| 0.971| 0.952| 16478| 17144| 0.961 |
| `␣` | 0.966| 0.887| 0.765| 0.925| 0.854| 6013| 6971| 0.863 |
| `⏎⇥⁻` | 0.942| 0.978| 0.630| 0.960| 0.755| 497| 772| 0.644 |
| `⏎` | 1.000| 0.729| 0.202| 0.843| 0.336| 454| 1637| 0.277 |
| `"` | 1.000| 1.000| 0.071| 1.000| 0.132| 299| 4219| 0.071 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 69| 911| 0.076 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 47| 219| 0.215 |
| `⏎⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 23| 108| 0.213 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 549| 0.000 |
| `micro avg` | 0.955| 0.955| 0.701| 0.955| 0.808| 23880| 32530| 0.734 |
| `weighted avg` | 0.950| 0.955| 0.701| 0.951| 0.737| 23880| 32530| 0.734 |
| `macro avg` | 0.540| 0.510| 0.291| 0.522| 0.337| 23880| 32530| 0.734 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎⇥⁺| ⏎⇥⁻| '| ⏎⏎| ⏎⏎⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|666 |16350 |128 |0 |0 |0 |0 |0 |0 |0 |
|958 |676 |5335 |0 |0 |0 |2 |0 |0 |0 |
|3920 |0 |0 |299 |0 |0 |0 |0 |0 |0 |
|1183 |111 |7 |0 |331 |0 |5 |0 |0 |0 |
|842 |28 |41 |0 |0 |0 |0 |0 |0 |0 |
|275 |11 |0 |0 |0 |0 |486 |0 |0 |0 |
|549 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|172 |34 |13 |0 |0 |0 |0 |0 |0 |0 |
|85 |0 |0 |0 |0 |0 |23 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| js/main.js | 322 |
| js/parsedata.js | 184 |
| js/parseinput.js | 105 |
| js/menu.js | 50 |
| index.js | 42 |
| global/js/menu.js | 40 |
| js/updatechecker.js | 33 |
| js/channel.js | 27 |
| js/onclick.js | 26 |
| js/socket.js | 26 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 299}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.5220400880319147, "precision": 0.5397416867284304, "recall": 0.5096020515659513, "support": 23880}, "micro avg": {"f1-score": 0.9548157453936349, "precision": 0.9548157453936349, "recall": 0.9548157453936349, "support": 23880}, "weighted avg": {"f1-score": 0.9511979969970393, "precision": 0.9498723858229007, "recall": 0.9548157453936349, "support": 23880}, "\u2205": {"f1-score": 0.9706720493944431, "precision": 0.9500290528762347, "recall": 0.9922320669984221, "support": 16478}, "\u23ce": {"f1-score": 0.8433121019108281, "precision": 1.0, "recall": 0.7290748898678414, "support": 454}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 69}, "\u23ce\u21e5\u207b": {"f1-score": 0.9595261599210266, "precision": 0.9418604651162791, "recall": 0.9778672032193159, "support": 497}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 47}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 23}, "\u2423": {"f1-score": 0.9248504810609344, "precision": 0.9657856625633598, "recall": 0.8872443040079827, "support": 6013}},
  "cl_report_full": {"\"": {"f1-score": 0.1323594510845507, "precision": 1.0, "recall": 0.07086987437781465, "support": 4219}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 549}, "macro avg": {"f1-score": 0.33657730558896637, "precision": 0.5397416867284304, "recall": 0.29128917335740906, "support": 32530}, "micro avg": {"f1-score": 0.8084027654671158, "precision": 0.9548157453936349, "recall": 0.7009222256378728, "support": 32530}, "weighted avg": {"f1-score": 0.7366459839856739, "precision": 0.9100186355766713, "recall": 0.7009222256378728, "support": 32530}, "\u2205": {"f1-score": 0.9518542236711881, "precision": 0.9500290528762347, "recall": 0.953686420905273, "support": 17144}, "\u23ce": {"f1-score": 0.33638211382113825, "precision": 1.0, "recall": 0.20219914477703116, "support": 1637}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 911}, "\u23ce\u21e5\u207b": {"f1-score": 0.7546583850931677, "precision": 0.9418604651162791, "recall": 0.6295336787564767, "support": 772}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 219}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 108}, "\u2423": {"f1-score": 0.8539415766306523, "precision": 0.9657856625633598, "recall": 0.7653134414000861, "support": 6971}},
  "ppcr": 0.7340916077466954
}
```
</details>
