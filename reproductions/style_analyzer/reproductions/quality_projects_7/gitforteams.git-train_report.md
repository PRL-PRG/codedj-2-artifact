# Train report for javascript / file:///tmp/top-repos-quality-repos-2avdd576/gitforteams.git HEAD 0e320375bc3d8813bdddb37bc0f0a17e9b561030

### Classification report

PPCR: 0.468

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.955| 0.995| 0.934| 0.974| 0.944| 14565| 15511| 0.939 |
| `␣` | 0.959| 0.729| 0.129| 0.828| 0.227| 2383| 13470| 0.177 |
| `⏎` | 0.972| 0.945| 0.178| 0.958| 0.300| 364| 1936| 0.188 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 19| 1033| 0.018 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 1001| 0.006 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 913| 0.007 |
| `⏎⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 236| 0.004 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1874| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 880| 0.000 |
| `⏎⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 176| 0.000 |
| `micro avg` | 0.956| 0.956| 0.448| 0.956| 0.610| 17344| 37030| 0.468 |
| `weighted avg` | 0.954| 0.956| 0.448| 0.952| 0.494| 17344| 37030| 0.468 |
| `macro avg` | 0.289| 0.267| 0.124| 0.276| 0.147| 17344| 37030| 0.468 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎⇥⁺| "| ⏎⏎| ⏎⇥⁻| ⏎⏎⇥⁻| ⏎⏎⇥⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|946 |14492 |73 |0 |0 |0 |0 |0 |0 |0 |0 |
|11087 |646 |1737 |0 |0 |0 |0 |0 |0 |0 |0 |
|1572 |19 |1 |344 |0 |0 |0 |0 |0 |0 |0 |
|1874 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|995 |4 |1 |1 |0 |0 |0 |0 |0 |0 |0 |
|880 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1014 |11 |0 |8 |0 |0 |0 |0 |0 |0 |0 |
|907 |6 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|235 |0 |0 |1 |0 |0 |0 |0 |0 |0 |0 |
|176 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| slides/lib/reveal.js/test/qunit-1.12.0.js | 396 |
| slides/lib/reveal.js/js/reveal.js | 159 |
| slides/lib/reveal.js/test/test.js | 42 |
| slides/lib/reveal.js/plugin/remotes/remotes.js | 26 |
| slides/lib/reveal.js/Gruntfile.js | 24 |
| slides/lib/reveal.js/plugin/markdown/markdown.js | 22 |
| slides/lib/reveal.js/test/test-markdown-slide-attributes.js | 17 |
| slides/lib/reveal.js/plugin/notes-server/client.js | 14 |
| slides/lib/reveal.js/test/test-markdown-element-attributes.js | 11 |
| slides/lib/reveal.js/plugin/multiplex/master.js | 11 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.2760827385446511, "precision": 0.2885163688300956, "recall": 0.26689560646543964, "support": 17344}, "micro avg": {"f1-score": 0.9555465867158671, "precision": 0.9555465867158671, "recall": 0.9555465867158671, "support": 17344}, "weighted avg": {"f1-score": 0.9522338152117354, "precision": 0.9539200395813227, "recall": 0.9555465867158671, "support": 17344}, "\u2205": {"f1-score": 0.974481390579296, "precision": 0.954803004348399, "recall": 0.9949879848952969, "support": 14565}, "\u23ce": {"f1-score": 0.958217270194986, "precision": 0.9717514124293786, "recall": 0.945054945054945, "support": 364}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 19}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.8281287246722289, "precision": 0.9586092715231788, "recall": 0.7289131347041544, "support": 2383}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 880}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1874}, "macro avg": {"f1-score": 0.1472205581586826, "precision": 0.2885163688300956, "recall": 0.1240943866808212, "support": 37030}, "micro avg": {"f1-score": 0.6095928200978409, "precision": 0.9555465867158671, "recall": 0.4475560356467729, "support": 37030}, "weighted avg": {"f1-score": 0.4940043186359693, "precision": 0.7994525255827305, "recall": 0.4475560356467729, "support": 37030}, "\u2205": {"f1-score": 0.9444426341685946, "precision": 0.954803004348399, "recall": 0.9343046869963252, "support": 15511}, "\u23ce": {"f1-score": 0.3004366812227074, "precision": 0.9717514124293786, "recall": 0.17768595041322313, "support": 1936}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1001}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 913}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1033}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 176}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 236}, "\u2423": {"f1-score": 0.22732626619552415, "precision": 0.9586092715231788, "recall": 0.1289532293986637, "support": 13470}},
  "ppcr": 0.4683769916284094
}
```
</details>
