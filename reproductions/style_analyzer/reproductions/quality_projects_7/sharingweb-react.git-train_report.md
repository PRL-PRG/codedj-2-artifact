# Train report for javascript / file:///tmp/top-repos-quality-repos-5jzssn9v/sharingweb-react.git HEAD 7ca4eb61cad869853b7b72839506e954b82d570a

### Classification report

PPCR: 0.684

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.980| 0.997| 0.878| 0.989| 0.927| 10890| 12359| 0.881 |
| `␣` | 0.975| 0.887| 0.406| 0.929| 0.573| 1626| 3550| 0.458 |
| `"` | 1.000| 1.000| 0.494| 1.000| 0.662| 805| 1628| 0.494 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 19| 323| 0.059 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 795| 0.013 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 264| 0.038 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 442| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 177| 0.000 |
| `micro avg` | 0.981| 0.981| 0.671| 0.981| 0.797| 13360| 19538| 0.684 |
| `weighted avg` | 0.978| 0.981| 0.671| 0.979| 0.745| 13360| 19538| 0.684 |
| `macro avg` | 0.369| 0.360| 0.222| 0.365| 0.270| 13360| 19538| 0.684 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1469 |10857 |33 |0 |0 |0 |0 |0 |0 |
|1924 |184 |1442 |0 |0 |0 |0 |0 |0 |
|823 |0 |0 |805 |0 |0 |0 |0 |0 |
|785 |8 |2 |0 |0 |0 |0 |0 |0 |
|442 |0 |0 |0 |0 |0 |0 |0 |0 |
|304 |17 |2 |0 |0 |0 |0 |0 |0 |
|254 |10 |0 |0 |0 |0 |0 |0 |0 |
|177 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| sharingweb-web/src/contexts/ShopStore.js | 22 |
| sharingweb-web/src/scenes/shoplanding/components/Checkout.js | 21 |
| sharingweb-web/src/scenes/shoplanding/components/Cart.js | 17 |
| sharingweb-web/src/scenes/shoplanding/ShopLanding.js | 15 |
| sharingweb-web/src/scenes/shoplanding/components/DetailProduct.js | 15 |
| sharingweb-web/src/scenes/home/components/Examples.js | 15 |
| sharingweb-web/src/scenes/shoplanding/components/EditForm.js | 12 |
| sharingweb-web/src/components/Register.js | 11 |
| sharingweb-web/src/components/Login.js | 11 |
| sharingweb-web/src/scenes/home/components/Footer.js | 10 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 805}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.36466902516449445, "precision": 0.369401326955858, "recall": 0.3604760706697976, "support": 13360}, "micro avg": {"f1-score": 0.9808383233532935, "precision": 0.9808383233532935, "recall": 0.9808383233532935, "support": 13360}, "weighted avg": {"f1-score": 0.979067029988019, "precision": 0.9779191763986855, "recall": 0.9808383233532935, "support": 13360}, "\u2205": {"f1-score": 0.988527724665392, "precision": 0.9802275189599133, "recall": 0.996969696969697, "support": 10890}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 19}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u2423": {"f1-score": 0.9288244766505637, "precision": 0.9749830966869506, "recall": 0.8868388683886839, "support": 1626}},
  "cl_report_full": {"\"": {"f1-score": 0.6617344841759144, "precision": 1.0, "recall": 0.49447174447174447, "support": 1628}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 442}, "macro avg": {"f1-score": 0.2702213961506471, "precision": 0.369401326955858, "recall": 0.22239225742213958, "support": 19538}, "micro avg": {"f1-score": 0.7966441728980486, "precision": 0.9808383233532935, "recall": 0.6706930084962637, "support": 19538}, "weighted avg": {"f1-score": 0.7454461035540406, "precision": 0.8805313696419409, "recall": 0.6706930084962637, "support": 19538}, "\u2205": {"f1-score": 0.9265628333688927, "precision": 0.9802275189599133, "recall": 0.8784691318067804, "support": 12359}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 795}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 177}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 323}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 264}, "\u2423": {"f1-score": 0.5734738516603698, "precision": 0.9749830966869506, "recall": 0.40619718309859154, "support": 3550}},
  "ppcr": 0.6837956802129184
}
```
</details>
