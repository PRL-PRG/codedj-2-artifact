# Train report for javascript / file:///tmp/top-repos-quality-repos-gwwnr072/disa_dj_project.git HEAD 37db0a60fe4fcdd32595ce32652cfd30acbdc61d

### Classification report

PPCR: 0.272

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.994| 1.000| 0.529| 0.997| 0.691| 11963| 22588| 0.530 |
| `␣` | 0.996| 0.986| 0.130| 0.991| 0.230| 1716| 13016| 0.132 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 16| 2974| 0.005 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 14| 1399| 0.010 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 1366| 0.006 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 244| 0.025 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 6406| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 907| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1398| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 223| 0.000 |
| `macro avg` | 0.199| 0.199| 0.066| 0.199| 0.092| 13723| 50521| 0.272 |
| `micro avg` | 0.995| 0.995| 0.270| 0.995| 0.425| 13723| 50521| 0.272 |
| `weighted avg` | 0.991| 0.995| 0.270| 0.993| 0.368| 13723| 50521| 0.272 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|10625 |11958 |5 |0 |0 |0 |0 |0 |0 |0 |0 |
|11300 |24 |1692 |0 |0 |0 |0 |0 |0 |0 |0 |
|2958 |16 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|6406 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|907 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1385 |13 |1 |0 |0 |0 |0 |0 |0 |0 |0 |
|1358 |8 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1398 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|238 |6 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|223 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| disa_app/static/lib/slickgrid/lib/jquery.event.drag-2.3.0.js | 22 |
| disa_app/static/js/browse_tabulator.js | 19 |
| disa_app/static/lib/select2-dist/js/select2.js | 13 |
| disa_app/static/js/browse.js | 6 |
| disa_app/static/js/editor/reference.locations.mgmt.js | 6 |
| disa_app/static/js/redesign_citation.js | 3 |
| disa_app/static/js/editor/editor.relationships.store.js | 1 |
| disa_app/static/js/editor/editor.relationships.adder.js | 1 |
| disa_app/static/js/js_demo_3.js | 1 |
| disa_app/static/js/hilitor.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.19882111530281837, "precision": 0.1990894705523681, "recall": 0.19855960306516188, "support": 13723}, "micro avg": {"f1-score": 0.9946804634555126, "precision": 0.9946804634555126, "recall": 0.9946804634555126, "support": 13723}, "weighted avg": {"f1-score": 0.9930783327557581, "precision": 0.9914947054394202, "recall": 0.9946804634555126, "support": 13723}, "\u2205": {"f1-score": 0.9969984992496248, "precision": 0.9944282744282744, "recall": 0.9995820446376327, "support": 11963}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9912126537785588, "precision": 0.9964664310954063, "recall": 0.986013986013986, "support": 1716}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 907}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6406}, "macro avg": {"f1-score": 0.09209393139916706, "precision": 0.1990894705523681, "recall": 0.06593899932616205, "support": 50521}, "micro avg": {"f1-score": 0.42494240707303405, "precision": 0.9946804634555126, "recall": 0.27018467567942045, "support": 50521}, "weighted avg": {"f1-score": 0.36817878392277775, "precision": 0.701335146373264, "recall": 0.27018467567942045, "support": 50521}, "\u2205": {"f1-score": 0.6909542657383064, "precision": 0.9944282744282744, "recall": 0.5293961395431203, "support": 22588}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2974}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1398}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1399}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 244}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1366}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 223}, "\u2423": {"f1-score": 0.22998504825336416, "precision": 0.9964664310954063, "recall": 0.1299938537185003, "support": 13016}},
  "ppcr": 0.27162961936620417
}
```
</details>
