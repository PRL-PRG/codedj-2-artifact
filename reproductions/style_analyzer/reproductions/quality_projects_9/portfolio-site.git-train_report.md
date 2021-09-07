# Train report for javascript / file:///tmp/top-repos-quality-repos-z_b870uy/portfolio-site.git HEAD 677a844282dfc0b83610c38d303b2df042334354

### Classification report

PPCR: 0.359

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.948| 1.000| 0.637| 0.973| 0.762| 2021| 3171| 0.637 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 81| 1489| 0.054 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 24| 136| 0.176 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 277| 0.011 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 143| 0.021 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 438| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 172| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 116| 0.000 |
| `weighted avg` | 0.899| 0.948| 0.340| 0.923| 0.407| 2132| 5942| 0.359 |
| `macro avg` | 0.118| 0.125| 0.080| 0.122| 0.095| 2132| 5942| 0.359 |
| `micro avg` | 0.948| 0.948| 0.340| 0.948| 0.501| 2132| 5942| 0.359 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1150 |2021 |0 |0 |0 |0 |0 |0 |0 |
|1408 |81 |0 |0 |0 |0 |0 |0 |0 |
|438 |0 |0 |0 |0 |0 |0 |0 |0 |
|274 |3 |0 |0 |0 |0 |0 |0 |0 |
|172 |0 |0 |0 |0 |0 |0 |0 |0 |
|140 |3 |0 |0 |0 |0 |0 |0 |0 |
|112 |24 |0 |0 |0 |0 |0 |0 |0 |
|116 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/components/Helmet.js | 28 |
| src/sections/Landing.js | 18 |
| src/components/Section.js | 14 |
| plugins/gatsby-source-medium-fix/gatsby-node.js | 10 |
| src/components/Header.js | 7 |
| src/pages/index.js | 6 |
| src/components/SocialLink.js | 5 |
| src/components/Footer.js | 4 |
| src/components/Layout.js | 4 |
| src/pages/404.js | 4 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.12165904165663376, "precision": 0.11849202626641651, "recall": 0.125, "support": 2132}, "micro avg": {"f1-score": 0.9479362101313321, "precision": 0.9479362101313321, "recall": 0.9479362101313321, "support": 2132}, "weighted avg": {"f1-score": 0.9226000870095941, "precision": 0.898583058478153, "recall": 0.9479362101313321, "support": 2132}, "\u2205": {"f1-score": 0.97327233325307, "precision": 0.9479362101313321, "recall": 1.0, "support": 2021}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 24}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 81}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 172}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 438}, "macro avg": {"f1-score": 0.09527625872147841, "precision": 0.11849202626641651, "recall": 0.07966729738252917, "support": 5942}, "micro avg": {"f1-score": 0.5006192717364379, "precision": 0.9479362101313321, "recall": 0.34012117132278696, "support": 5942}, "weighted avg": {"f1-score": 0.4067600355514076, "precision": 0.5058744063154584, "recall": 0.34012117132278696, "support": 5942}, "\u2205": {"f1-score": 0.7622100697718273, "precision": 0.9479362101313321, "recall": 0.6373383790602334, "support": 3171}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 277}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 116}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 143}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 136}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1489}},
  "ppcr": 0.35880175025244027
}
```
</details>
