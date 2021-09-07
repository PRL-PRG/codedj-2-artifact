# Train report for javascript / file:///tmp/top-repos-quality-repos-u039w1z_/daily-algorithms.git HEAD c8a92a2708924cbb7733fd3c5bbc2745192ad7fe

### Classification report

PPCR: 0.801

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.966| 0.984| 0.938| 0.975| 0.952| 15434| 16183| 0.954 |
| `␣` | 0.961| 0.944| 0.592| 0.952| 0.732| 7205| 11492| 0.627 |
| `⏎` | 0.953| 0.908| 0.476| 0.930| 0.635| 1238| 2362| 0.524 |
| `⏎␣⁺␣⁺` | 0.990| 0.970| 0.967| 0.980| 0.978| 1079| 1083| 0.996 |
| `⏎␣⁻␣⁻` | 0.975| 0.988| 0.975| 0.981| 0.975| 1047| 1060| 0.988 |
| `'` | 1.000| 1.000| 0.722| 1.000| 0.839| 187| 259| 0.722 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 86| 378| 0.228 |
| `micro avg` | 0.966| 0.966| 0.773| 0.966| 0.859| 26276| 32817| 0.801 |
| `weighted avg` | 0.963| 0.966| 0.773| 0.964| 0.842| 26276| 32817| 0.801 |
| `macro avg` | 0.835| 0.828| 0.667| 0.831| 0.730| 26276| 32817| 0.801 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| '| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|749 |15186 |217 |13 |1 |17 |0 |0 |
|4287 |385 |6798 |3 |10 |9 |0 |0 |
|1124 |72 |42 |1124 |0 |0 |0 |0 |
|4 |20 |12 |0 |1047 |0 |0 |0 |
|13 |7 |6 |0 |0 |1034 |0 |0 |
|292 |47 |0 |39 |0 |0 |0 |0 |
|72 |0 |0 |0 |0 |0 |0 |187 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| leetcode/429.n叉树的层序遍历.js | 34 |
| leetcode/804.唯一摩尔斯密码词.js | 28 |
| codewars/Strings Mix.js | 26 |
| codewars/Merged String Checker.js | 22 |
| leetcode/17.电话号码的字母组合.js | 18 |
| jianzhioffer/顺时针打印矩阵.js | 14 |
| codewars/Range Extraction.js | 14 |
| codewars/Memoized Fibonacci.js | 14 |
| codewars/ormat a string of names like 'Bart, Lisa & Maggie'.js | 14 |
| leetcode/1041.困于环中的机器人.js | 13 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 187}, "macro avg": {"f1-score": 0.831220118848717, "precision": 0.8350697146570599, "recall": 0.8276122151088245, "support": 26276}, "micro avg": {"f1-score": 0.9657482112954787, "precision": 0.9657482112954787, "recall": 0.9657482112954787, "support": 26276}, "weighted avg": {"f1-score": 0.9640459518724278, "precision": 0.9625445129434529, "recall": 0.9657482112954787, "support": 26276}, "\u2205": {"f1-score": 0.9749927771179094, "precision": 0.9662149265126933, "recall": 0.9839315796293897, "support": 15434}, "\u23ce": {"f1-score": 0.9300786098469177, "precision": 0.9533502968617472, "recall": 0.9079159935379645, "support": 1238}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 86}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9798783341132429, "precision": 0.9896030245746692, "recall": 0.9703429101019463, "support": 1079}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9814902705268154, "precision": 0.9754716981132076, "recall": 0.9875835721107927, "support": 1047}, "\u2423": {"f1-score": 0.9521008403361344, "precision": 0.9608480565371025, "recall": 0.9435114503816794, "support": 7205}},
  "cl_report_full": {"\u0027": {"f1-score": 0.8385650224215246, "precision": 1.0, "recall": 0.722007722007722, "support": 259}, "macro avg": {"f1-score": 0.7301857876042632, "precision": 0.8350697146570599, "recall": 0.6671486305090939, "support": 32817}, "micro avg": {"f1-score": 0.8588496099368792, "precision": 0.9657482112954787, "recall": 0.7732577627449188, "support": 32817}, "weighted avg": {"f1-score": 0.8420324205603644, "precision": 0.9536178047439166, "recall": 0.7732577627449188, "support": 32817}, "\u2205": {"f1-score": 0.9521003134796239, "precision": 0.9662149265126933, "recall": 0.938392139899895, "support": 16183}, "\u23ce": {"f1-score": 0.6348489127365151, "precision": 0.9533502968617472, "recall": 0.4758679085520745, "support": 2362}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 378}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9780476412891173, "precision": 0.9896030245746692, "recall": 0.9667590027700831, "support": 1083}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9754716981132076, "precision": 0.9754716981132076, "recall": 0.9754716981132076, "support": 1060}, "\u2423": {"f1-score": 0.732266925189853, "precision": 0.9608480565371025, "recall": 0.5915419422206752, "support": 11492}},
  "ppcr": 0.8006825730566475
}
```
</details>
