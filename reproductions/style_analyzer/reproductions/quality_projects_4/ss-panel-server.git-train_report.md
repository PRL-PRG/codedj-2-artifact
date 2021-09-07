# Train report for javascript / file:///tmp/top-repos-quality-repos-yfwh2k4w/ss-panel-server.git HEAD 609c634ed0f55d7413c6073b19230c73aa96a9f6

### Classification report

PPCR: 0.849

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.943| 0.992| 0.941| 0.967| 0.942| 9365| 9873| 0.949 |
| `␣` | 0.971| 0.946| 0.821| 0.958| 0.890| 4832| 5564| 0.868 |
| `'` | 1.000| 1.000| 0.994| 1.000| 0.997| 1726| 1736| 0.994 |
| `⏎␣⁻␣⁻` | 0.998| 0.778| 0.760| 0.874| 0.863| 679| 695| 0.977 |
| `⏎` | 0.948| 0.713| 0.309| 0.814| 0.466| 464| 1072| 0.433 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 65| 778| 0.084 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 32| 494| 0.065 |
| `macro avg` | 0.694| 0.633| 0.546| 0.659| 0.594| 17163| 20212| 0.849 |
| `weighted avg` | 0.954| 0.958| 0.814| 0.955| 0.845| 17163| 20212| 0.849 |
| `micro avg` | 0.958| 0.958| 0.814| 0.958| 0.880| 17163| 20212| 0.849 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|508 |9294 |71 |0 |0 |0 |0 |0 |
|732 |259 |4569 |0 |3 |0 |1 |0 |
|10 |0 |0 |1726 |0 |0 |0 |0 |
|608 |86 |47 |0 |331 |0 |0 |0 |
|713 |64 |1 |0 |0 |0 |0 |0 |
|16 |148 |3 |0 |0 |0 |528 |0 |
|462 |3 |14 |0 |15 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| test/api/nodes.test.js | 47 |
| src/service/traffic.js | 45 |
| test/api/node_tokens.test.js | 42 |
| src/lib/errors.js | 40 |
| test/api/users.test.js | 37 |
| test/lib/random.js | 22 |
| config/docker.js | 22 |
| src/service/user.js | 18 |
| test/api/profile.test.js | 17 |
| src/service/node_token.js | 17 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1726}, "macro avg": {"f1-score": 0.6591101684561345, "precision": 0.6943997965730946, "recall": 0.6327094256108359, "support": 17163}, "micro avg": {"f1-score": 0.9583406164423469, "precision": 0.9583406164423469, "recall": 0.9583406164423469, "support": 17163}, "weighted avg": {"f1-score": 0.9546556036749614, "precision": 0.9537320955416666, "recall": 0.9583406164423469, "support": 17163}, "\u2205": {"f1-score": 0.9671679067589364, "precision": 0.9431702861782018, "recall": 0.992418579818473, "support": 9365}, "\u23ce": {"f1-score": 0.8142681426814269, "precision": 0.9484240687679083, "recall": 0.7133620689655172, "support": 464}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 32}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 65}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8741721854304636, "precision": 0.998109640831758, "recall": 0.7776141384388807, "support": 679}, "\u2423": {"f1-score": 0.9581629443221139, "precision": 0.9710945802337938, "recall": 0.9455711920529801, "support": 4832}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9971114962449451, "precision": 1.0, "recall": 0.9942396313364056, "support": 1736}, "macro avg": {"f1-score": 0.5939786098113083, "precision": 0.6943997965730946, "recall": 0.5464639354694333, "support": 20212}, "micro avg": {"f1-score": 0.8801605351170568, "precision": 0.9583406164423469, "recall": 0.8137739956461508, "support": 20212}, "weighted avg": {"f1-score": 0.8452480010497834, "precision": 0.8985497368867745, "recall": 0.8137739956461508, "support": 20212}, "\u2205": {"f1-score": 0.9422618745881279, "precision": 0.9431702861782018, "recall": 0.9413552111820116, "support": 9873}, "\u23ce": {"f1-score": 0.4658691062631949, "precision": 0.9484240687679083, "recall": 0.3087686567164179, "support": 1072}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 494}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 778}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8627450980392156, "precision": 0.998109640831758, "recall": 0.7597122302158273, "support": 695}, "\u2423": {"f1-score": 0.8898626935436752, "precision": 0.9710945802337938, "recall": 0.8211718188353703, "support": 5564}},
  "ppcr": 0.8491490203839304
}
```
</details>
