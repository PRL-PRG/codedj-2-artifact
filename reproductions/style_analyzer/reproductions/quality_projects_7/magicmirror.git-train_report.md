# Train report for javascript / file:///tmp/top-repos-quality-repos-b_5io10d/magicmirror.git HEAD a31546b1ff8c7c10e12d7acbab063297d59c6b93

### Classification report

PPCR: 0.662

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.991| 0.998| 0.919| 0.994| 0.954| 24629| 26730| 0.921 |
| `"` | 1.000| 1.000| 0.968| 1.000| 0.984| 5155| 5324| 0.968 |
| `␣` | 0.992| 0.967| 0.258| 0.980| 0.409| 3459| 12983| 0.266 |
| `⏎⇥⁻` | 0.943| 0.926| 0.631| 0.934| 0.756| 1171| 1719| 0.681 |
| `⏎` | 0.922| 0.908| 0.129| 0.915| 0.227| 404| 2836| 0.142 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 33| 1780| 0.019 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 22| 1343| 0.016 |
| `micro avg` | 0.990| 0.990| 0.655| 0.990| 0.788| 34873| 52715| 0.662 |
| `weighted avg` | 0.989| 0.990| 0.655| 0.989| 0.721| 34873| 52715| 0.662 |
| `macro avg` | 0.693| 0.686| 0.415| 0.689| 0.476| 34873| 52715| 0.662 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎⇥⁺| ⏎⇥⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|2101 |24576 |13 |0 |0 |0 |40 |0 |
|9524 |91 |3346 |0 |0 |0 |22 |0 |
|169 |0 |0 |5155 |0 |0 |0 |0 |
|2432 |27 |6 |0 |367 |0 |4 |0 |
|1747 |25 |7 |0 |1 |0 |0 |0 |
|548 |79 |0 |0 |8 |0 |1084 |0 |
|1321 |0 |0 |0 |22 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| tests/e2e/env_spec.js | 17 |
| modules/default/calendar/calendarfetcher.js | 16 |
| modules/default/newsfeed/newsfeed.js | 13 |
| modules/default/weather/weather.js | 13 |
| js/main.js | 13 |
| modules/default/calendar/calendar.js | 12 |
| js/loader.js | 11 |
| modules/default/alert/alert.js | 9 |
| clientonly/index.js | 9 |
| modules/default/weather/providers/openweathermap.js | 8 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 5155}, "macro avg": {"f1-score": 0.6890542765137238, "precision": 0.6925794794310937, "recall": 0.6856142902353024, "support": 34873}, "micro avg": {"f1-score": 0.9901069595388983, "precision": 0.9901069595388983, "recall": 0.9901069595388983, "support": 34873}, "weighted avg": {"f1-score": 0.9892793410153967, "precision": 0.9885059952402215, "recall": 0.9901069595388983, "support": 34873}, "\u2205": {"f1-score": 0.9944362393024055, "precision": 0.991047665134285, "recall": 0.997848065288887, "support": 24629}, "\u23ce": {"f1-score": 0.915211970074813, "precision": 0.9221105527638191, "recall": 0.9084158415841584, "support": 404}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 33}, "\u23ce\u21e5\u207b": {"f1-score": 0.9340801378716069, "precision": 0.9426086956521739, "recall": 0.9257045260461144, "support": 1171}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u2423": {"f1-score": 0.9796515883472404, "precision": 0.9922894424673784, "recall": 0.967331598727956, "support": 3459}},
  "cl_report_full": {"\"": {"f1-score": 0.9838725069185991, "precision": 1.0, "recall": 0.9682569496619083, "support": 5324}, "macro avg": {"f1-score": 0.4756515241216813, "precision": 0.6925794794310937, "recall": 0.4150573962377943, "support": 52715}, "micro avg": {"f1-score": 0.7884185048180115, "precision": 0.9901069595388983, "recall": 0.6549938347718866, "support": 52715}, "weighted avg": {"f1-score": 0.7206776581543104, "precision": 0.9282566213802085, "recall": 0.6549938347718866, "support": 52715}, "\u2205": {"f1-score": 0.9538891476478808, "precision": 0.991047665134285, "recall": 0.9194163860830528, "support": 26730}, "\u23ce": {"f1-score": 0.22696351267779835, "precision": 0.9221105527638191, "recall": 0.12940761636107193, "support": 2836}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1780}, "\u23ce\u21e5\u207b": {"f1-score": 0.755663994423144, "precision": 0.9426086956521739, "recall": 0.6305991855730075, "support": 1719}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1343}, "\u2423": {"f1-score": 0.40917150718434725, "precision": 0.9922894424673784, "recall": 0.2577216359855195, "support": 12983}},
  "ppcr": 0.6615384615384615
}
```
</details>
