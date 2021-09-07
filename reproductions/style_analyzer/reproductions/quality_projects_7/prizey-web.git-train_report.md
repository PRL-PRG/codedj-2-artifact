# Train report for javascript / file:///tmp/top-repos-quality-repos-7s5cpqyw/prizey-web.git HEAD e8fa61071d69a295397656cd5e52645ba5782562

### Classification report

PPCR: 0.778

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.945| 0.989| 0.897| 0.966| 0.920| 15134| 16683| 0.907 |
| `␣` | 0.953| 0.921| 0.655| 0.937| 0.777| 5817| 8181| 0.711 |
| `'` | 1.000| 1.000| 0.874| 1.000| 0.933| 2330| 2666| 0.874 |
| `⏎␣⁻␣⁻` | 1.000| 0.620| 0.365| 0.766| 0.535| 616| 1047| 0.588 |
| `"` | 1.000| 1.000| 0.989| 1.000| 0.994| 528| 534| 0.989 |
| `⏎␣⁺␣⁺` | 0.956| 0.508| 0.183| 0.663| 0.307| 386| 1071| 0.360 |
| `⏎⏎` | 0.902| 0.969| 0.478| 0.934| 0.625| 257| 521| 0.493 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 111| 1679| 0.066 |
| `micro avg` | 0.954| 0.954| 0.741| 0.954| 0.834| 25179| 32382| 0.778 |
| `weighted avg` | 0.950| 0.954| 0.741| 0.949| 0.801| 25179| 32382| 0.778 |
| `macro avg` | 0.845| 0.751| 0.555| 0.783| 0.636| 25179| 32382| 0.778 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1549 |14966 |167 |0 |0 |1 |0 |0 |0 |
|2364 |445 |5359 |0 |0 |8 |0 |5 |0 |
|336 |0 |0 |2330 |0 |0 |0 |0 |0 |
|1568 |49 |40 |0 |0 |0 |0 |22 |0 |
|685 |143 |47 |0 |0 |196 |0 |0 |0 |
|431 |226 |8 |0 |0 |0 |382 |0 |0 |
|264 |8 |0 |0 |0 |0 |0 |249 |0 |
|6 |0 |0 |0 |0 |0 |0 |0 |528 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/advideo/AdVideoScreen.js | 69 |
| src/payment/CreditCardForm.js | 61 |
| src/shipping/ConfirmOrder.js | 48 |
| src/design/StripeField/StripeField.js | 38 |
| src/game/ClaimProductScreen/SellItBack.js | 37 |
| src/game/ChooseDifficultyScreen.js | 31 |
| src/payment/__tests__/PaymentInfo.test.js | 30 |
| src/payment/PaymentInfo.js | 30 |
| src/game/Roulette.js | 30 |
| src/game/AllPrizesScreen.js | 26 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 528}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 2330}, "macro avg": {"f1-score": 0.7833315015909519, "precision": 0.8445828450887949, "recall": 0.7508672388286106, "support": 25179}, "micro avg": {"f1-score": 0.9535724214623297, "precision": 0.9535724214623297, "recall": 0.9535724214623297, "support": 25179}, "weighted avg": {"f1-score": 0.9493165365703304, "precision": 0.9500949227362959, "recall": 0.9535724214623297, "support": 25179}, "\u2205": {"f1-score": 0.9664524878111782, "precision": 0.945002210014523, "recall": 0.9888991674375578, "support": 15134}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 111}, "\u23ce\u23ce": {"f1-score": 0.9343339587242027, "precision": 0.9021739130434783, "recall": 0.9688715953307393, "support": 257}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.6632825719120136, "precision": 0.9560975609756097, "recall": 0.5077720207253886, "support": 386}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7655310621242485, "precision": 1.0, "recall": 0.6201298701298701, "support": 616}, "\u2423": {"f1-score": 0.9370519321559714, "precision": 0.9533890766767479, "recall": 0.9212652570053292, "support": 5817}},
  "cl_report_full": {"\"": {"f1-score": 0.9943502824858756, "precision": 1.0, "recall": 0.9887640449438202, "support": 534}, "\u0027": {"f1-score": 0.932746196957566, "precision": 1.0, "recall": 0.8739684921230307, "support": 2666}, "macro avg": {"f1-score": 0.6363452009423629, "precision": 0.8445828450887949, "recall": 0.5550816686767803, "support": 32382}, "micro avg": {"f1-score": 0.8342454092180469, "precision": 0.9535724214623297, "recall": 0.7414613056636403, "support": 32382}, "weighted avg": {"f1-score": 0.801072926002778, "precision": 0.9050139275667127, "recall": 0.7414613056636403, "support": 32382}, "\u2205": {"f1-score": 0.9204182041820418, "precision": 0.945002210014523, "recall": 0.8970808607564587, "support": 16683}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1679}, "\u23ce\u23ce": {"f1-score": 0.6248431618569636, "precision": 0.9021739130434783, "recall": 0.4779270633397313, "support": 521}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.3072100313479623, "precision": 0.9560975609756097, "recall": 0.1830065359477124, "support": 1071}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.5346396081175647, "precision": 1.0, "recall": 0.3648519579751671, "support": 1047}, "\u2423": {"f1-score": 0.7765541225909288, "precision": 0.9533890766767479, "recall": 0.6550543943283217, "support": 8181}},
  "ppcr": 0.7775616083009079
}
```
</details>
