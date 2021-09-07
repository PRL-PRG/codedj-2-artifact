# Train report for javascript / file:///tmp/top-repos-quality-repos-j3fdupy1/kotoz.git HEAD 3dd5fd7040864c3f92e014f59ddb1d94245a341c

### Classification report

PPCR: 0.846

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.947| 0.998| 0.956| 0.972| 0.952| 10599| 11061| 0.958 |
| `␣` | 0.988| 0.894| 0.623| 0.939| 0.764| 2914| 4180| 0.697 |
| `'` | 1.000| 1.000| 0.974| 1.000| 0.987| 949| 974| 0.974 |
| `"` | 1.000| 1.000| 1.000| 1.000| 1.000| 466| 466| 1.000 |
| `⏎` | 0.992| 0.813| 0.303| 0.894| 0.464| 305| 818| 0.373 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 132| 252| 0.524 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 93| 284| 0.327 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 235| 0.030 |
| `macro avg` | 0.616| 0.588| 0.482| 0.601| 0.521| 15465| 18270| 0.846 |
| `micro avg` | 0.960| 0.960| 0.812| 0.960| 0.880| 15465| 18270| 0.846 |
| `weighted avg` | 0.946| 0.960| 0.812| 0.952| 0.850| 15465| 18270| 0.846 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|462 |10576 |23 |0 |0 |0 |0 |0 |0 |
|1266 |310 |2604 |0 |0 |0 |0 |0 |0 |
|25 |0 |0 |949 |0 |0 |0 |0 |0 |
|513 |57 |0 |0 |248 |0 |0 |0 |0 |
|0 |0 |0 |0 |0 |466 |0 |0 |0 |
|191 |85 |8 |0 |0 |0 |0 |0 |0 |
|120 |132 |0 |0 |0 |0 |0 |0 |0 |
|228 |5 |0 |0 |2 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/components/nav.js | 88 |
| src/components/head.js | 46 |
| src/components/layout.js | 42 |
| src/components/hero.js | 30 |
| gatsby-config.js | 27 |
| src/components/loader.js | 26 |
| src/components/projects.js | 26 |
| src/pages/pensieve/tags.js | 24 |
| gatsby-node.js | 24 |
| src/config/index.js | 20 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 466}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 949}, "macro avg": {"f1-score": 0.6005151686017223, "precision": 0.6159351439635415, "recall": 0.5880702199169634, "support": 15465}, "micro avg": {"f1-score": 0.9597801487229227, "precision": 0.9597801487229227, "recall": 0.9597801487229227, "support": 15465}, "weighted avg": {"f1-score": 0.9520505569367447, "precision": 0.9464685736606852, "recall": 0.9597801487229227, "support": 15465}, "\u2205": {"f1-score": 0.9718801690865649, "precision": 0.9472458575906851, "recall": 0.9978299839607511, "support": 10599}, "\u23ce": {"f1-score": 0.8936936936936937, "precision": 0.992, "recall": 0.8131147540983606, "support": 305}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 93}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 132}, "\u2423": {"f1-score": 0.9385474860335196, "precision": 0.9882352941176471, "recall": 0.8936170212765957, "support": 2914}},
  "cl_report_full": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 466}, "\u0027": {"f1-score": 0.9869994799791991, "precision": 1.0, "recall": 0.9743326488706365, "support": 974}, "macro avg": {"f1-score": 0.5209117244287299, "precision": 0.6159351439635415, "recall": 0.4820787358484452, "support": 18270}, "micro avg": {"f1-score": 0.8799762857566327, "precision": 0.9597801487229227, "recall": 0.8124247400109469, "support": 18270}, "weighted avg": {"f1-score": 0.8499226737988311, "precision": 0.9228114920756615, "recall": 0.8124247400109469, "support": 18270}, "\u2205": {"f1-score": 0.9516782147035003, "precision": 0.9472458575906851, "recall": 0.9561522466323117, "support": 11061}, "\u23ce": {"f1-score": 0.4644194756554307, "precision": 0.992, "recall": 0.30317848410757947, "support": 818}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 235}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 284}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 252}, "\u2423": {"f1-score": 0.7641966250917094, "precision": 0.9882352941176471, "recall": 0.6229665071770335, "support": 4180}},
  "ppcr": 0.8464696223316913
}
```
</details>
