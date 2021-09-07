# Train report for javascript / file:///tmp/top-repos-quality-repos-av0if8yx/create-react-app HEAD 32106d216e4c31fda30ec475f9f03186d116c893

### Classification report

PPCR: 0.849

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.925| 0.975| 0.933| 0.949| 0.929| 10456| 10931| 0.957 |
| `␣` | 0.928| 0.859| 0.605| 0.892| 0.732| 3428| 4870| 0.704 |
| `'` | 1.000| 1.000| 0.971| 1.000| 0.985| 2882| 2969| 0.971 |
| `⏎` | 0.917| 0.929| 0.515| 0.923| 0.659| 1158| 2091| 0.554 |
| `⏎␣⁺␣⁺` | 0.797| 0.738| 0.708| 0.766| 0.750| 866| 902| 0.960 |
| `⏎␣⁻␣⁻` | 1.000| 0.726| 0.635| 0.841| 0.777| 774| 885| 0.875 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 57| 459| 0.124 |
| `weighted avg` | 0.931| 0.933| 0.792| 0.930| 0.839| 19621| 23107| 0.849 |
| `macro avg` | 0.795| 0.747| 0.624| 0.767| 0.690| 19621| 23107| 0.849 |
| `micro avg` | 0.933| 0.933| 0.792| 0.933| 0.857| 19621| 23107| 0.849 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|475 |10199 |206 |0 |0 |51 |0 |0 |
|1442 |320 |2945 |0 |51 |112 |0 |0 |
|87 |0 |0 |2882 |0 |0 |0 |0 |
|933 |82 |0 |0 |1076 |0 |0 |0 |
|36 |206 |21 |0 |0 |639 |0 |0 |
|111 |195 |2 |0 |15 |0 |562 |0 |
|402 |25 |0 |0 |32 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| packages/create-react-app/createReactApp.js | 283 |
| docusaurus/website/pages/en/index.js | 109 |
| fixtures/utils.js | 94 |
| docusaurus/website/core/Footer.js | 92 |
| fixtures/output/webpack-message-formatting/index.test.js | 74 |
| packages/react-scripts/config/webpack.config.prod.js | 65 |
| fixtures/browser/graphql-with-mjs/src/App.js | 60 |
| packages/react-scripts/config/webpack.config.dev.js | 46 |
| packages/babel-plugin-named-asset-import/index.js | 30 |
| packages/babel-preset-react-app/create.js | 29 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 2882}, "macro avg": {"f1-score": 0.767423756272068, "precision": 0.7951493827175943, "recall": 0.7468120090640405, "support": 19621}, "micro avg": {"f1-score": 0.9328270730339941, "precision": 0.9328270730339941, "recall": 0.9328270730339941, "support": 19621}, "weighted avg": {"f1-score": 0.9302042637144687, "precision": 0.9305783517158412, "recall": 0.9328270730339941, "support": 19621}, "\u2205": {"f1-score": 0.9494949494949495, "precision": 0.9249115806656388, "recall": 0.9754208110175976, "support": 10456}, "\u23ce": {"f1-score": 0.9228130360205833, "precision": 0.9165247018739353, "recall": 0.9291882556131261, "support": 1158}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 57}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7661870503597122, "precision": 0.7967581047381546, "recall": 0.7378752886836027, "support": 866}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.841317365269461, "precision": 1.0, "recall": 0.7260981912144703, "support": 774}, "\u2423": {"f1-score": 0.8921538927597698, "precision": 0.9278512917454317, "recall": 0.8591015169194866, "support": 3428}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9851307468808751, "precision": 1.0, "recall": 0.9706972044459414, "support": 2969}, "macro avg": {"f1-score": 0.6903143050703824, "precision": 0.7951493827175943, "recall": 0.6237849682376798, "support": 23107}, "micro avg": {"f1-score": 0.8567215877176559, "precision": 0.9328270730339941, "recall": 0.79209763275198, "support": 23107}, "weighted avg": {"f1-score": 0.8390245769661662, "precision": 0.9139210300406182, "recall": 0.79209763275198, "support": 23107}, "\u2205": {"f1-score": 0.9289552782584936, "precision": 0.9249115806656388, "recall": 0.9330344890677889, "support": 10931}, "\u23ce": {"f1-score": 0.6591117917304746, "precision": 0.9165247018739353, "recall": 0.5145863223338115, "support": 2091}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 459}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7499999999999999, "precision": 0.7967581047381546, "recall": 0.7084257206208425, "support": 902}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7767795438838978, "precision": 1.0, "recall": 0.6350282485875707, "support": 885}, "\u2423": {"f1-score": 0.7322227747389359, "precision": 0.9278512917454317, "recall": 0.6047227926078029, "support": 4870}},
  "ppcr": 0.8491366252650712
}
```
</details>
