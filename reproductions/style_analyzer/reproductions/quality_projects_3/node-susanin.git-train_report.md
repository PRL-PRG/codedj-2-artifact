# Train report for javascript / file:///tmp/top-repos-quality-repos-anl6kymb/node-susanin.git HEAD 6f35e4da69815d6250e16655af8a32bcaded0ea7

### Classification report

PPCR: 0.676

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.950| 0.996| 0.827| 0.972| 0.884| 3548| 4271| 0.831 |
| `␣` | 0.957| 0.896| 0.541| 0.926| 0.692| 1456| 2410| 0.604 |
| `'` | 1.000| 1.000| 0.820| 1.000| 0.901| 528| 644| 0.820 |
| `⏎` | 0.938| 0.965| 0.589| 0.951| 0.723| 423| 693| 0.610 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 76| 382| 0.199 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 14| 308| 0.045 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 230| 0.000 |
| `macro avg` | 0.549| 0.551| 0.397| 0.550| 0.457| 6045| 8938| 0.676 |
| `weighted avg` | 0.941| 0.955| 0.646| 0.948| 0.730| 6045| 8938| 0.676 |
| `micro avg` | 0.955| 0.955| 0.646| 0.955| 0.771| 6045| 8938| 0.676 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|723 |3533 |15 |0 |0 |0 |0 |0 |
|954 |124 |1305 |27 |0 |0 |0 |0 |
|270 |5 |10 |408 |0 |0 |0 |0 |
|116 |0 |0 |0 |528 |0 |0 |0 |
|306 |57 |19 |0 |0 |0 |0 |0 |
|294 |0 |14 |0 |0 |0 |0 |0 |
|230 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| cli/lib/documentation-generator/generators/apidoc/index.js | 63 |
| cli/lib/documentation-generator/types-parsers/common.js | 19 |
| lib/createRouter.js | 17 |
| cli/lib/documentation-generator/apidoc-parser.js | 16 |
| cli/lib/documentation-generator/routes-ast.js | 16 |
| cli/lib/documentation-generator/index.js | 14 |
| test/router-params.test.js | 10 |
| cli/lib/documentation-generator/generators/apidoc/types.js | 9 |
| test/router.test.js | 8 |
| test/routes/controller-params.js | 8 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 528}, "macro avg": {"f1-score": 0.5498928433557431, "precision": 0.5493377712170109, "recall": 0.5509432117069709, "support": 6045}, "micro avg": {"f1-score": 0.9551695616211745, "precision": 0.9551695616211745, "recall": 0.9551695616211745, "support": 6045}, "weighted avg": {"f1-score": 0.9475948843282971, "precision": 0.9411645458697776, "recall": 0.9551695616211745, "support": 6045}, "\u2205": {"f1-score": 0.9723407183156737, "precision": 0.9499865555256789, "recall": 0.9957722660653889, "support": 3548}, "\u23ce": {"f1-score": 0.951048951048951, "precision": 0.9379310344827586, "recall": 0.9645390070921985, "support": 423}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 76}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u2423": {"f1-score": 0.9258602341255765, "precision": 0.9574468085106383, "recall": 0.8962912087912088, "support": 1456}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9010238907849829, "precision": 1.0, "recall": 0.8198757763975155, "support": 644}, "macro avg": {"f1-score": 0.45722011611128305, "precision": 0.5493377712170109, "recall": 0.3967601263181716, "support": 8938}, "micro avg": {"f1-score": 0.7707401721951544, "precision": 0.9551695616211745, "recall": 0.6460058178563437, "support": 8938}, "weighted avg": {"f1-score": 0.7301180961222198, "precision": 0.8568835974555119, "recall": 0.6460058178563437, "support": 8938}, "\u2205": {"f1-score": 0.8843554443053817, "precision": 0.9499865555256789, "recall": 0.8272067431514868, "support": 4271}, "\u23ce": {"f1-score": 0.7234042553191489, "precision": 0.9379310344827586, "recall": 0.5887445887445888, "support": 693}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 230}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 382}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 308}, "\u2423": {"f1-score": 0.6917572223694674, "precision": 0.9574468085106383, "recall": 0.5414937759336099, "support": 2410}},
  "ppcr": 0.6763257999552472
}
```
</details>
