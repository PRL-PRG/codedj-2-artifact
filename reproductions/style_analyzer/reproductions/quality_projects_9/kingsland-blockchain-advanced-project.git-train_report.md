# Train report for javascript / file:///tmp/top-repos-quality-repos-54h1u8ih/kingsland-blockchain-advanced-project.git HEAD 30c7ea23a73509b0abee2d47920032f112a205c3

### Classification report

PPCR: 0.865

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.996| 0.988| 0.958| 0.992| 0.977| 42277| 43630| 0.969 |
| `␣` | 0.974| 0.993| 0.921| 0.983| 0.947| 20533| 22127| 0.928 |
| `⏎␣⁺␣⁺` | 0.898| 0.982| 0.965| 0.938| 0.930| 2271| 2310| 0.983 |
| `⏎␣⁻␣⁻` | 0.932| 0.978| 0.913| 0.954| 0.922| 2097| 2247| 0.933 |
| `'` | 0.996| 1.000| 0.381| 0.998| 0.551| 1517| 3980| 0.381 |
| `⏎` | 0.951| 0.942| 0.324| 0.946| 0.483| 1293| 3760| 0.344 |
| `⏎⏎` | 0.988| 0.908| 0.136| 0.946| 0.239| 357| 2388| 0.149 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 206| 215| 0.958 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 142| 163| 0.871 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 794| 0.008 |
| `⏎⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 99| 0.010 |
| `weighted avg` | 0.979| 0.983| 0.851| 0.981| 0.886| 70700| 81713| 0.865 |
| `macro avg` | 0.612| 0.617| 0.418| 0.614| 0.459| 70700| 81713| 0.865 |
| `micro avg` | 0.983| 0.983| 0.851| 0.983| 0.912| 70700| 81713| 0.865 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1353 |41788 |417 |0 |1 |0 |58 |13 |0 |0 |0 |0 |
|1594 |67 |20390 |0 |29 |0 |43 |4 |0 |0 |0 |0 |
|2463 |0 |0 |1517 |0 |0 |0 |0 |0 |0 |0 |0 |
|2467 |25 |44 |0 |1218 |3 |1 |2 |0 |0 |0 |0 |
|2031 |1 |0 |0 |31 |324 |1 |0 |0 |0 |0 |0 |
|39 |11 |30 |0 |1 |0 |2229 |0 |0 |0 |0 |0 |
|150 |35 |10 |0 |1 |0 |0 |2051 |0 |0 |0 |0 |
|788 |0 |0 |6 |0 |0 |0 |0 |0 |0 |0 |0 |
|9 |11 |45 |0 |0 |0 |150 |0 |0 |0 |0 |0 |
|21 |10 |1 |0 |0 |0 |0 |131 |0 |0 |0 |0 |
|98 |0 |0 |0 |0 |1 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| Faucet/public/static/bootstrap4/js/bootstrap.bundle.js | 189 |
| Wallet/static/bootstrap4/js/bootstrap.bundle.js | 188 |
| Wallet/static/js/wallet.js | 161 |
| Faucet/public/static/js/wallet.js | 139 |
| Wallet/libs/walletApp.js | 100 |
| Faucet/libs/faucet-transaction.js | 48 |
| block-explorer/src/containers/BlockPage.js | 34 |
| block-explorer/src/containers/AddressPage.js | 28 |
| Faucet/routes/index.js | 22 |
| block-explorer/src/containers/TransactionPage.js | 21 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u0027": {"f1-score": 0.9980263157894737, "precision": 0.9960604070912672, "recall": 1.0, "support": 1517}, "macro avg": {"f1-score": 0.6143987190678056, "precision": 0.6122418135673818, "recall": 0.6173270235048761, "support": 70700}, "micro avg": {"f1-score": 0.9832673267326733, "precision": 0.9832673267326733, "recall": 0.9832673267326733, "support": 70700}, "weighted avg": {"f1-score": 0.980896941400693, "precision": 0.9787691029182415, "recall": 0.9832673267326733, "support": 70700}, "\u2205": {"f1-score": 0.9922944493915108, "precision": 0.9961857537904072, "recall": 0.9884334271589753, "support": 42277}, "\u23ce": {"f1-score": 0.9463869463869464, "precision": 0.9508196721311475, "recall": 0.9419953596287703, "support": 1293}, "\u23ce\u23ce": {"f1-score": 0.945985401459854, "precision": 0.9878048780487805, "recall": 0.907563025210084, "support": 357}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9379339364611824, "precision": 0.8980660757453667, "recall": 0.9815059445178336, "support": 2271}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 206}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9543973941368078, "precision": 0.9318491594729669, "recall": 0.978063900810682, "support": 2097}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 142}, "\u2423": {"f1-score": 0.9833614661200868, "precision": 0.9738740029612647, "recall": 0.9930356012272926, "support": 20533}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 794}, "\u0027": {"f1-score": 0.5513356351081229, "precision": 0.9960604070912672, "recall": 0.3811557788944724, "support": 3980}, "macro avg": {"f1-score": 0.45902188840680763, "precision": 0.6122418135673818, "recall": 0.4179779960343591, "support": 81713}, "micro avg": {"f1-score": 0.9122187739890955, "precision": 0.9832673267326733, "recall": 0.8507459033446331, "support": 81713}, "weighted avg": {"f1-score": 0.885599671701321, "precision": 0.9677673397614062, "recall": 0.8507459033446331, "support": 81713}, "\u2205": {"f1-score": 0.9766061370913086, "precision": 0.9961857537904072, "recall": 0.9577813431125373, "support": 43630}, "\u23ce": {"f1-score": 0.4832374528863321, "precision": 0.9508196721311475, "recall": 0.32393617021276594, "support": 3760}, "\u23ce\u23ce": {"f1-score": 0.23858615611192932, "precision": 0.9878048780487805, "recall": 0.135678391959799, "support": 2388}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 99}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9303005008347246, "precision": 0.8980660757453667, "recall": 0.964935064935065, "support": 2310}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 215}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9222122302158272, "precision": 0.9318491594729669, "recall": 0.9127725856697819, "support": 2247}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 163}, "\u2423": {"f1-score": 0.9469626602266394, "precision": 0.9738740029612647, "recall": 0.9214986215935282, "support": 22127}},
  "ppcr": 0.8652234038647462
}
```
</details>
