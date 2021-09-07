# Train report for javascript / file:///tmp/top-repos-quality-repos-vtewvg7z/fullstack.git HEAD 5083c5f741dcf3a068db10fa1a2025728435dc80

### Classification report

PPCR: 0.796

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.931| 0.980| 0.928| 0.955| 0.929| 8075| 8531| 0.947 |
| `␣` | 0.941| 0.905| 0.744| 0.923| 0.831| 3584| 4362| 0.822 |
| `⏎␣⁻␣⁻` | 0.945| 0.782| 0.698| 0.856| 0.803| 510| 572| 0.892 |
| `⏎` | 0.926| 0.778| 0.424| 0.846| 0.582| 433| 795| 0.545 |
| `⏎␣⁺␣⁺` | 0.965| 0.616| 0.305| 0.752| 0.464| 315| 636| 0.495 |
| `'` | 1.000| 1.000| 0.214| 1.000| 0.352| 212| 992| 0.214 |
| `"` | 1.000| 1.000| 0.225| 1.000| 0.367| 97| 431| 0.225 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 21| 331| 0.063 |
| `weighted avg` | 0.935| 0.936| 0.745| 0.933| 0.797| 13247| 16650| 0.796 |
| `macro avg` | 0.839| 0.758| 0.442| 0.791| 0.541| 13247| 16650| 0.796 |
| `micro avg` | 0.936| 0.936| 0.745| 0.936| 0.829| 13247| 16650| 0.796 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|456 |7913 |161 |0 |0 |0 |1 |0 |0 |
|778 |283 |3245 |0 |27 |7 |22 |0 |0 |
|780 |0 |0 |212 |0 |0 |0 |0 |0 |
|362 |83 |13 |0 |337 |0 |0 |0 |0 |
|321 |93 |28 |0 |0 |194 |0 |0 |0 |
|62 |110 |1 |0 |0 |0 |399 |0 |0 |
|334 |0 |0 |0 |0 |0 |0 |97 |0 |
|310 |21 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| osa5/bloglist-frontend/src/App.js | 61 |
| osa1/unicafe/src/index.js | 53 |
| osa4/blogilista/utils/list_helper.js | 47 |
| osa1/anekdootit/src/serviceWorker.js | 46 |
| osa2/kurssitiedot2/src/serviceWorker.js | 46 |
| osa1/unicafe/src/serviceWorker.js | 46 |
| osa2/maidentiedot/src/serviceWorker.js | 46 |
| osa2/puhelinluettelo/src/serviceWorker.js | 46 |
| osa2/puhelinluettelo/src/App.js | 45 |
| osa4/blogilista/tests/blog_api.test.js | 35 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 97}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 212}, "macro avg": {"f1-score": 0.7914243603123798, "precision": 0.8385292438066227, "recall": 0.757733497130626, "support": 13247}, "micro avg": {"f1-score": 0.9358345285725069, "precision": 0.9358345285725069, "recall": 0.9358345285725069, "support": 13247}, "weighted avg": {"f1-score": 0.9334321796160957, "precision": 0.934838552391067, "recall": 0.9358345285725069, "support": 13247}, "\u2205": {"f1-score": 0.9546386777657135, "precision": 0.9306127249206163, "recall": 0.979938080495356, "support": 8075}, "\u23ce": {"f1-score": 0.8456712672521958, "precision": 0.9258241758241759, "recall": 0.7782909930715936, "support": 433}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 21}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.751937984496124, "precision": 0.9651741293532339, "recall": 0.6158730158730159, "support": 315}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8562231759656653, "precision": 0.9454976303317536, "recall": 0.7823529411764706, "support": 510}, "\u2423": {"f1-score": 0.9229237770193401, "precision": 0.9411252900232019, "recall": 0.9054129464285714, "support": 3584}},
  "cl_report_full": {"\"": {"f1-score": 0.36742424242424243, "precision": 1.0, "recall": 0.22505800464037123, "support": 431}, "\u0027": {"f1-score": 0.35215946843853824, "precision": 1.0, "recall": 0.21370967741935484, "support": 992}, "macro avg": {"f1-score": 0.5409457099339867, "precision": 0.8385292438066227, "recall": 0.4420917586355907, "support": 16650}, "micro avg": {"f1-score": 0.8293139779911028, "precision": 0.9358345285725069, "recall": 0.7445645645645645, "support": 16650}, "weighted avg": {"f1-score": 0.7972868692743693, "precision": 0.9223994763950524, "recall": 0.7445645645645645, "support": 16650}, "\u2205": {"f1-score": 0.9290830104496888, "precision": 0.9306127249206163, "recall": 0.9275583167272301, "support": 8531}, "\u23ce": {"f1-score": 0.5815358067299397, "precision": 0.9258241758241759, "recall": 0.4238993710691824, "support": 795}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 331}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.4635603345280765, "precision": 0.9651741293532339, "recall": 0.3050314465408805, "support": 636}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8028169014084507, "precision": 0.9454976303317536, "recall": 0.6975524475524476, "support": 572}, "\u2423": {"f1-score": 0.8309859154929577, "precision": 0.9411252900232019, "recall": 0.743924805135259, "support": 4362}},
  "ppcr": 0.7956156156156157
}
```
</details>
