# Train report for javascript / file:///tmp/top-repos-quality-repos-_madopii/plugin-battle-detail.git HEAD d8c7a8ae65e1787b449ebf7681c2261c0a7ec2ee

### Classification report

PPCR: 0.482

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.978| 0.978| 0.580| 0.978| 0.728| 3036| 5114| 0.594 |
| `␣` | 0.943| 0.998| 0.407| 0.970| 0.568| 1264| 3102| 0.407 |
| `'` | 1.000| 1.000| 0.727| 1.000| 0.842| 516| 710| 0.727 |
| `⏎␣⁻␣⁻` | 0.940| 0.804| 0.597| 0.867| 0.730| 271| 365| 0.742 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 34| 417| 0.082 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 748| 0.001 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 167| 0.000 |
| `weighted avg` | 0.963| 0.969| 0.467| 0.965| 0.598| 5122| 10623| 0.482 |
| `macro avg` | 0.551| 0.540| 0.330| 0.545| 0.410| 5122| 10623| 0.482 |
| `micro avg` | 0.969| 0.969| 0.467| 0.969| 0.631| 5122| 10623| 0.482 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|2078 |2968 |55 |0 |0 |0 |13 |0 |
|1838 |1 |1262 |0 |0 |0 |1 |0 |
|194 |0 |0 |516 |0 |0 |0 |0 |
|747 |1 |0 |0 |0 |0 |0 |0 |
|383 |13 |21 |0 |0 |0 |0 |0 |
|94 |53 |0 |0 |0 |0 |218 |0 |
|167 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| lib/convert-replay.es | 36 |
| lib/csv-stringify/sync.js | 22 |
| lib/wctf.es | 20 |
| lib/csv-parse/sync.js | 16 |
| lib/deck-builder.es | 14 |
| views/option-area.es | 11 |
| lib/compat.es | 8 |
| views/browse-area/sortie-viewer/groupping.es | 7 |
| views/selectors.es | 7 |
| views/modal-area.es | 4 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 516}, "macro avg": {"f1-score": 0.5449177674276434, "precision": 0.5514937263765765, "recall": 0.5400639819766173, "support": 5122}, "micro avg": {"f1-score": 0.9691526747364311, "precision": 0.9691526747364311, "recall": 0.9691526747364311, "support": 5122}, "weighted avg": {"f1-score": 0.9654454766308449, "precision": 0.9626805623222221, "recall": 0.9691526747364311, "support": 5122}, "\u2205": {"f1-score": 0.9776021080368906, "precision": 0.9776021080368906, "recall": 0.9776021080368906, "support": 3036}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 34}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8667992047713718, "precision": 0.9396551724137931, "recall": 0.8044280442804428, "support": 271}, "\u2423": {"f1-score": 0.9700230591852421, "precision": 0.9431988041853513, "recall": 0.9984177215189873, "support": 1264}},
  "cl_report_full": {"\u0027": {"f1-score": 0.8417618270799349, "precision": 1.0, "recall": 0.7267605633802817, "support": 710}, "macro avg": {"f1-score": 0.40984173025529447, "precision": 0.5514937263765765, "recall": 0.3301746794438435, "support": 10623}, "micro avg": {"f1-score": 0.6305493807557956, "precision": 0.9691526747364311, "recall": 0.46728796008660456, "support": 10623}, "weighted avg": {"f1-score": 0.5979812866069153, "precision": 0.8451693503732142, "recall": 0.46728796008660456, "support": 10623}, "\u2205": {"f1-score": 0.7283435582822086, "precision": 0.9776021080368906, "recall": 0.5803676183026985, "support": 5114}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 748}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 167}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 417}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7303182579564489, "precision": 0.9396551724137931, "recall": 0.5972602739726027, "support": 365}, "\u2423": {"f1-score": 0.5684684684684685, "precision": 0.9431988041853513, "recall": 0.40683430045132174, "support": 3102}},
  "ppcr": 0.4821613480184505
}
```
</details>
