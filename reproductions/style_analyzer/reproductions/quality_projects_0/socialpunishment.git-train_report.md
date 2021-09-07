# Train report for javascript / file:///tmp/top-repos-quality-repos-fybgglzv/socialpunishment.git HEAD 59992a0ba121c39af887002bcb00082b6d44e813

### Classification report

PPCR: 0.842

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.985| 0.987| 0.977| 0.986| 0.981| 41353| 41788| 0.990 |
| `␣` | 0.940| 0.984| 0.956| 0.962| 0.948| 20081| 20687| 0.971 |
| `'` | 0.967| 1.000| 0.644| 0.983| 0.773| 4712| 7318| 0.644 |
| `⏎` | 0.959| 0.531| 0.144| 0.683| 0.250| 1224| 4514| 0.271 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.992| 0.616| 0.163| 0.760| 0.280| 380| 1438| 0.264 |
| `⏎⏎` | 0.908| 0.671| 0.129| 0.771| 0.226| 249| 1292| 0.193 |
| `⏎␣⁻␣⁻` | 1.000| 0.508| 0.109| 0.674| 0.196| 181| 845| 0.214 |
| `"` | 1.000| 0.070| 0.011| 0.130| 0.021| 172| 1105| 0.156 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 130| 1476| 0.088 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 25| 869| 0.029 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 22| 81| 0.272 |
| `⏎⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 13| 37| 0.351 |
| `macro avg` | 0.646| 0.447| 0.261| 0.496| 0.306| 68542| 81450| 0.842 |
| `micro avg` | 0.970| 0.970| 0.816| 0.970| 0.886| 68542| 81450| 0.842 |
| `weighted avg` | 0.967| 0.970| 0.816| 0.966| 0.838| 68542| 81450| 0.842 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎⏎| ⏎⇥⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|435 |40821 |532 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|606 |289 |19768 |0 |24 |0 |0 |0 |0 |0 |0 |0 |0 |
|2606 |0 |0 |4712 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3290 |140 |417 |0 |650 |17 |0 |0 |0 |0 |0 |0 |0 |
|1043 |33 |43 |0 |4 |167 |0 |0 |0 |0 |2 |0 |0 |
|844 |4 |21 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|664 |43 |46 |0 |0 |0 |0 |92 |0 |0 |0 |0 |0 |
|933 |0 |0 |160 |0 |0 |0 |0 |12 |0 |0 |0 |0 |
|1346 |22 |108 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1058 |78 |68 |0 |0 |0 |0 |0 |0 |0 |234 |0 |0 |
|24 |2 |11 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|59 |0 |22 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| _static_root/bootstrap4/js/bootstrap.bb38938b1b90.js | 338 |
| _static_root/django_extensions/js/jquery.autocomplete.26e55daaf7c5.js | 180 |
| _static_root/django_extensions/js/jquery.autocomplete.js | 180 |
| _static_root/otree/js/jquery.animate-colors.js | 111 |
| _static_root/otree/js/jquery.animate-colors.06edd9739ece.js | 111 |
| _static_root/django_extensions/js/jquery.ajaxQueue.js | 70 |
| _static_root/django_extensions/js/jquery.ajaxQueue.ac504621bdd8.js | 70 |
| _static_root/admin/js/inlines.eda404ee376d.js | 52 |
| _static_root/admin/js/inlines.js | 52 |
| _static_root/admin/js/urlify.js | 45 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.13043478260869565, "precision": 1.0, "recall": 0.06976744186046512, "support": 172}, "\u0027": {"f1-score": 0.98330550918197, "precision": 0.9671592775041051, "recall": 1.0, "support": 4712}, "macro avg": {"f1-score": 0.49583906468555144, "precision": 0.6458308989205402, "recall": 0.44726008093261743, "support": 68542}, "micro avg": {"f1-score": 0.9695661054535905, "precision": 0.9695661054535905, "recall": 0.9695661054535905, "support": 68542}, "weighted avg": {"f1-score": 0.965627389052635, "precision": 0.9672931956040831, "recall": 0.9695661054535905, "support": 68542}, "\u2205": {"f1-score": 0.98619315093314, "precision": 0.9852529445838965, "recall": 0.9871351534350591, "support": 41353}, "\u23ce": {"f1-score": 0.683491062039958, "precision": 0.9587020648967551, "recall": 0.5310457516339869, "support": 1224}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u23ce\u23ce": {"f1-score": 0.771362586605081, "precision": 0.907608695652174, "recall": 0.6706827309236948, "support": 249}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 13}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 25}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 130}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.673992673992674, "precision": 1.0, "recall": 0.5082872928176796, "support": 181}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.7597402597402598, "precision": 0.9915254237288136, "recall": 0.6157894736842106, "support": 380}, "\u2423": {"f1-score": 0.9615487511248388, "precision": 0.9397223806807378, "recall": 0.9844131268363129, "support": 20081}},
  "cl_report_full": {"\"": {"f1-score": 0.021486123545210387, "precision": 1.0, "recall": 0.01085972850678733, "support": 1105}, "\u0027": {"f1-score": 0.773092698933552, "precision": 0.9671592775041051, "recall": 0.6438917737086636, "support": 7318}, "macro avg": {"f1-score": 0.30631780532055025, "precision": 0.6458308989205402, "recall": 0.2610035018894371, "support": 81450}, "micro avg": {"f1-score": 0.8861272601205399, "precision": 0.9695661054535905, "recall": 0.8159116022099447, "support": 81450}, "weighted avg": {"f1-score": 0.8381837131729435, "precision": 0.940030492881988, "recall": 0.8159116022099447, "support": 81450}, "\u2205": {"f1-score": 0.9810382119682768, "precision": 0.9852529445838965, "recall": 0.9768593854695128, "support": 41788}, "\u23ce": {"f1-score": 0.2503852080123267, "precision": 0.9587020648967551, "recall": 0.14399645547186532, "support": 4514}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 81}, "\u23ce\u23ce": {"f1-score": 0.2262872628726287, "precision": 0.907608695652174, "recall": 0.12925696594427244, "support": 1292}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 37}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 869}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1476}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.19637139807897544, "precision": 1.0, "recall": 0.10887573964497041, "support": 845}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.27956989247311825, "precision": 0.9915254237288136, "recall": 0.1627260083449235, "support": 1438}, "\u2423": {"f1-score": 0.9475828679625147, "precision": 0.9397223806807378, "recall": 0.9555759655822498, "support": 20687}},
  "ppcr": 0.8415224063842849
}
```
</details>
