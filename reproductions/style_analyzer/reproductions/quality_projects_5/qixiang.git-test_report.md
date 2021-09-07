# Test report for javascript / file:///tmp/top-repos-quality-repos-mw8neqtd/qixiang.git HEAD d888c3b4f626ebe7fd86b81b2055358ee458cf56

### Classification report

PPCR: 0.570

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `␣` | 0.962| 0.846| 0.725| 0.900| 0.827| 240| 280| 0.857 |
| `∅` | 0.804| 0.976| 0.477| 0.882| 0.599| 168| 344| 0.488 |
| `'` | 0.694| 0.704| 0.500| 0.699| 0.581| 71| 100| 0.710 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 54| 0.093 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 38| 0.026 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 19| 0.053 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 19| 0.053 |
| `micro avg` | 0.856| 0.856| 0.488| 0.856| 0.622| 487| 854| 0.570 |
| `macro avg` | 0.351| 0.361| 0.243| 0.354| 0.287| 487| 854| 0.570 |
| `weighted avg` | 0.853| 0.856| 0.488| 0.850| 0.580| 487| 854| 0.570 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|176 |164 |4 |0 |0 |0 |0 |0 |
|40 |15 |203 |22 |0 |0 |0 |0 |
|29 |20 |1 |50 |0 |0 |0 |0 |
|49 |5 |0 |0 |0 |0 |0 |0 |
|18 |0 |1 |0 |0 |0 |0 |0 |
|18 |0 |1 |0 |0 |0 |0 |0 |
|37 |0 |1 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.6993006993006993, "precision": 0.6944444444444444, "recall": 0.704225352112676, "support": 71}, "macro avg": {"f1-score": 0.3544632655568926, "precision": 0.3514930458755382, "recall": 0.3608927373766408, "support": 487}, "micro avg": {"f1-score": 0.8562628336755647, "precision": 0.8562628336755647, "recall": 0.8562628336755647, "support": 487}, "weighted avg": {"f1-score": 0.8497589260493215, "precision": 0.8526999035289868, "recall": 0.8562628336755647, "support": 487}, "\u2205": {"f1-score": 0.8817204301075269, "precision": 0.803921568627451, "recall": 0.9761904761904762, "support": 168}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.9002217294900223, "precision": 0.9620853080568721, "recall": 0.8458333333333333, "support": 240}},
  "cl_report_full": {"\u0027": {"f1-score": 0.5813953488372093, "precision": 0.6944444444444444, "recall": 0.5, "support": 100}, "macro avg": {"f1-score": 0.2866884864585109, "precision": 0.3514930458755382, "recall": 0.24310631229235882, "support": 854}, "micro avg": {"f1-score": 0.6219239373601789, "precision": 0.8562628336755647, "recall": 0.4882903981264637, "support": 854}, "weighted avg": {"f1-score": 0.58028669790521, "precision": 0.7205823774100841, "recall": 0.4882903981264637, "support": 854}, "\u2205": {"f1-score": 0.5985401459854015, "precision": 0.803921568627451, "recall": 0.47674418604651164, "support": 344}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 54}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 38}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 19}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 19}, "\u2423": {"f1-score": 0.8268839103869653, "precision": 0.9620853080568721, "recall": 0.725, "support": 280}},
  "ppcr": 0.5702576112412178
}
```
</details>
