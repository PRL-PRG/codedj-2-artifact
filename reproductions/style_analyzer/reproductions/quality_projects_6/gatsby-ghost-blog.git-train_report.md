# Train report for javascript / file:///tmp/top-repos-quality-repos-koe6a6vy/gatsby-ghost-blog.git HEAD f8d82237cb3277d95fa0a2093fa418bea4903ad6

### Classification report

PPCR: 0.631

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.959| 1.000| 0.854| 0.979| 0.903| 6532| 7649| 0.854 |
| `␣` | 0.998| 0.785| 0.161| 0.878| 0.278| 525| 2553| 0.206 |
| `"` | 1.000| 1.000| 0.460| 1.000| 0.630| 275| 598| 0.460 |
| `⏎` | 0.970| 0.970| 0.355| 0.970| 0.520| 233| 636| 0.366 |
| `'` | 1.000| 1.000| 0.493| 1.000| 0.661| 144| 292| 0.493 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 97| 292| 0.332 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 69| 283| 0.244 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 187| 0.005 |
| `micro avg` | 0.964| 0.964| 0.608| 0.964| 0.745| 7876| 12490| 0.631 |
| `weighted avg` | 0.944| 0.964| 0.608| 0.952| 0.682| 7876| 12490| 0.631 |
| `macro avg` | 0.616| 0.594| 0.290| 0.603| 0.374| 7876| 12490| 0.631 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| ⏎␣⁻␣⁻␣⁻␣⁻| '| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1117 |6532 |0 |0 |0 |0 |0 |0 |0 |
|2028 |110 |412 |3 |0 |0 |0 |0 |0 |
|403 |6 |1 |226 |0 |0 |0 |0 |0 |
|323 |0 |0 |0 |275 |0 |0 |0 |0 |
|195 |93 |0 |4 |0 |0 |0 |0 |0 |
|148 |0 |0 |0 |0 |0 |144 |0 |0 |
|214 |69 |0 |0 |0 |0 |0 |0 |0 |
|186 |1 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/components/common/meta/ArticleMeta.js | 49 |
| src/components/common/Layout.js | 24 |
| src/components/common/meta/WebsiteMeta.js | 23 |
| src/components/common/meta/AuthorMeta.js | 21 |
| src/components/common/cvRaw.js | 16 |
| src/templates/author.js | 16 |
| src/utils/rss/generate-feed.js | 15 |
| gatsby-node.js | 15 |
| src/components/common/PostCard.js | 12 |
| plugins/gatsby-plugin-ghost-manifest/src/common.js | 11 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 275}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 144}, "macro avg": {"f1-score": 0.603439007492832, "precision": 0.615821578273743, "recall": 0.5943398732883711, "support": 7876}, "micro avg": {"f1-score": 0.9635601828339259, "precision": 0.9635601828339259, "recall": 0.9635601828339259, "support": 7876}, "weighted avg": {"f1-score": 0.9524645699062679, "precision": 0.9437731756994935, "recall": 0.9635601828339259, "support": 7876}, "\u2205": {"f1-score": 0.9790901596342652, "precision": 0.9590368521509323, "recall": 1.0, "support": 6532}, "\u23ce": {"f1-score": 0.9699570815450643, "precision": 0.9699570815450643, "recall": 0.9699570815450643, "support": 233}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 69}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 97}, "\u2423": {"f1-score": 0.8784648187633262, "precision": 0.9975786924939467, "recall": 0.7847619047619048, "support": 525}},
  "cl_report_full": {"\"": {"f1-score": 0.6300114547537228, "precision": 1.0, "recall": 0.459866220735786, "support": 598}, "\u0027": {"f1-score": 0.6605504587155963, "precision": 1.0, "recall": 0.4931506849315068, "support": 292}, "macro avg": {"f1-score": 0.373996632158554, "precision": 0.615821578273743, "recall": 0.2904636783280743, "support": 12490}, "micro avg": {"f1-score": 0.7452617106942945, "precision": 0.9635601828339259, "recall": 0.6076060848678944, "support": 12490}, "weighted avg": {"f1-score": 0.6821654555206733, "precision": 0.9118802232107436, "recall": 0.6076060848678944, "support": 12490}, "\u2205": {"f1-score": 0.9034578146611341, "precision": 0.9590368521509323, "recall": 0.8539678389331938, "support": 7649}, "\u23ce": {"f1-score": 0.5201380897583429, "precision": 0.9699570815450643, "recall": 0.3553459119496855, "support": 636}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 187}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 283}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 292}, "\u2423": {"f1-score": 0.27781523937963587, "precision": 0.9975786924939467, "recall": 0.16137877007442225, "support": 2553}},
  "ppcr": 0.6305844675740593
}
```
</details>
