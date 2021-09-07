# Train report for javascript / file:///tmp/top-repos-quality-repos-rluio1rd/imlachchatt.git HEAD 09411754b4fde46f86fa16057c114ff2ff25cb06

### Classification report

PPCR: 0.753

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.983| 0.996| 0.961| 0.989| 0.971| 18713| 19408| 0.964 |
| `␣` | 0.964| 0.913| 0.274| 0.938| 0.427| 2140| 7131| 0.300 |
| `'` | 1.000| 1.000| 0.956| 1.000| 0.978| 2042| 2136| 0.956 |
| `⏎␣⁻␣⁻` | 0.987| 0.880| 0.859| 0.930| 0.919| 1019| 1044| 0.976 |
| `⏎␣⁺␣⁺` | 0.952| 0.985| 0.795| 0.968| 0.867| 855| 1059| 0.807 |
| `⏎` | 0.973| 0.948| 0.296| 0.960| 0.454| 648| 2076| 0.312 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 33| 140| 0.236 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 11| 807| 0.014 |
| `micro avg` | 0.982| 0.982| 0.739| 0.982| 0.843| 25461| 33801| 0.753 |
| `macro avg` | 0.732| 0.715| 0.518| 0.723| 0.577| 25461| 33801| 0.753 |
| `weighted avg` | 0.980| 0.982| 0.739| 0.980| 0.793| 25461| 33801| 0.753 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|695 |18642 |50 |0 |0 |13 |8 |0 |0 |
|4991 |177 |1953 |0 |5 |1 |4 |0 |0 |
|94 |0 |0 |2042 |0 |0 |0 |0 |0 |
|1428 |27 |5 |0 |614 |2 |0 |0 |0 |
|204 |8 |4 |0 |1 |842 |0 |0 |0 |
|25 |96 |0 |0 |0 |26 |897 |0 |0 |
|796 |0 |0 |0 |11 |0 |0 |0 |0 |
|107 |20 |13 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/web_app/js/call.js | 75 |
| src/web_app/js/peerconnectionclient.js | 60 |
| Gruntfile.js | 45 |
| src/web_app/js/sdputils.js | 32 |
| src/web_app/js/roomselection_test.js | 31 |
| src/web_app/js/background_test.js | 25 |
| src/web_app/js/appcontroller.js | 24 |
| src/web_app/js/utils_test.js | 22 |
| src/web_app/js/roomselection.js | 19 |
| src/web_app/js/peerconnectionclient_test.js | 16 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 2042}, "macro avg": {"f1-score": 0.7232778060108278, "precision": 0.7324374988200448, "recall": 0.715177954207916, "support": 25461}, "micro avg": {"f1-score": 0.9815011193590196, "precision": 0.9815011193590196, "recall": 0.9815011193590196, "support": 25461}, "weighted avg": {"f1-score": 0.9804046194625581, "precision": 0.9797660122794221, "recall": 0.9815011193590196, "support": 25461}, "\u2205": {"f1-score": 0.9894116710452989, "precision": 0.9827095413811281, "recall": 0.9962058462031743, "support": 18713}, "\u23ce": {"f1-score": 0.9601250977326037, "precision": 0.9730586370839936, "recall": 0.9475308641975309, "support": 648}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.968372627947096, "precision": 0.9524886877828054, "recall": 0.9847953216374269, "support": 855}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 33}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9304979253112032, "precision": 0.9867986798679867, "recall": 0.8802747791952895, "support": 1019}, "\u2423": {"f1-score": 0.9378151260504202, "precision": 0.9644444444444444, "recall": 0.9126168224299065, "support": 2140}},
  "cl_report_full": {"\u0027": {"f1-score": 0.977501196744854, "precision": 1.0, "recall": 0.9559925093632958, "support": 2136}, "macro avg": {"f1-score": 0.5768159085441003, "precision": 0.7324374988200448, "recall": 0.5175556336636296, "support": 33801}, "micro avg": {"f1-score": 0.8433734939759036, "precision": 0.9815011193590196, "recall": 0.7393272388390876, "support": 33801}, "weighted avg": {"f1-score": 0.7929770873391563, "precision": 0.9510030231409962, "recall": 0.7393272388390876, "support": 33801}, "\u2205": {"f1-score": 0.9714940851529522, "precision": 0.9827095413811281, "recall": 0.9605317394888706, "support": 19408}, "\u23ce": {"f1-score": 0.4536387144440339, "precision": 0.9730586370839936, "recall": 0.2957610789980732, "support": 2076}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 807}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8667009778692742, "precision": 0.9524886877828054, "recall": 0.7950897072710104, "support": 1059}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 140}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9185867895545314, "precision": 0.9867986798679867, "recall": 0.8591954022988506, "support": 1044}, "\u2423": {"f1-score": 0.426605504587156, "precision": 0.9644444444444444, "recall": 0.2738746318889356, "support": 7131}},
  "ppcr": 0.7532617378184078
}
```
</details>
