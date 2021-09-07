# Train report for javascript / file:///tmp/top-repos-quality-repos-uw4ywgtf/create-react-app.git HEAD 08dc7ab0f520f680b678556edeb702a5791af20e

### Classification report

PPCR: 0.739

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.935| 0.999| 0.928| 0.966| 0.931| 9061| 9756| 0.929 |
| `'` | 1.000| 1.000| 0.987| 1.000| 0.993| 2816| 2854| 0.987 |
| `␣` | 0.978| 0.883| 0.359| 0.928| 0.526| 1695| 4166| 0.407 |
| `⏎` | 0.964| 0.911| 0.434| 0.937| 0.598| 923| 1940| 0.476 |
| `⏎␣⁻␣⁻` | 1.000| 0.792| 0.609| 0.884| 0.757| 662| 860| 0.770 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 221| 896| 0.247 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 46| 407| 0.113 |
| `weighted avg` | 0.940| 0.955| 0.705| 0.946| 0.762| 15424| 20879| 0.739 |
| `macro avg` | 0.697| 0.655| 0.474| 0.674| 0.544| 15424| 20879| 0.739 |
| `micro avg` | 0.955| 0.955| 0.705| 0.955| 0.811| 15424| 20879| 0.739 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|695 |9049 |12 |0 |0 |0 |0 |0 |
|2471 |197 |1497 |0 |1 |0 |0 |0 |
|38 |0 |0 |2816 |0 |0 |0 |0 |
|1017 |82 |0 |0 |841 |0 |0 |0 |
|675 |201 |20 |0 |0 |0 |0 |0 |
|198 |125 |0 |0 |13 |0 |524 |0 |
|361 |28 |1 |0 |17 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| packages/create-react-app/createReactApp.js | 231 |
| packages/react-scripts/config/webpack.config.js | 114 |
| packages/react-scripts/fixtures/kitchensink/template/integration/syntax.test.js | 43 |
| packages/eslint-config-react-app/index.js | 40 |
| packages/cra-template/template/src/serviceWorker.js | 35 |
| packages/react-scripts/fixtures/kitchensink/template/integration/webpack.test.js | 31 |
| packages/babel-preset-react-app/create.js | 28 |
| packages/babel-plugin-named-asset-import/index.js | 26 |
| docusaurus/website/src/pages/index.js | 20 |
| packages/babel-preset-react-app/dependencies.js | 18 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 2816}, "macro avg": {"f1-score": 0.6735212993006853, "precision": 0.6967859799884204, "recall": 0.6549373617633456, "support": 15424}, "micro avg": {"f1-score": 0.9548106846473029, "precision": 0.9548106846473029, "recall": 0.9548106846473029, "support": 15424}, "weighted avg": {"f1-score": 0.9458403738681077, "precision": 0.9397840051579995, "recall": 0.9548106846473029, "support": 15424}, "\u2205": {"f1-score": 0.965587152536947, "precision": 0.9346209460855195, "recall": 0.998675642865026, "support": 9061}, "\u23ce": {"f1-score": 0.9370473537604457, "precision": 0.9644495412844036, "recall": 0.9111592632719393, "support": 923}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 46}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 221}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8836424957841484, "precision": 1.0, "recall": 0.7915407854984894, "support": 662}, "\u2423": {"f1-score": 0.9283720930232559, "precision": 0.9784313725490196, "recall": 0.8831858407079646, "support": 1695}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9932980599647266, "precision": 1.0, "recall": 0.9866853538892782, "support": 2854}, "macro avg": {"f1-score": 0.5436241664105734, "precision": 0.6967859799884204, "recall": 0.4737660147752346, "support": 20879}, "micro avg": {"f1-score": 0.8113379059581852, "precision": 0.9548106846473029, "recall": 0.7053498730782125, "support": 20879}, "weighted avg": {"f1-score": 0.762475759635661, "precision": 0.8994367143130076, "recall": 0.7053498730782125, "support": 20879}, "\u2205": {"f1-score": 0.9310628665500567, "precision": 0.9346209460855195, "recall": 0.9275317753177532, "support": 9756}, "\u23ce": {"f1-score": 0.5981507823613087, "precision": 0.9644495412844036, "recall": 0.43350515463917527, "support": 1940}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 407}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 896}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7572254335260116, "precision": 1.0, "recall": 0.6093023255813953, "support": 860}, "\u2423": {"f1-score": 0.5256320224719102, "precision": 0.9784313725490196, "recall": 0.35933749399903986, "support": 4166}},
  "ppcr": 0.7387326979261459
}
```
</details>
