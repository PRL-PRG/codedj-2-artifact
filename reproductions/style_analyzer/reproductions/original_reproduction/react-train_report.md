# Train report for javascript / file:///tmp/top-repos-quality-repos-dz80hf36/react HEAD 1034e26fe5e42ba07492a736da7bdf5bf2108bc6

### Classification report

PPCR: 0.952

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.981| 0.994| 0.981| 0.987| 0.981| 208908| 211735| 0.987 |
| `␣` | 0.972| 0.977| 0.920| 0.975| 0.946| 71692| 76103| 0.942 |
| `'` | 1.000| 1.000| 0.978| 1.000| 0.989| 34623| 35392| 0.978 |
| `⏎` | 0.893| 0.958| 0.807| 0.924| 0.848| 23432| 27799| 0.843 |
| `⏎␣⁻␣⁻` | 0.969| 0.918| 0.892| 0.943| 0.929| 14532| 14956| 0.972 |
| `⏎␣⁺␣⁺` | 0.974| 0.817| 0.700| 0.888| 0.814| 13233| 15443| 0.857 |
| `⏎⏎` | 0.916| 0.505| 0.252| 0.651| 0.395| 3484| 7000| 0.498 |
| `"` | 1.000| 1.000| 1.000| 1.000| 1.000| 2634| 2634| 1.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 316| 398| 0.794 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 75| 90| 0.833 |
| `weighted avg` | 0.973| 0.974| 0.928| 0.973| 0.945| 372929| 391550| 0.952 |
| `macro avg` | 0.770| 0.717| 0.653| 0.737| 0.690| 372929| 391550| 0.952 |
| `micro avg` | 0.974| 0.974| 0.928| 0.974| 0.950| 372929| 391550| 0.952 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2827 |207632 |756 |0 |24 |242 |253 |1 |0 |0 |0 |
|4411 |840 |70052 |0 |778 |16 |4 |2 |0 |0 |0 |
|769 |0 |0 |34623 |0 |0 |0 |0 |0 |0 |0 |
|4367 |303 |517 |0 |22440 |14 |0 |158 |0 |0 |0 |
|2210 |1920 |477 |0 |30 |10806 |0 |0 |0 |0 |0 |
|424 |783 |199 |0 |209 |0 |13341 |0 |0 |0 |0 |
|3516 |69 |20 |0 |1634 |0 |0 |1761 |0 |0 |0 |
|0 |0 |0 |0 |0 |0 |0 |0 |2634 |0 |0 |
|82 |116 |6 |0 |18 |0 |176 |0 |0 |0 |0 |
|15 |6 |49 |0 |0 |20 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| packages/react/src/__tests__/ReactProfiler-test.internal.js | 410 |
| scripts/bench/benchmarks/pe-functional-components/benchmark.js | 325 |
| scripts/bench/benchmarks/pe-class-components/benchmark.js | 320 |
| fixtures/attribute-behavior/src/attributes.js | 301 |
| packages/react-reconciler/src/__tests__/ReactIncremental-test.internal.js | 246 |
| packages/react-reconciler/src/__tests__/ReactNewContext-test.internal.js | 241 |
| packages/react-reconciler/src/__tests__/ReactSuspenseWithNoopRenderer-test.internal.js | 235 |
| scripts/bench/benchmarks/pe-no-components/benchmark.js | 228 |
| packages/events/__tests__/ResponderEventPlugin-test.internal.js | 222 |
| packages/react-reconciler/src/__tests__/ReactIncrementalSideEffects-test.internal.js | 164 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 2634}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 34623}, "macro avg": {"f1-score": 0.7368379058965435, "precision": 0.770418234212048, "recall": 0.7168772463392029, "support": 372929}, "micro avg": {"f1-score": 0.9741505755787294, "precision": 0.9741505755787294, "recall": 0.9741505755787294, "support": 372929}, "weighted avg": {"f1-score": 0.9727512443294726, "precision": 0.9731955875876339, "recall": 0.9741505755787294, "support": 372929}, "\u2205": {"f1-score": 0.9873673548482204, "precision": 0.980927769300181, "recall": 0.993892048174316, "support": 208908}, "\u23ce": {"f1-score": 0.9241223103057759, "precision": 0.8928500377989098, "recall": 0.9576647319904404, "support": 23432}, "\u23ce\u23ce": {"f1-score": 0.6514983351831298, "precision": 0.9162330905306972, "recall": 0.5054535017221584, "support": 3484}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8882495581768115, "precision": 0.9736889529644981, "recall": 0.8165948764452505, "support": 13233}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 75}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9426270048752915, "precision": 0.9685639610861042, "recall": 0.9180429397192403, "support": 14532}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 316}, "\u2423": {"f1-score": 0.974514495576206, "precision": 0.971918530440091, "recall": 0.9771243653406237, "support": 71692}},
  "cl_report_full": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 2634}, "\u0027": {"f1-score": 0.9890166392915803, "precision": 1.0, "recall": 0.9782719258589512, "support": 35392}, "macro avg": {"f1-score": 0.6900934396527673, "precision": 0.770418234212048, "recall": 0.6529929066100871, "support": 391550}, "micro avg": {"f1-score": 0.9504224445668226, "precision": 0.9741505755787294, "recall": 0.9278227557144682, "support": 391550}, "weighted avg": {"f1-score": 0.945104924932102, "precision": 0.9716387385824986, "recall": 0.9278227557144682, "support": 391550}, "\u2205": {"f1-score": 0.9807748627788118, "precision": 0.980927769300181, "recall": 0.9806220039199943, "support": 211735}, "\u23ce": {"f1-score": 0.8478802992518703, "precision": 0.8928500377989098, "recall": 0.8072232814130005, "support": 27799}, "\u23ce\u23ce": {"f1-score": 0.3947545393409549, "precision": 0.9162330905306972, "recall": 0.25157142857142856, "support": 7000}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.814287329038092, "precision": 0.9736889529644981, "recall": 0.699734507543871, "support": 15443}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 90}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9287156282631395, "precision": 0.9685639610861042, "recall": 0.8920165819737897, "support": 14956}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 398}, "\u2423": {"f1-score": 0.9455050985632242, "precision": 0.971918530440091, "recall": 0.9204893368198362, "support": 76103}},
  "ppcr": 0.9524428553186055
}
```
</details>
