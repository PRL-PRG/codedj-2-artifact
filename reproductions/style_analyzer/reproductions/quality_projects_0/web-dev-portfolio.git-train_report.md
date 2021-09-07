# Train report for javascript / file:///tmp/top-repos-quality-repos-d_85bwa5/web-dev-portfolio.git HEAD 072995f16f6bc081ce106652b0966b8431e233c6

### Classification report

PPCR: 0.438

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.979| 1.000| 0.697| 0.989| 0.814| 2327| 3337| 0.697 |
| `"` | 1.000| 1.000| 0.286| 1.000| 0.445| 224| 782| 0.286 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 39| 1277| 0.031 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 11| 298| 0.037 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 238| 0.000 |
| `macro avg` | 0.396| 0.400| 0.197| 0.398| 0.252| 2601| 5932| 0.438 |
| `micro avg` | 0.981| 0.981| 0.430| 0.981| 0.598| 2601| 5932| 0.438 |
| `weighted avg` | 0.962| 0.981| 0.430| 0.971| 0.517| 2601| 5932| 0.438 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| '| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|1010 |2327 |0 |0 |0 |0 |
|1238 |39 |0 |0 |0 |0 |
|558 |0 |0 |224 |0 |0 |
|287 |11 |0 |0 |0 |0 |
|238 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/App.js | 11 |
| src/components/Hero/Hero.js | 8 |
| src/components/About/About.js | 7 |
| src/serviceWorker.js | 7 |
| src/components/Projects/Projects.js | 6 |
| src/components/Contact/Contact.js | 4 |
| src/components/Footer/Footer.js | 2 |
| src/components/About/SkillsCard.js | 1 |
| src/components/Utils/Helpers.js | 1 |
| src/components/Projects/ProjPg.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 224}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.39787414965986395, "precision": 0.395793016407236, "recall": 0.4, "support": 2601}, "micro avg": {"f1-score": 0.9807766243752403, "precision": 0.9807766243752403, "recall": 0.9807766243752403, "support": 2601}, "weighted avg": {"f1-score": 0.9712671016118867, "precision": 0.9619576108797351, "recall": 0.9807766243752403, "support": 2601}, "\u2205": {"f1-score": 0.9893707482993197, "precision": 0.97896508203618, "recall": 1.0, "support": 2327}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 39}},
  "cl_report_full": {"\"": {"f1-score": 0.4453280318091451, "precision": 1.0, "recall": 0.2864450127877238, "support": 782}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 238}, "macro avg": {"f1-score": 0.25196375126907433, "precision": 0.395793016407236, "recall": 0.19675558931211473, "support": 5932}, "micro avg": {"f1-score": 0.5979139810148834, "precision": 0.9807766243752403, "recall": 0.4300404585300067, "support": 5932}, "weighted avg": {"f1-score": 0.5168917850054181, "precision": 0.6825364933841425, "recall": 0.4300404585300067, "support": 5932}, "\u2205": {"f1-score": 0.8144907245362267, "precision": 0.97896508203618, "recall": 0.6973329337728499, "support": 3337}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 298}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1277}},
  "ppcr": 0.4384693189480782
}
```
</details>
