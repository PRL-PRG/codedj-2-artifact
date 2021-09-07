# Train report for javascript / file:///tmp/top-repos-quality-repos-5jqtck35/react.git HEAD 8723e772b98d1a61aa0170981483c4da1e9237c5

### Classification report

PPCR: 0.906

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.986| 0.996| 0.974| 0.991| 0.980| 353786| 362057| 0.977 |
| `␣` | 0.963| 0.989| 0.927| 0.976| 0.945| 113846| 121506| 0.937 |
| `'` | 1.000| 1.000| 0.946| 1.000| 0.972| 49423| 52257| 0.946 |
| `⏎␣⁻␣⁻` | 0.969| 0.954| 0.864| 0.962| 0.914| 21687| 23945| 0.906 |
| `⏎␣⁺␣⁺` | 0.993| 0.776| 0.611| 0.871| 0.756| 19666| 24971| 0.788 |
| `⏎` | 0.944| 0.870| 0.375| 0.906| 0.537| 19384| 44936| 0.431 |
| `⏎⏎` | 0.908| 0.885| 0.287| 0.896| 0.436| 4137| 12756| 0.324 |
| `"` | 1.000| 1.000| 0.982| 1.000| 0.991| 3856| 3926| 0.982 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 524| 711| 0.737 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 66| 141| 0.468 |
| `⏎⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 61| 0.131 |
| `weighted avg` | 0.979| 0.980| 0.888| 0.979| 0.919| 586383| 647267| 0.906 |
| `macro avg` | 0.706| 0.679| 0.542| 0.691| 0.594| 586383| 647267| 0.906 |
| `micro avg` | 0.980| 0.980| 0.888| 0.980| 0.932| 586383| 647267| 0.906 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|8271 |352471 |941 |3 |0 |97 |160 |114 |0 |0 |0 |0 |
|7660 |1015 |112613 |152 |0 |0 |20 |46 |0 |0 |0 |0 |
|25552 |641 |1667 |16864 |0 |0 |6 |206 |0 |0 |0 |0 |
|2834 |0 |0 |0 |49423 |0 |0 |0 |0 |0 |0 |0 |
|5305 |2823 |1266 |323 |0 |15251 |3 |0 |0 |0 |0 |0 |
|2258 |570 |359 |69 |0 |0 |20688 |1 |0 |0 |0 |0 |
|8619 |3 |30 |441 |0 |0 |0 |3663 |0 |0 |0 |0 |
|70 |0 |0 |0 |0 |0 |0 |0 |3856 |0 |0 |0 |
|187 |48 |6 |3 |0 |0 |467 |0 |0 |0 |0 |0 |
|53 |0 |0 |2 |0 |0 |0 |6 |0 |0 |0 |0 |
|75 |14 |36 |1 |0 |15 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| packages/eslint-plugin-react-hooks/__tests__/ESLintRuleExhaustiveDeps-test.js | 646 |
| packages/react/src/__tests__/ReactProfiler-test.internal.js | 489 |
| packages/eslint-plugin-react-hooks/src/ExhaustiveDeps.js | 285 |
| packages/react-reconciler/src/__tests__/useMutableSource-test.internal.js | 258 |
| scripts/rollup/build.js | 239 |
| packages/react-reconciler/src/__tests__/ReactSuspenseList-test.internal.js | 205 |
| packages/react-reconciler/src/__tests__/ReactNewContext-test.internal.js | 203 |
| packages/react-dom/src/events/__tests__/DOMModernPluginEventSystem-test.internal.js | 184 |
| packages/react-reconciler/src/__tests__/ReactIncremental-test.internal.js | 180 |
| scripts/rollup/forks.js | 172 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 3856}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 49423}, "macro avg": {"f1-score": 0.6910252550555871, "precision": 0.7057066716274923, "recall": 0.6791190126291977, "support": 586383}, "micro avg": {"f1-score": 0.9802961545610974, "precision": 0.9802961545610974, "recall": 0.9802961545610974, "support": 586383}, "weighted avg": {"f1-score": 0.9792612587099349, "precision": 0.9793296517431223, "recall": 0.9802961545610974, "support": 586383}, "\u2205": {"f1-score": 0.9909625216659099, "precision": 0.9856985052504998, "recall": 0.9962830637730153, "support": 353786}, "\u23ce": {"f1-score": 0.9056441651898394, "precision": 0.9443386717437563, "recall": 0.8699958728848535, "support": 19384}, "\u23ce\u23ce": {"f1-score": 0.8963660834454912, "precision": 0.9075817641228939, "recall": 0.8854242204496011, "support": 4137}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8707642239287448, "precision": 0.9927097572088784, "recall": 0.7755008644360826, "support": 19666}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 66}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9615393553484697, "precision": 0.9692653673163418, "recall": 0.9539355374187302, "support": 21687}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 524}, "\u2423": {"f1-score": 0.9760014560330034, "precision": 0.9631793222600455, "recall": 0.9891695799588919, "support": 113846}},
  "cl_report_full": {"\"": {"f1-score": 0.9910048830634798, "precision": 1.0, "recall": 0.9821701477330617, "support": 3926}, "\u0027": {"f1-score": 0.9721282454760032, "precision": 1.0, "recall": 0.9457680310771762, "support": 52257}, "macro avg": {"f1-score": 0.5936896785528352, "precision": 0.7057066716274923, "recall": 0.5423135081891964, "support": 647267}, "micro avg": {"f1-score": 0.9319158594414948, "precision": 0.9802961545610974, "recall": 0.8880863693035486, "support": 647267}, "weighted avg": {"f1-score": 0.9186222784807831, "precision": 0.9765738722201056, "recall": 0.8880863693035486, "support": 647267}, "\u2205": {"f1-score": 0.9795731766628406, "precision": 0.9856985052504998, "recall": 0.9735235059672924, "support": 362057}, "\u23ce": {"f1-score": 0.5371213810236646, "precision": 0.9443386717437563, "recall": 0.37528930033825886, "support": 44936}, "\u23ce\u23ce": {"f1-score": 0.4362791805621725, "precision": 0.9075817641228939, "recall": 0.2871589840075259, "support": 12756}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 61}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7562354341250559, "precision": 0.9927097572088784, "recall": 0.6107484682231389, "support": 24971}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 141}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9135993287553269, "precision": 0.9692653673163418, "recall": 0.8639799540613907, "support": 23945}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 711}, "\u2423": {"f1-score": 0.944644834412643, "precision": 0.9631793222600455, "recall": 0.9268101986733166, "support": 121506}},
  "ppcr": 0.9059368081487238
}
```
</details>
