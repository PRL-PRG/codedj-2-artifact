# Train report for javascript / file:///tmp/top-repos-quality-repos-rh_s98mm/altas.git HEAD 7ae9f5010044492d937f5468e2316c4bc172201a

### Classification report

PPCR: 0.727

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.978| 0.995| 0.868| 0.986| 0.920| 19741| 22625| 0.873 |
| `␣` | 0.965| 0.955| 0.516| 0.960| 0.673| 5802| 10735| 0.540 |
| `'` | 0.967| 1.000| 1.000| 0.983| 0.983| 4182| 4182| 1.000 |
| `"` | 1.000| 0.867| 0.859| 0.929| 0.924| 1066| 1076| 0.991 |
| `⏎␣⁻␣⁻` | 0.983| 0.948| 0.540| 0.965| 0.697| 691| 1214| 0.569 |
| `⏎` | 0.800| 0.740| 0.157| 0.769| 0.263| 485| 2285| 0.212 |
| `⏎⏎` | 0.959| 0.887| 0.168| 0.922| 0.286| 212| 1120| 0.189 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 199| 1323| 0.150 |
| `weighted avg` | 0.966| 0.972| 0.707| 0.969| 0.783| 32378| 44560| 0.727 |
| `micro avg` | 0.972| 0.972| 0.707| 0.972| 0.818| 32378| 44560| 0.727 |
| `macro avg` | 0.832| 0.799| 0.513| 0.814| 0.593| 32378| 44560| 0.727 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| "| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2884 |19635 |106 |0 |0 |0 |0 |0 |0 |
|4933 |172 |5541 |0 |81 |0 |0 |8 |0 |
|0 |0 |0 |4182 |0 |0 |0 |0 |0 |
|1800 |61 |54 |0 |359 |0 |0 |3 |8 |
|1124 |161 |38 |0 |0 |0 |0 |0 |0 |
|10 |0 |0 |142 |0 |0 |924 |0 |0 |
|523 |34 |0 |0 |2 |0 |0 |655 |0 |
|908 |15 |2 |0 |7 |0 |0 |0 |188 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| app/components/Header/index.js | 108 |
| internals/scripts/generate-templates-for-linting.js | 57 |
| app/utils/tests/sagaInjectors.test.js | 27 |
| internals/templates/utils/tests/sagaInjectors.test.js | 27 |
| internals/scripts/setup.js | 22 |
| app/components/MyForm3/Form.js | 19 |
| app/components/MyForm1/Form.js | 18 |
| internals/scripts/extract-intl.js | 17 |
| app/components/MyFormX1/Form.js | 15 |
| app/components/MyFormX2/Form.js | 15 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.928643216080402, "precision": 1.0, "recall": 0.8667917448405253, "support": 1066}, "\u0027": {"f1-score": 0.9833059017164354, "precision": 0.9671600370027752, "recall": 1.0, "support": 4182}, "macro avg": {"f1-score": 0.8142367655906171, "precision": 0.831560084084641, "recall": 0.7989172439426817, "support": 32378}, "micro avg": {"f1-score": 0.9723886589659645, "precision": 0.9723886589659645, "recall": 0.9723886589659645, "support": 32378}, "weighted avg": {"f1-score": 0.9690685309299079, "precision": 0.9662947603620843, "recall": 0.9723886589659645, "support": 32378}, "\u2205": {"f1-score": 0.986212612069615, "precision": 0.9779360494073115, "recall": 0.9946304645154754, "support": 19741}, "\u23ce": {"f1-score": 0.7687366167023554, "precision": 0.799554565701559, "recall": 0.7402061855670103, "support": 485}, "\u23ce\u23ce": {"f1-score": 0.9215686274509803, "precision": 0.9591836734693877, "recall": 0.8867924528301887, "support": 212}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 199}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9653647752394989, "precision": 0.9834834834834835, "recall": 0.9479015918958031, "support": 691}, "\u2423": {"f1-score": 0.9600623754656502, "precision": 0.965162863612611, "recall": 0.9550155118924509, "support": 5802}},
  "cl_report_full": {"\"": {"f1-score": 0.924, "precision": 1.0, "recall": 0.8587360594795539, "support": 1076}, "\u0027": {"f1-score": 0.9833059017164354, "precision": 0.9671600370027752, "recall": 1.0, "support": 4182}, "macro avg": {"f1-score": 0.5930837256928898, "precision": 0.831560084084641, "recall": 0.5134063631503191, "support": 44560}, "micro avg": {"f1-score": 0.8184252255062517, "precision": 0.9723886589659645, "recall": 0.7065529622980251, "support": 44560}, "weighted avg": {"f1-score": 0.7831925057060425, "precision": 0.9358776162327228, "recall": 0.7065529622980251, "support": 44560}, "\u2205": {"f1-score": 0.9196075217197854, "precision": 0.9779360494073115, "recall": 0.8678453038674033, "support": 22625}, "\u23ce": {"f1-score": 0.26261887344550106, "precision": 0.799554565701559, "recall": 0.15711159737417943, "support": 2285}, "\u23ce\u23ce": {"f1-score": 0.28571428571428564, "precision": 0.9591836734693877, "recall": 0.16785714285714284, "support": 1120}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1323}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.696808510638298, "precision": 0.9834834834834835, "recall": 0.5395387149917628, "support": 1214}, "\u2423": {"f1-score": 0.6726147123088129, "precision": 0.965162863612611, "recall": 0.5161620866325105, "support": 10735}},
  "ppcr": 0.7266157989228007
}
```
</details>
