# Train report for javascript / file:///tmp/top-repos-quality-repos-t35jwtlp/sauti-studio-fe-v2.git HEAD 9ab26b97a4990879526f2949933e5866f3033d2f

### Classification report

PPCR: 0.230

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.997| 1.000| 0.298| 0.999| 0.459| 1589| 5334| 0.298 |
| `⏎` | 0.945| 1.000| 0.457| 0.972| 0.616| 395| 865| 0.457 |
| `"` | 1.000| 1.000| 0.254| 1.000| 0.405| 133| 524| 0.254 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 22| 154| 0.143 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 1793| 0.002 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 245| 0.004 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 263| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 155| 0.000 |
| `micro avg` | 0.987| 0.987| 0.227| 0.987| 0.369| 2144| 9333| 0.230 |
| `macro avg` | 0.368| 0.375| 0.126| 0.371| 0.185| 2144| 9333| 0.230 |
| `weighted avg` | 0.975| 0.987| 0.227| 0.981| 0.342| 2144| 9333| 0.230 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| '| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3745 |1589 |0 |0 |0 |0 |0 |0 |0 |
|1789 |4 |0 |0 |0 |0 |0 |0 |0 |
|470 |0 |0 |395 |0 |0 |0 |0 |0 |
|391 |0 |0 |0 |133 |0 |0 |0 |0 |
|263 |0 |0 |0 |0 |0 |0 |0 |0 |
|244 |0 |0 |1 |0 |0 |0 |0 |0 |
|155 |0 |0 |0 |0 |0 |0 |0 |0 |
|132 |0 |0 |22 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/actions/index.js | 8 |
| src/components/Canvas/main.js | 5 |
| src/components/ContactForm.js | 3 |
| src/App.test.js | 2 |
| src/components/Project.js | 2 |
| src/pages/Profile.js | 2 |
| src/components/Folder.js | 2 |
| src/pages/Login.js | 1 |
| src/index.js | 1 |
| src/components/Modal.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 133}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.3713065807590823, "precision": 0.3678081363741489, "recall": 0.375, "support": 2144}, "micro avg": {"f1-score": 0.9874067164179104, "precision": 0.9874067164179104, "recall": 0.9874067164179104, "support": 2144}, "weighted avg": {"f1-score": 0.981262990856078, "precision": 0.9754083928085269, "recall": 0.9874067164179104, "support": 2144}, "\u2205": {"f1-score": 0.9987429289754871, "precision": 0.997489014438167, "recall": 1.0, "support": 1589}, "\u23ce": {"f1-score": 0.971709717097171, "precision": 0.9449760765550239, "recall": 1.0, "support": 395}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}},
  "cl_report_full": {"\"": {"f1-score": 0.4048706240487063, "precision": 1.0, "recall": 0.2538167938931298, "support": 524}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 155}, "macro avg": {"f1-score": 0.18492492997629512, "precision": 0.3678081363741489, "recall": 0.12604555690053149, "support": 9333}, "micro avg": {"f1-score": 0.3689117365165113, "precision": 0.9874067164179104, "recall": 0.22682952962605807, "support": 9333}, "weighted avg": {"f1-score": 0.3420044373493617, "precision": 0.7138123550019585, "recall": 0.22682952962605807, "support": 9333}, "\u2205": {"f1-score": 0.45878446658004907, "precision": 0.997489014438167, "recall": 0.2979002624671916, "support": 5334}, "\u23ce": {"f1-score": 0.6157443491816056, "precision": 0.9449760765550239, "recall": 0.45664739884393063, "support": 865}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 154}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 263}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 245}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1793}},
  "ppcr": 0.22972249008893175
}
```
</details>
