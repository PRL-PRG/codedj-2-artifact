# Train report for javascript / file:///tmp/top-repos-quality-repos-ob_dfvcz/full-stack-web-development.git HEAD 9be50607073550aba331a1923eb2854ec42d3c4f

### Classification report

PPCR: 0.578

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.988| 0.998| 0.849| 0.993| 0.913| 7790| 9150| 0.851 |
| `␣` | 0.963| 0.896| 0.309| 0.928| 0.468| 766| 2221| 0.345 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 15| 1138| 0.013 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 225| 0.036 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 186| 0.005 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1224| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 508| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 181| 0.000 |
| `weighted avg` | 0.983| 0.986| 0.570| 0.984| 0.633| 8580| 14833| 0.578 |
| `micro avg` | 0.986| 0.986| 0.570| 0.986| 0.722| 8580| 14833| 0.578 |
| `macro avg` | 0.244| 0.237| 0.145| 0.240| 0.173| 8580| 14833| 0.578 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| '| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1360 |7771 |19 |0 |0 |0 |0 |0 |0 |
|1455 |80 |686 |0 |0 |0 |0 |0 |0 |
|1123 |12 |3 |0 |0 |0 |0 |0 |0 |
|1224 |0 |0 |0 |0 |0 |0 |0 |0 |
|508 |0 |0 |0 |0 |0 |0 |0 |0 |
|217 |4 |4 |0 |0 |0 |0 |0 |0 |
|181 |0 |0 |0 |0 |0 |0 |0 |0 |
|185 |1 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| stud_table.js | 21 |
| jsonp7.js | 12 |
| java script/Activities/Pig Game/challenges.js | 11 |
| java script/Activities/Pig Game/app.js | 9 |
| java script/Activities/weather.js | 8 |
| af_iife_prog.js | 7 |
| Productapp/client/dashboard.js | 7 |
| java script/Activities/corona/form.js | 7 |
| java script/Activities/Rollover Image/main.js | 6 |
| java script/Activities/asyncweather.js | 6 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.24010913848463547, "precision": 0.24389434089442086, "recall": 0.23664029166401324, "support": 8580}, "micro avg": {"f1-score": 0.9856643356643356, "precision": 0.9856643356643356, "recall": 0.9856643356643356, "support": 8580}, "weighted avg": {"f1-score": 0.9840737208913125, "precision": 0.982749383049275, "recall": 0.9856643356643356, "support": 8580}, "\u2205": {"f1-score": 0.9925916464427129, "precision": 0.9876715810879512, "recall": 0.9975609756097561, "support": 7790}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 15}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9282814614343708, "precision": 0.9634831460674157, "recall": 0.8955613577023499, "support": 766}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1224}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 508}, "macro avg": {"f1-score": 0.17263109171785257, "precision": 0.24389434089442086, "recall": 0.14476993698993462, "support": 14833}, "micro avg": {"f1-score": 0.7224191688378252, "precision": 0.9856643356643356, "recall": 0.5701476437672757, "support": 14833}, "weighted avg": {"f1-score": 0.6334083002149327, "precision": 0.7535286883550518, "recall": 0.5701476437672757, "support": 14833}, "\u2205": {"f1-score": 0.9132683041485485, "precision": 0.9876715810879512, "recall": 0.8492896174863388, "support": 9150}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1138}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 186}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 225}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 181}, "\u2423": {"f1-score": 0.4677804295942721, "precision": 0.9634831460674157, "recall": 0.30886987843313823, "support": 2221}},
  "ppcr": 0.5784399649430324
}
```
</details>
