# Test report for javascript / file:///tmp/top-repos-quality-repos-mi98bflo/stockmarketnews.git HEAD cb31825adf600fdc5810932601f2e2113bb915c7

### Classification report

PPCR: 0.507

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.904| 0.974| 0.694| 0.938| 0.785| 272| 382| 0.712 |
| `␣` | 0.833| 0.897| 0.196| 0.864| 0.317| 39| 179| 0.218 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 21| 52| 0.404 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 37| 0.081 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 11| 0.000 |
| `micro avg` | 0.896| 0.896| 0.454| 0.896| 0.602| 335| 661| 0.507 |
| `weighted avg` | 0.831| 0.896| 0.454| 0.862| 0.540| 335| 661| 0.507 |
| `macro avg` | 0.348| 0.374| 0.178| 0.360| 0.220| 335| 661| 0.507 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|110 |265 |7 |0 |0 |0 |
|140 |4 |35 |0 |0 |0 |
|31 |21 |0 |0 |0 |0 |
|34 |3 |0 |0 |0 |0 |
|11 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 21}, "macro avg": {"f1-score": 0.36045012564186607, "precision": 0.34755403868031853, "recall": 0.3743401206636501, "support": 335}, "micro avg": {"f1-score": 0.8955223880597015, "precision": 0.8955223880597015, "recall": 0.8955223880597015, "support": 335}, "weighted avg": {"f1-score": 0.8622511826315816, "precision": 0.831363659518109, "recall": 0.8955223880597015, "support": 335}, "\u2205": {"f1-score": 0.9380530973451328, "precision": 0.9044368600682594, "recall": 0.9742647058823529, "support": 272}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.8641975308641975, "precision": 0.8333333333333334, "recall": 0.8974358974358975, "support": 39}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 52}, "macro avg": {"f1-score": 0.2203854533266298, "precision": 0.34755403868031853, "recall": 0.17784960074877884, "support": 661}, "micro avg": {"f1-score": 0.6024096385542169, "precision": 0.8955223880597015, "recall": 0.45385779122541603, "support": 661}, "weighted avg": {"f1-score": 0.539542470983247, "precision": 0.7483533240737394, "recall": 0.45385779122541603, "support": 661}, "\u2205": {"f1-score": 0.7851851851851852, "precision": 0.9044368600682594, "recall": 0.693717277486911, "support": 382}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 37}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u2423": {"f1-score": 0.31674208144796373, "precision": 0.8333333333333334, "recall": 0.19553072625698323, "support": 179}},
  "ppcr": 0.5068078668683812
}
```
</details>
