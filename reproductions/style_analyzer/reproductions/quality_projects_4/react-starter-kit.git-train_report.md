# Train report for javascript / file:///tmp/top-repos-quality-repos-jq9lowhm/react-starter-kit.git HEAD b821a03895ff6901722ed6f4be100699cc72a674

### Classification report

PPCR: 0.054

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `'` | 1.000| 1.000| 0.364| 1.000| 0.534| 274| 752| 0.364 |
| `␣` | 0.903| 1.000| 0.091| 0.949| 0.165| 196| 2161| 0.091 |
| `∅` | 0.000| 0.000| 0.000| 0.000| 0.000| 19| 4685| 0.004 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 282| 0.007 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 569| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 278| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 258| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 114| 0.000 |
| `macro avg` | 0.238| 0.250| 0.057| 0.244| 0.087| 491| 9099| 0.054 |
| `weighted avg` | 0.919| 0.957| 0.052| 0.937| 0.083| 491| 9099| 0.054 |
| `micro avg` | 0.957| 0.957| 0.052| 0.957| 0.098| 491| 9099| 0.054 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|4666 |0 |19 |0 |0 |0 |0 |0 |0 |
|1965 |0 |196 |0 |0 |0 |0 |0 |0 |
|478 |0 |0 |274 |0 |0 |0 |0 |0 |
|569 |0 |0 |0 |0 |0 |0 |0 |0 |
|278 |0 |0 |0 |0 |0 |0 |0 |0 |
|280 |0 |2 |0 |0 |0 |0 |0 |0 |
|258 |0 |0 |0 |0 |0 |0 |0 |0 |
|114 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| test/setup.js | 7 |
| src/client.js | 3 |
| src/DOMUtils.js | 3 |
| src/server.js | 3 |
| src/components/Layout/Layout.test.js | 2 |
| src/routes/index.js | 1 |
| src/passport.js | 1 |
| src/components/Link/Link.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 274}, "macro avg": {"f1-score": 0.24364406779661016, "precision": 0.23790322580645162, "recall": 0.25, "support": 491}, "micro avg": {"f1-score": 0.9572301425661914, "precision": 0.9572301425661914, "recall": 0.9572301425661914, "support": 491}, "weighted avg": {"f1-score": 0.9369325831060789, "precision": 0.9185993035937191, "recall": 0.9572301425661914, "support": 491}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 19}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u2423": {"f1-score": 0.9491525423728813, "precision": 0.9032258064516129, "recall": 1.0, "support": 196}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 114}, "\u0027": {"f1-score": 0.5341130604288499, "precision": 1.0, "recall": 0.36436170212765956, "support": 752}, "macro avg": {"f1-score": 0.08736968343670128, "precision": 0.23790322580645162, "recall": 0.05688255658826193, "support": 9099}, "micro avg": {"f1-score": 0.09801876955161627, "precision": 0.9572301425661914, "recall": 0.05165402791515551, "support": 9099}, "weighted avg": {"f1-score": 0.08329286571155535, "precision": 0.2971613328653627, "recall": 0.05165402791515551, "support": 9099}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4685}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 569}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 258}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 278}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 282}, "\u2423": {"f1-score": 0.1648444070647603, "precision": 0.9032258064516129, "recall": 0.0906987505784359, "support": 2161}},
  "ppcr": 0.053961973843279484
}
```
</details>
