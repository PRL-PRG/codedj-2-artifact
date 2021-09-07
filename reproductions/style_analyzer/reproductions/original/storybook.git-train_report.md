# Train report for javascript / file:///tmp/top-repos-quality-repos-ds7nbgjy/storybook.git HEAD ed40e92cb76fe70f488908d0b3abb3202e8408b1

### Classification report

PPCR: 0.850

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.967| 0.995| 0.941| 0.981| 0.954| 52238| 55216| 0.946 |
| `␣` | 0.939| 0.969| 0.802| 0.954| 0.865| 24967| 30183| 0.827 |
| `'` | 1.000| 1.000| 0.999| 1.000| 1.000| 12066| 12076| 0.999 |
| `⏎␣⁻␣⁻` | 1.000| 0.875| 0.783| 0.934| 0.878| 3253| 3637| 0.894 |
| `⏎␣⁺␣⁺` | 0.993| 0.225| 0.111| 0.367| 0.200| 1848| 3751| 0.493 |
| `⏎` | 0.973| 0.456| 0.120| 0.621| 0.213| 1802| 6864| 0.263 |
| `⏎⏎` | 0.664| 0.945| 0.362| 0.780| 0.469| 1067| 2786| 0.383 |
| `"` | 1.000| 1.000| 1.000| 1.000| 1.000| 614| 614| 1.000 |
| `weighted avg` | 0.962| 0.960| 0.816| 0.954| 0.853| 97855| 115127| 0.850 |
| `macro avg` | 0.942| 0.808| 0.640| 0.830| 0.697| 97855| 115127| 0.850 |
| `micro avg` | 0.960| 0.960| 0.816| 0.960| 0.882| 97855| 115127| 0.850 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2978 |51982 |256 |0 |0 |0 |0 |0 |0 |
|5216 |764 |24200 |0 |0 |3 |0 |0 |0 |
|10 |0 |0 |12066 |0 |0 |0 |0 |0 |
|5062 |322 |149 |0 |822 |0 |0 |509 |0 |
|1903 |356 |1076 |0 |0 |416 |0 |0 |0 |
|384 |323 |65 |0 |17 |0 |2848 |0 |0 |
|1719 |36 |17 |0 |6 |0 |0 |1008 |0 |
|0 |0 |0 |0 |0 |0 |0 |0 |614 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| lib/api/src/tests/stories.test.js | 98 |
| lib/api/src/tests/refs.test.js | 91 |
| lib/client-api/src/hooks.test.js | 75 |
| lib/codemod/src/transforms/storiesof-to-csf.js | 71 |
| lib/codemod/src/transforms/__testfixtures__/update-addon-info/update-addon-info.input.js | 65 |
| examples/official-storybook/stories/addon-docs/addon-docs-blocks.stories.js | 57 |
| examples/cra-ts-kitchen-sink/src/stories/docgen-tests/types/prop-types.js | 52 |
| addons/docs/src/mdx/mdx-compiler-plugin.js | 51 |
| lib/codemod/src/transforms/mdx-to-csf.js | 49 |
| lib/ui/src/components/notifications/NotificationItem.stories.js | 44 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 614}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 12066}, "macro avg": {"f1-score": 0.8295696590027213, "precision": 0.9419919612362324, "recall": 0.808231396000653, "support": 97855}, "micro avg": {"f1-score": 0.960155331868581, "precision": 0.960155331868581, "recall": 0.960155331868581, "support": 97855}, "weighted avg": {"f1-score": 0.9543885905019168, "precision": 0.9623497760091068, "recall": 0.960155331868581, "support": 97855}, "\u2205": {"f1-score": 0.9805981833787645, "precision": 0.9665135823587379, "recall": 0.9950993529614457, "support": 52238}, "\u23ce": {"f1-score": 0.6210804684548547, "precision": 0.9727810650887574, "recall": 0.45615982241953384, "support": 1802}, "\u23ce\u23ce": {"f1-score": 0.7801857585139318, "precision": 0.6644693473961767, "recall": 0.9447047797563262, "support": 1067}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.36700485222761364, "precision": 0.9928400954653938, "recall": 0.22510822510822512, "support": 1848}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.933617439763973, "precision": 1.0, "recall": 0.875499538887181, "support": 3253}, "\u2423": {"f1-score": 0.9540705696826335, "precision": 0.9393315995807942, "recall": 0.9692794488725117, "support": 24967}},
  "cl_report_full": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 614}, "\u0027": {"f1-score": 0.9995857841106784, "precision": 1.0, "recall": 0.9991719112288837, "support": 12076}, "macro avg": {"f1-score": 0.6972669015406036, "precision": 0.9419919612362324, "recall": 0.6397385973189906, "support": 115127}, "micro avg": {"f1-score": 0.8822905222037543, "precision": 0.960155331868581, "recall": 0.8161074291868979, "support": 115127}, "weighted avg": {"f1-score": 0.8527477226193375, "precision": 0.958058350034074, "recall": 0.8161074291868979, "support": 115127}, "\u2205": {"f1-score": 0.9538069156597767, "precision": 0.9665135823587379, "recall": 0.9414300202839757, "support": 55216}, "\u23ce": {"f1-score": 0.21325723180697884, "precision": 0.9727810650887574, "recall": 0.11975524475524475, "support": 6864}, "\u23ce\u23ce": {"f1-score": 0.4685103416221241, "precision": 0.6644693473961767, "recall": 0.36180904522613067, "support": 2786}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.19952038369304556, "precision": 0.9928400954653938, "recall": 0.11090375899760065, "support": 3751}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8783346183500385, "precision": 1.0, "recall": 0.7830629639813033, "support": 3637}, "\u2423": {"f1-score": 0.8651199370821864, "precision": 0.9393315995807942, "recall": 0.801775834078786, "support": 30183}},
  "ppcr": 0.849974376123759
}
```
</details>
