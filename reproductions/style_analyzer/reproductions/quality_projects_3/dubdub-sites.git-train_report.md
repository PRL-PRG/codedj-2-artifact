# Train report for javascript / file:///tmp/top-repos-quality-repos-mhpdq1pj/dubdub-sites.git HEAD f86014e831aec5000fe943db5b48cfde8fcbff53

### Classification report

PPCR: 0.486

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.964| 0.999| 0.760| 0.981| 0.850| 5272| 6926| 0.761 |
| `'` | 1.000| 1.000| 0.742| 1.000| 0.852| 741| 998| 0.742 |
| `␣` | 0.968| 0.700| 0.061| 0.813| 0.115| 257| 2956| 0.087 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 98| 419| 0.234 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 12| 439| 0.027 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 791| 0.011 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 408| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 216| 0.000 |
| `macro avg` | 0.366| 0.337| 0.195| 0.349| 0.227| 6389| 13153| 0.486 |
| `weighted avg` | 0.950| 0.968| 0.470| 0.958| 0.538| 6389| 13153| 0.486 |
| `micro avg` | 0.968| 0.968| 0.470| 0.968| 0.633| 6389| 13153| 0.486 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| "| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1654 |5266 |6 |0 |0 |0 |0 |0 |0 |
|2699 |77 |180 |0 |0 |0 |0 |0 |0 |
|257 |0 |0 |741 |0 |0 |0 |0 |0 |
|782 |9 |0 |0 |0 |0 |0 |0 |0 |
|427 |12 |0 |0 |0 |0 |0 |0 |0 |
|408 |0 |0 |0 |0 |0 |0 |0 |0 |
|321 |98 |0 |0 |0 |0 |0 |0 |0 |
|216 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/components/Meta.js | 21 |
| src/components/FormControlled.js | 18 |
| src/components/FormSimpleAjax.js | 17 |
| src/components/Gallery.js | 13 |
| src/components/Image.js | 11 |
| src/templates/SinglePost.js | 11 |
| src/templates/ContactPage.js | 10 |
| src/templates/BlogIndex.js | 10 |
| src/components/Nav.js | 10 |
| src/components/InstagramFeed.js | 9 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 741}, "macro avg": {"f1-score": 0.34922779707610385, "precision": 0.36648220550194305, "recall": 0.33740637713077826, "support": 6389}, "micro avg": {"f1-score": 0.9683831585537643, "precision": 0.9683831585537643, "recall": 0.9683831585537643, "support": 6389}, "weighted avg": {"f1-score": 0.958309052374661, "precision": 0.950466065549904, "recall": 0.9683831585537643, "support": 6389}, "\u2205": {"f1-score": 0.9811812930873859, "precision": 0.9641157085316734, "recall": 0.9988619119878603, "support": 5272}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 98}, "\u2423": {"f1-score": 0.8126410835214447, "precision": 0.967741935483871, "recall": 0.7003891050583657, "support": 257}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 408}, "\u0027": {"f1-score": 0.8522139160437033, "precision": 1.0, "recall": 0.7424849699398798, "support": 998}, "macro avg": {"f1-score": 0.22712102624976505, "precision": 0.36648220550194305, "recall": 0.1954626859653605, "support": 13153}, "micro avg": {"f1-score": 0.6332002865622762, "precision": 0.9683831585537643, "recall": 0.4703869839580324, "support": 13153}, "weighted avg": {"f1-score": 0.5380923149299461, "precision": 0.801042390221295, "recall": 0.4703869839580324, "support": 13153}, "\u2205": {"f1-score": 0.850177591217307, "precision": 0.9641157085316734, "recall": 0.7603234190008663, "support": 6926}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 791}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 216}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 439}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 419}, "\u2423": {"f1-score": 0.11457670273711011, "precision": 0.967741935483871, "recall": 0.06089309878213803, "support": 2956}},
  "ppcr": 0.48574469702729417
}
```
</details>
