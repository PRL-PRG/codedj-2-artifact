# Train report for javascript / file:///tmp/top-repos-quality-repos-in1nv9w1/sonos.git HEAD 787364aa6e1e9bdcc7828f71a3ffca5da680c3e7

### Classification report

PPCR: 0.174

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `'` | 1.000| 1.000| 0.491| 1.000| 0.658| 1250| 2548| 0.491 |
| `␣` | 0.976| 1.000| 0.277| 0.988| 0.431| 1054| 3807| 0.277 |
| `⏎` | 0.965| 0.985| 0.295| 0.975| 0.452| 526| 1754| 0.300 |
| `⏎␣⁻␣⁻` | 1.000| 0.970| 0.616| 0.985| 0.762| 466| 734| 0.635 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 12| 805| 0.015 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 11| 386| 0.028 |
| `∅` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 8952| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 57| 0.000 |
| `micro avg` | 0.986| 0.986| 0.172| 0.986| 0.293| 3319| 19043| 0.174 |
| `macro avg` | 0.493| 0.494| 0.210| 0.493| 0.288| 3319| 19043| 0.174 |
| `weighted avg` | 0.980| 0.986| 0.172| 0.983| 0.245| 3319| 19043| 0.174 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|8952 |0 |0 |0 |0 |0 |0 |0 |0 |
|2753 |0 |1054 |0 |0 |0 |0 |0 |0 |
|1298 |0 |0 |1250 |0 |0 |0 |0 |0 |
|1228 |0 |8 |0 |518 |0 |0 |0 |0 |
|793 |0 |12 |0 |0 |0 |0 |0 |0 |
|268 |0 |6 |0 |8 |0 |452 |0 |0 |
|375 |0 |0 |0 |11 |0 |0 |0 |0 |
|57 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| packages/create-react-app/createReactApp.js | 9 |
| packages/react-scripts/config/webpack.config.js | 6 |
| packages/babel-plugin-named-asset-import/index.test.js | 5 |
| packages/eslint-config-react-app/index.js | 5 |
| packages/react-scripts/fixtures/kitchensink/integration/webpack.test.js | 4 |
| docusaurus/website/pages/en/index.js | 4 |
| docusaurus/website/siteConfig.js | 2 |
| packages/babel-preset-react-app/create.js | 2 |
| packages/create-react-app/index.js | 2 |
| tasks/cra.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1250}, "macro avg": {"f1-score": 0.4933957438610477, "precision": 0.49256802193254706, "recall": 0.4943434945087224, "support": 3319}, "micro avg": {"f1-score": 0.9864416993070203, "precision": 0.9864416993070202, "recall": 0.9864416993070202, "support": 3319}, "weighted avg": {"f1-score": 0.9830344480736082, "precision": 0.9798177538960671, "recall": 0.9864416993070202, "support": 3319}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce": {"f1-score": 0.9746001881467545, "precision": 0.9646182495344506, "recall": 0.9847908745247148, "support": 526}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9847494553376905, "precision": 1.0, "recall": 0.9699570815450643, "support": 466}, "\u2423": {"f1-score": 0.9878163074039363, "precision": 0.975925925925926, "recall": 1.0, "support": 1054}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 57}, "\u0027": {"f1-score": 0.6582411795681938, "precision": 1.0, "recall": 0.49058084772370486, "support": 2548}, "macro avg": {"f1-score": 0.28800248779618304, "precision": 0.49256802193254706, "recall": 0.20982100657921504, "support": 19043}, "micro avg": {"f1-score": 0.29281817368750557, "precision": 0.9864416993070202, "recall": 0.17192669222286405, "support": 19043}, "weighted avg": {"f1-score": 0.24533856727370432, "precision": 0.45629839887010587, "recall": 0.17192669222286405, "support": 19043}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8952}, "\u23ce": {"f1-score": 0.45220427760803145, "precision": 0.9646182495344506, "recall": 0.2953249714937286, "support": 1754}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 386}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 805}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7622259696458684, "precision": 1.0, "recall": 0.6158038147138964, "support": 734}, "\u2423": {"f1-score": 0.43134847554737055, "precision": 0.975925925925926, "recall": 0.2768584187023903, "support": 3807}},
  "ppcr": 0.17428976526807752
}
```
</details>
