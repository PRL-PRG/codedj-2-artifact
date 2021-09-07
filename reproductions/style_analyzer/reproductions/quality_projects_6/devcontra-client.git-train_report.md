# Train report for javascript / file:///tmp/top-repos-quality-repos-t4upby31/devcontra-client.git HEAD 7abd70e6679357276ccce4d563f45fedc1ecbfd2

### Classification report

PPCR: 0.719

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.902| 1.000| 0.974| 0.949| 0.937| 6738| 6913| 0.975 |
| `␣` | 0.999| 0.757| 0.493| 0.861| 0.660| 2179| 3347| 0.651 |
| `⏎` | 0.958| 0.964| 0.755| 0.961| 0.844| 1541| 1969| 0.783 |
| `⏎␣⁻␣⁻` | 1.000| 0.674| 0.667| 0.805| 0.800| 481| 486| 0.990 |
| `⏎␣⁺␣⁺` | 0.910| 0.814| 0.681| 0.859| 0.779| 397| 474| 0.838 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 117| 0.060 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 2158| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 304| 0.000 |
| `micro avg` | 0.927| 0.927| 0.667| 0.927| 0.776| 11343| 15768| 0.719 |
| `weighted avg` | 0.932| 0.927| 0.667| 0.924| 0.704| 11343| 15768| 0.719 |
| `macro avg` | 0.596| 0.526| 0.446| 0.554| 0.503| 11343| 15768| 0.719 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| "| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|175 |6735 |0 |0 |0 |0 |3 |0 |0 |
|1168 |451 |1649 |0 |56 |0 |23 |0 |0 |
|2158 |0 |0 |0 |0 |0 |0 |0 |0 |
|428 |50 |0 |0 |1486 |0 |5 |0 |0 |
|5 |149 |0 |0 |8 |324 |0 |0 |0 |
|77 |72 |1 |0 |1 |0 |323 |0 |0 |
|304 |0 |0 |0 |0 |0 |0 |0 |0 |
|110 |6 |0 |0 |0 |0 |1 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| components/ConfigureSite.js | 157 |
| pages/signup.js | 156 |
| pages/configure.js | 123 |
| pages/signin.js | 113 |
| components/SnackBarContentWrapper.js | 44 |
| pages/demo.js | 39 |
| pages/_document.js | 32 |
| components/AppBar.js | 26 |
| pages/_app.js | 25 |
| pages/index.js | 23 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.5543809888884347, "precision": 0.5962245931464596, "recall": 0.5259789379038906, "support": 11343}, "micro avg": {"f1-score": 0.9271797584413295, "precision": 0.9271797584413295, "recall": 0.9271797584413295, "support": 11343}, "weighted avg": {"f1-score": 0.9236889110019453, "precision": 0.932472431682485, "recall": 0.9271797584413295, "support": 11343}, "\u2205": {"f1-score": 0.9485247517780439, "precision": 0.9024520970119255, "recall": 0.9995547640249333, "support": 6738}, "\u23ce": {"f1-score": 0.9611901681759379, "precision": 0.9580915538362347, "recall": 0.9643088903309539, "support": 1541}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8590425531914894, "precision": 0.9098591549295775, "recall": 0.8136020151133502, "support": 397}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8049689440993789, "precision": 1.0, "recall": 0.6735966735966736, "support": 481}, "\u2423": {"f1-score": 0.8613214938626274, "precision": 0.9993939393939394, "recall": 0.7567691601652135, "support": 2179}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 304}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2158}, "macro avg": {"f1-score": 0.5025680734444697, "precision": 0.5962245931464596, "recall": 0.4462163130387931, "support": 15768}, "micro avg": {"f1-score": 0.7758474419977132, "precision": 0.9271797584413295, "recall": 0.6669837645865043, "support": 15768}, "weighted avg": {"f1-score": 0.7043991357715614, "precision": 0.7856023827203907, "recall": 0.6669837645865043, "support": 15768}, "\u2205": {"f1-score": 0.9369782971619366, "precision": 0.9024520970119255, "recall": 0.9742514103862289, "support": 6913}, "\u23ce": {"f1-score": 0.8443181818181819, "precision": 0.9580915538362347, "recall": 0.7546978161503302, "support": 1969}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 117}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7792521109770808, "precision": 0.9098591549295775, "recall": 0.6814345991561181, "support": 474}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8, "precision": 1.0, "recall": 0.6666666666666666, "support": 486}, "\u2423": {"f1-score": 0.6599959975985592, "precision": 0.9993939393939394, "recall": 0.4926800119510009, "support": 3347}},
  "ppcr": 0.7193683409436834
}
```
</details>
