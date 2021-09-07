# Train report for javascript / file:///tmp/top-repos-quality-repos-vheepge6/scrapbook_frontend.git HEAD 79cc2b7cdd1bbed3ea47fc15af0bbeea04590f55

### Classification report

PPCR: 0.120

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `␣` | 0.967| 0.987| 0.296| 0.977| 0.454| 1061| 3533| 0.300 |
| `⏎` | 0.976| 0.997| 0.797| 0.987| 0.877| 1038| 1299| 0.799 |
| `∅` | 0.000| 0.000| 0.000| 0.000| 0.000| 34| 10394| 0.003 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 102| 0.039 |
| `␣⇥⇥⇥⇥` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 141| 0.021 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 224| 0.004 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 196| 0.005 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 143| 0.007 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 694| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 384| 0.000 |
| `␣⇥⇥⇥` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 169| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 161| 0.000 |
| `␣⇥⇥` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 160| 0.000 |
| `␣⇥` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 104| 0.000 |
| `␣⇥⇥⇥⇥⇥` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 89| 0.000 |
| `macro avg` | 0.130| 0.132| 0.073| 0.131| 0.089| 2143| 17793| 0.120 |
| `micro avg` | 0.972| 0.972| 0.117| 0.972| 0.209| 2143| 17793| 0.120 |
| `weighted avg` | 0.952| 0.972| 0.117| 0.961| 0.154| 2143| 17793| 0.120 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| '| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ␣⇥⇥⇥| ⏎␣⁻␣⁻| ␣⇥⇥| ⏎␣⁺␣⁺| ␣⇥⇥⇥⇥| ␣⇥| ⏎⏎| ␣⇥⇥⇥⇥⇥| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|10360 |0 |29 |5 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2472 |0 |1047 |14 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|261 |0 |3 |1035 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|694 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|384 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|223 |0 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|195 |0 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|169 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|161 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|160 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|142 |0 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|138 |0 |0 |3 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|104 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|98 |0 |1 |3 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|89 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/React App_files/bundle.js | 28 |
| src/React App_files/main.ecf983c33cb09a2c689f.hot-update.js | 11 |
| src/actions/index.js | 9 |
| src/components/NewsList.js | 2 |
| src/components/Signin.js | 2 |
| src/index.js | 2 |
| src/components/NewsShow.js | 1 |
| src/reducers/news.js | 1 |
| src/reducers/getUser.js | 1 |
| src/components/NewsCard.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.13088887070842165, "precision": 0.1295449398073137, "recall": 0.13226098184175689, "support": 2143}, "micro avg": {"f1-score": 0.971535230984601, "precision": 0.971535230984601, "recall": 0.971535230984601, "support": 2143}, "weighted avg": {"f1-score": 0.9614574597914655, "precision": 0.9515866401603296, "recall": 0.971535230984601, "support": 2143}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 34}, "\u23ce": {"f1-score": 0.986653956148713, "precision": 0.9764150943396226, "recall": 0.9971098265895953, "support": 1038}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.976679104477612, "precision": 0.9667590027700831, "recall": 0.9868049010367578, "support": 1061}, "\u2423\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423\u21e5\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423\u21e5\u21e5\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423\u21e5\u21e5\u21e5\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u2423\u21e5\u21e5\u21e5\u21e5\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 694}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 384}, "macro avg": {"f1-score": 0.08874199845277093, "precision": 0.1295449398073137, "recall": 0.07287436371944105, "support": 17793}, "micro avg": {"f1-score": 0.20886837881219902, "precision": 0.971535230984601, "recall": 0.11701230821109425, "support": 17793}, "weighted avg": {"f1-score": 0.15413749877829033, "precision": 0.26324525174697205, "recall": 0.11701230821109425, "support": 17793}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10394}, "\u23ce": {"f1-score": 0.877490462060195, "precision": 0.9764150943396226, "recall": 0.7967667436489607, "support": 1299}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 102}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 143}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 224}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 161}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 196}, "\u2423": {"f1-score": 0.4536395147313691, "precision": 0.9667590027700831, "recall": 0.29634871214265496, "support": 3533}, "\u2423\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 104}, "\u2423\u21e5\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 160}, "\u2423\u21e5\u21e5\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 169}, "\u2423\u21e5\u21e5\u21e5\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 141}, "\u2423\u21e5\u21e5\u21e5\u21e5\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 89}},
  "ppcr": 0.12044062271679874
}
```
</details>
