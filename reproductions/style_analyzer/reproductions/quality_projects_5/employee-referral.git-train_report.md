# Train report for javascript / file:///tmp/top-repos-quality-repos-k_dgv63y/employee-referral.git HEAD e22271e89f4ec92df5d7cdfbcfa4d52637b1eb2f

### Classification report

PPCR: 0.898

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.990| 0.998| 0.962| 0.994| 0.976| 68351| 70891| 0.964 |
| `"` | 0.999| 1.000| 1.000| 1.000| 1.000| 14630| 14636| 1.000 |
| `␣` | 0.984| 0.948| 0.727| 0.966| 0.836| 12418| 16183| 0.767 |
| `'` | 1.000| 0.994| 0.334| 0.997| 0.501| 1425| 4242| 0.336 |
| `⏎␣⁻␣⁻` | 0.947| 0.934| 0.778| 0.940| 0.854| 514| 617| 0.833 |
| `⏎` | 0.977| 0.740| 0.151| 0.842| 0.262| 292| 1427| 0.205 |
| `⏎␣⁺␣⁺` | 0.888| 0.878| 0.400| 0.883| 0.551| 288| 633| 0.455 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 356| 0.014 |
| `micro avg` | 0.990| 0.990| 0.890| 0.990| 0.937| 97923| 108985| 0.898 |
| `macro avg` | 0.848| 0.812| 0.544| 0.828| 0.623| 97923| 108985| 0.898 |
| `weighted avg` | 0.990| 0.990| 0.890| 0.990| 0.924| 97923| 108985| 0.898 |

### Confusion matrix

|refusal|  ∅| ␣| "| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2540 |68218 |130 |0 |0 |0 |0 |3 |0 |
|3765 |589 |11773 |0 |0 |0 |32 |24 |0 |
|6 |0 |0 |14630 |0 |0 |0 |0 |0 |
|2817 |0 |0 |8 |1417 |0 |0 |0 |0 |
|1135 |40 |36 |0 |0 |216 |0 |0 |0 |
|345 |7 |28 |0 |0 |0 |253 |0 |0 |
|103 |34 |0 |0 |0 |0 |0 |480 |0 |
|351 |0 |0 |0 |0 |5 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/views/Base/Switches/Switches.js | 165 |
| src/views/Dashboard/Dashboard.js | 104 |
| src/views/Buttons/ButtonDropdowns/ButtonDropdowns.js | 74 |
| src/views/Base/Tabs/Tabs.js | 64 |
| src/views/Base/Dropdowns/Dropdowns.js | 43 |
| src/views/Base/Forms/Forms.js | 40 |
| src/views/index.js | 37 |
| src/views/Base/Collapses/Collapses.js | 36 |
| src/serviceWorker.js | 33 |
| src/views/Widgets/Widgets.js | 29 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9997266639333059, "precision": 0.9994534772509905, "recall": 1.0, "support": 14630}, "\u0027": {"f1-score": 0.9971850809289233, "precision": 1.0, "recall": 0.9943859649122807, "support": 1425}, "macro avg": {"f1-score": 0.8277607137138836, "precision": 0.8481695904615716, "recall": 0.8115687231274916, "support": 97923}, "micro avg": {"f1-score": 0.9904414693177293, "precision": 0.9904414693177293, "recall": 0.9904414693177293, "support": 97923}, "weighted avg": {"f1-score": 0.9902914784943311, "precision": 0.9903454751892745, "recall": 0.9904414693177293, "support": 97923}, "\u2205": {"f1-score": 0.9941488935360938, "precision": 0.990274068052491, "recall": 0.9980541616069991, "support": 68351}, "\u23ce": {"f1-score": 0.8421052631578947, "precision": 0.9773755656108597, "recall": 0.7397260273972602, "support": 292}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8830715532286213, "precision": 0.887719298245614, "recall": 0.8784722222222222, "support": 288}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.940254652301665, "precision": 0.9467455621301775, "recall": 0.933852140077821, "support": 514}, "\u2423": {"f1-score": 0.9655936026245643, "precision": 0.98378875240244, "recall": 0.94805926880335, "support": 12418}},
  "cl_report_full": {"\"": {"f1-score": 0.9995217599234816, "precision": 0.9994534772509905, "recall": 0.999590051926756, "support": 14636}, "\u0027": {"f1-score": 0.5007951934970843, "precision": 1.0, "recall": 0.33404054691183405, "support": 4242}, "macro avg": {"f1-score": 0.6225343670746403, "precision": 0.8481695904615716, "recall": 0.5440531306981206, "support": 108985}, "micro avg": {"f1-score": 0.9374891256017167, "precision": 0.9904414693177293, "recall": 0.8899114557049135, "support": 108985}, "weighted avg": {"f1-score": 0.9243024689789353, "precision": 0.9866766682684206, "recall": 0.8899114557049135, "support": 108985}, "\u2205": {"f1-score": 0.9760836749440188, "precision": 0.990274068052491, "recall": 0.9622942263474912, "support": 70891}, "\u23ce": {"f1-score": 0.2621359223300971, "precision": 0.9773755656108597, "recall": 0.1513665031534688, "support": 1427}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 356}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.55119825708061, "precision": 0.887719298245614, "recall": 0.39968404423380727, "support": 633}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8540925266903916, "precision": 0.9467455621301775, "recall": 0.7779578606158833, "support": 617}, "\u2423": {"f1-score": 0.8364476021314388, "precision": 0.98378875240244, "recall": 0.7274918123957239, "support": 16183}},
  "ppcr": 0.898499793549571
}
```
</details>
