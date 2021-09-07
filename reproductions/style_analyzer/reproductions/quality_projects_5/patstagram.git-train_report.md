# Train report for javascript / file:///tmp/top-repos-quality-repos-_eymforx/patstagram.git HEAD d20b076a10b70613fc513008351572da4a1b43aa

### Classification report

PPCR: 0.417

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.960| 0.984| 0.563| 0.972| 0.710| 3815| 6666| 0.572 |
| `␣` | 0.925| 0.959| 0.380| 0.941| 0.539| 1307| 3295| 0.397 |
| `⏎` | 0.938| 0.898| 0.283| 0.917| 0.435| 322| 1021| 0.315 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 38| 241| 0.158 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 33| 262| 0.126 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 30| 249| 0.120 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 22| 204| 0.108 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 260| 0.031 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 848| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 320| 0.000 |
| `micro avg` | 0.950| 0.950| 0.396| 0.950| 0.559| 5575| 13366| 0.417 |
| `macro avg` | 0.282| 0.284| 0.123| 0.283| 0.168| 5575| 13366| 0.417 |
| `weighted avg` | 0.928| 0.950| 0.396| 0.939| 0.520| 5575| 13366| 0.417 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| '| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺| ⏎⏎| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2851 |3755 |60 |0 |0 |0 |0 |0 |0 |0 |0 |
|1988 |54 |1253 |0 |0 |0 |0 |0 |0 |0 |0 |
|699 |25 |8 |289 |0 |0 |0 |0 |0 |0 |0 |
|848 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|320 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|252 |2 |6 |0 |0 |0 |0 |0 |0 |0 |0 |
|219 |30 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|229 |11 |22 |0 |0 |0 |0 |0 |0 |0 |0 |
|182 |0 |5 |17 |0 |0 |0 |0 |0 |0 |0 |
|203 |35 |1 |2 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| frontend/src/redux/modules/user.js | 75 |
| frontend/src/redux/modules/photos.js | 35 |
| frontend/config/webpack.config.js | 27 |
| frontend/src/components/FeedPhoto/presenter.js | 22 |
| frontend/src/components/Auth/presenter.js | 11 |
| frontend/config/env.js | 9 |
| frontend/src/components/PhotoComments/index.js | 7 |
| frontend/src/components/Navigation/presenter.js | 7 |
| frontend/src/components/Search/presenter.js | 6 |
| frontend/src/components/App/presenter.js | 6 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.2830774418871967, "precision": 0.2822902011208782, "recall": 0.2840472145257461, "support": 5575}, "micro avg": {"f1-score": 0.950134529147982, "precision": 0.950134529147982, "recall": 0.950134529147982, "support": 5575}, "weighted avg": {"f1-score": 0.9387786055101767, "precision": 0.9278282585524192, "recall": 0.950134529147982, "support": 5575}, "\u2205": {"f1-score": 0.9719166558819723, "precision": 0.9598670756646217, "recall": 0.9842726081258192, "support": 3815}, "\u23ce": {"f1-score": 0.9174603174603174, "precision": 0.9383116883116883, "recall": 0.8975155279503105, "support": 322}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 33}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 38}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 30}, "\u2423": {"f1-score": 0.9413974455296771, "precision": 0.9247232472324723, "recall": 0.9586840091813313, "support": 1307}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 848}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 320}, "macro avg": {"f1-score": 0.16838022763412594, "precision": 0.2822902011208782, "recall": 0.12266352993759573, "support": 13366}, "micro avg": {"f1-score": 0.559315770022702, "precision": 0.950134529147982, "recall": 0.39630405506509053, "support": 13366}, "weighted avg": {"f1-score": 0.5201574273565125, "precision": 0.7783520320049078, "recall": 0.39630405506509053, "support": 13366}, "\u2205": {"f1-score": 0.7099640763849498, "precision": 0.9598670756646217, "recall": 0.5633063306330633, "support": 6666}, "\u23ce": {"f1-score": 0.43491346877351394, "precision": 0.9383116883116883, "recall": 0.28305582761998044, "support": 1021}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 204}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 262}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 260}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 241}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 249}, "\u2423": {"f1-score": 0.5389247311827957, "precision": 0.9247232472324723, "recall": 0.3802731411229135, "support": 3295}},
  "ppcr": 0.4171030974113422
}
```
</details>
