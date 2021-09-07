# Train report for javascript / file:///tmp/top-repos-quality-repos-z3mne4fm/3dhop-evt2.git HEAD 2a6ad41666f0fa82b0714931bcfa09ff52ba7242

### Classification report

PPCR: 0.883

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.993| 0.983| 0.973| 0.988| 0.983| 59501| 60107| 0.990 |
| `␣` | 0.944| 0.992| 0.973| 0.967| 0.958| 31319| 31925| 0.981 |
| `'` | 1.000| 1.000| 1.000| 1.000| 1.000| 9532| 9532| 1.000 |
| `⏎` | 0.914| 0.834| 0.387| 0.872| 0.543| 3002| 6476| 0.464 |
| `⏎⇥⁻` | 0.997| 0.892| 0.247| 0.941| 0.395| 711| 2572| 0.276 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.995| 0.913| 0.215| 0.952| 0.354| 437| 1856| 0.235 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 258| 1788| 0.144 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 174| 2644| 0.066 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 165| 1943| 0.085 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 19| 114| 0.167 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 19| 111| 0.171 |
| `macro avg` | 0.531| 0.510| 0.345| 0.520| 0.385| 105137| 119068| 0.883 |
| `micro avg` | 0.976| 0.976| 0.862| 0.976| 0.916| 105137| 119068| 0.883 |
| `weighted avg` | 0.971| 0.976| 0.862| 0.973| 0.877| 105137| 119068| 0.883 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⇥⁺| ⏎⇥⁻| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|606 |58497 |945 |0 |59 |0 |0 |0 |0 |0 |0 |0 |
|606 |222 |31070 |0 |25 |0 |0 |0 |2 |0 |0 |0 |
|0 |0 |0 |9532 |0 |0 |0 |0 |0 |0 |0 |0 |
|3474 |12 |485 |0 |2503 |0 |2 |0 |0 |0 |0 |0 |
|2470 |1 |156 |0 |17 |0 |0 |0 |0 |0 |0 |0 |
|1861 |0 |69 |0 |8 |0 |634 |0 |0 |0 |0 |0 |
|1778 |18 |132 |0 |15 |0 |0 |0 |0 |0 |0 |0 |
|1419 |18 |20 |0 |0 |0 |0 |0 |399 |0 |0 |0 |
|1530 |139 |7 |0 |112 |0 |0 |0 |0 |0 |0 |0 |
|95 |2 |16 |0 |1 |0 |0 |0 |0 |0 |0 |0 |
|92 |0 |19 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| app/src/dataHandler/parser.service.js | 144 |
| app/src/interface/interface.service.js | 130 |
| app/src/buttonSwitch/buttonSwitch.provider.js | 118 |
| app/src/dataHandler/namedEntitiesParser.service.js | 105 |
| app/src/dataHandler/criticalApparatusParser.service.js | 103 |
| Gruntfile.js | 98 |
| app/src/popover/popover.directive.js | 93 |
| app/src/box/box.directive.js | 89 |
| app/src/dataHandler/parsedData.service.js | 86 |
| app/src/reading/reading.controller.js | 80 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 9532}, "macro avg": {"f1-score": 0.5200747157177749, "precision": 0.5311099505512653, "recall": 0.5103362433586466, "support": 105137}, "micro avg": {"f1-score": 0.9762024786706868, "precision": 0.9762024786706868, "recall": 0.9762024786706868, "support": 105137}, "weighted avg": {"f1-score": 0.9732086539827033, "precision": 0.9707586413586959, "recall": 0.9762024786706868, "support": 105137}, "\u2205": {"f1-score": 0.9880415505447174, "precision": 0.9930061620465463, "recall": 0.9831263340111931, "support": 59501}, "\u23ce": {"f1-score": 0.8718216649251134, "precision": 0.9135036496350365, "recall": 0.8337774816788808, "support": 3002}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 174}, "\u23ce\u21e5\u207b": {"f1-score": 0.9413511507052709, "precision": 0.9968553459119497, "recall": 0.8917018284106891, "support": 711}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 258}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 19}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 165}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 19}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9522673031026253, "precision": 0.9950124688279302, "recall": 0.9130434782608695, "support": 437}, "\u2423": {"f1-score": 0.9673402036177963, "precision": 0.9438318296424557, "recall": 0.9920495545834797, "support": 31319}},
  "cl_report_full": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 9532}, "macro avg": {"f1-score": 0.3848477180142847, "precision": 0.5311099505512653, "recall": 0.34494692325255616, "support": 119068}, "micro avg": {"f1-score": 0.9155460404540487, "precision": 0.9762024786706868, "recall": 0.8619864279235395, "support": 119068}, "weighted avg": {"f1-score": 0.8768271288320469, "precision": 0.9211287438298605, "recall": 0.8619864279235395, "support": 119068}, "\u2205": {"f1-score": 0.9830106876386367, "precision": 0.9930061620465463, "recall": 0.973214434258905, "support": 60107}, "\u23ce": {"f1-score": 0.5431857638888888, "precision": 0.9135036496350365, "recall": 0.3865040148239654, "support": 6476}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2644}, "\u23ce\u21e5\u207b": {"f1-score": 0.3952618453865337, "precision": 0.9968553459119497, "recall": 0.24650077760497668, "support": 2572}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1788}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 111}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1943}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 114}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.35356668143553394, "precision": 0.9950124688279302, "recall": 0.21497844827586207, "support": 1856}, "\u2423": {"f1-score": 0.9582999198075381, "precision": 0.9438318296424557, "recall": 0.9732184808144088, "support": 31925}},
  "ppcr": 0.8829996304632647
}
```
</details>
