# Train report for javascript / file:///tmp/top-repos-quality-repos-ivbso1fn/hipeerlab-platform-fe.git HEAD 659a5990d2c0f263ed76228a8886bb487ff231e1

### Classification report

PPCR: 0.557

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.947| 0.997| 0.579| 0.971| 0.719| 3072| 5289| 0.581 |
| `␣` | 0.919| 0.898| 0.502| 0.909| 0.649| 874| 1564| 0.559 |
| `"` | 1.000| 0.973| 0.679| 0.986| 0.809| 662| 949| 0.698 |
| `'` | 0.966| 1.000| 0.938| 0.983| 0.952| 514| 548| 0.938 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 83| 193| 0.430 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 60| 198| 0.303 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 602| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 116| 0.000 |
| `weighted avg` | 0.925| 0.951| 0.529| 0.938| 0.645| 5265| 9459| 0.557 |
| `micro avg` | 0.951| 0.951| 0.529| 0.951| 0.680| 5265| 9459| 0.557 |
| `macro avg` | 0.479| 0.484| 0.337| 0.481| 0.391| 5265| 9459| 0.557 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2217 |3063 |9 |0 |0 |0 |0 |0 |0 |
|690 |89 |785 |0 |0 |0 |0 |0 |0 |
|34 |0 |0 |514 |0 |0 |0 |0 |0 |
|602 |0 |0 |0 |0 |0 |0 |0 |0 |
|287 |0 |0 |18 |0 |644 |0 |0 |0 |
|138 |0 |60 |0 |0 |0 |0 |0 |0 |
|110 |83 |0 |0 |0 |0 |0 |0 |0 |
|116 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/pages/Team/index.js | 62 |
| src/pages/Ventilator/index.js | 46 |
| src/pages/Faq/Drop/Respirador/index.js | 19 |
| src/components/NavBar/index.js | 18 |
| src/pages/VentilatorHipeerlab/index.js | 17 |
| src/components/Carousel/index.js | 11 |
| src/pages/Home/index.js | 10 |
| src/index.js | 10 |
| src/pages/Donate/index.js | 9 |
| src/pages/Faq/Drop/Gerais/index.js | 8 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9862174578866768, "precision": 1.0, "recall": 0.972809667673716, "support": 662}, "\u0027": {"f1-score": 0.982791586998088, "precision": 0.9661654135338346, "recall": 1.0, "support": 514}, "macro avg": {"f1-score": 0.4811094484922556, "precision": 0.4790250863431922, "recall": 0.48350616456976947, "support": 5265}, "micro avg": {"f1-score": 0.9508072174738842, "precision": 0.9508072174738842, "recall": 0.9508072174738842, "support": 5265}, "weighted avg": {"f1-score": 0.9375033979363173, "precision": 0.9251015305005069, "recall": 0.9508072174738842, "support": 5265}, "\u2205": {"f1-score": 0.9713017282384652, "precision": 0.9468315301391036, "recall": 0.9970703125, "support": 3072}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 60}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 83}, "\u2423": {"f1-score": 0.9085648148148149, "precision": 0.9192037470725996, "recall": 0.8981693363844394, "support": 874}},
  "cl_report_full": {"\"": {"f1-score": 0.8085373509102323, "precision": 1.0, "recall": 0.678609062170706, "support": 949}, "\u0027": {"f1-score": 0.9518518518518518, "precision": 0.9661654135338346, "recall": 0.9379562043795621, "support": 548}, "macro avg": {"f1-score": 0.39104535249971833, "precision": 0.4790250863431922, "recall": 0.3372012392571689, "support": 9459}, "micro avg": {"f1-score": 0.6799782667753327, "precision": 0.9508072174738842, "recall": 0.5292314198118194, "support": 9459}, "weighted avg": {"f1-score": 0.6454697243160306, "precision": 0.8377085600955498, "recall": 0.5292314198118194, "support": 9459}, "\u2205": {"f1-score": 0.7186766776161426, "precision": 0.9468315301391036, "recall": 0.579126488939308, "support": 5289}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 602}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 116}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 198}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 193}, "\u2423": {"f1-score": 0.6492969396195202, "precision": 0.9192037470725996, "recall": 0.5019181585677749, "support": 1564}},
  "ppcr": 0.5566127497621313
}
```
</details>
