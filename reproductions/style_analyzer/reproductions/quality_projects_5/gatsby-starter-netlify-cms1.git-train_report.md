# Train report for javascript / file:///tmp/top-repos-quality-repos-ocw146us/gatsby-starter-netlify-cms1.git HEAD 7c1d09728f585d13ce558891541b3f8630b24999

### Classification report

PPCR: 0.407

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.986| 1.000| 0.626| 0.993| 0.766| 2114| 3378| 0.626 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 28| 1074| 0.026 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 244| 0.012 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 296| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 278| 0.000 |
| `micro avg` | 0.986| 0.986| 0.401| 0.986| 0.570| 2145| 5270| 0.407 |
| `macro avg` | 0.197| 0.200| 0.125| 0.199| 0.153| 2145| 5270| 0.407 |
| `weighted avg` | 0.971| 0.986| 0.401| 0.978| 0.491| 2145| 5270| 0.407 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|1264 |2114 |0 |0 |0 |0 |
|1046 |28 |0 |0 |0 |0 |
|241 |3 |0 |0 |0 |0 |
|296 |0 |0 |0 |0 |0 |
|278 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/components/Layout.js | 8 |
| src/components/Navbar.js | 6 |
| src/components/Footer.js | 5 |
| src/pages/contact/index.js | 4 |
| src/pages/contact/file-upload.js | 3 |
| src/pages/template.js | 1 |
| src/templates/about-page.js | 1 |
| src/templates/index-page.js | 1 |
| src/components/Testimonials.js | 1 |
| src/components/Features.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.19854425921577837, "precision": 0.1971095571095571, "recall": 0.2, "support": 2145}, "micro avg": {"f1-score": 0.9855477855477855, "precision": 0.9855477855477855, "recall": 0.9855477855477855, "support": 2145}, "weighted avg": {"f1-score": 0.9783742750166795, "precision": 0.9713044375981439, "recall": 0.9855477855477855, "support": 2145}, "\u2205": {"f1-score": 0.9927212960788918, "precision": 0.9855477855477855, "recall": 1.0, "support": 2114}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 28}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 278}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 296}, "macro avg": {"f1-score": 0.15310519645120405, "precision": 0.1971095571095571, "recall": 0.1251628182356424, "support": 5270}, "micro avg": {"f1-score": 0.5701955495616992, "precision": 0.9855477855477855, "recall": 0.40113851992409866, "support": 5270}, "weighted avg": {"f1-score": 0.49069198634930483, "precision": 0.6317230397685806, "recall": 0.40113851992409866, "support": 5270}, "\u2205": {"f1-score": 0.7655259822560202, "precision": 0.9855477855477855, "recall": 0.625814091178212, "support": 3378}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 244}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1074}},
  "ppcr": 0.40702087286527516
}
```
</details>
