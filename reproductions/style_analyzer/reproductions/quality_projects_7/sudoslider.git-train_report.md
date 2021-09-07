# Train report for javascript / file:///tmp/top-repos-quality-repos-7off3d1q/sudoslider.git HEAD 36461b800aeeda39f780d60a4e3a4203edbebe06

### Classification report

PPCR: 0.861

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.956| 0.990| 0.957| 0.972| 0.957| 9967| 10305| 0.967 |
| `␣` | 0.977| 0.937| 0.815| 0.956| 0.888| 5975| 6870| 0.870 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.948| 0.967| 0.950| 0.957| 0.949| 860| 876| 0.982 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.978| 0.954| 0.940| 0.966| 0.959| 854| 867| 0.985 |
| `"` | 0.985| 1.000| 0.831| 0.992| 0.901| 654| 787| 0.831 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 99| 1324| 0.075 |
| `'` | 1.000| 0.286| 0.167| 0.444| 0.286| 14| 24| 0.583 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 353| 0.017 |
| `micro avg` | 0.964| 0.964| 0.830| 0.964| 0.892| 18429| 21406| 0.861 |
| `weighted avg` | 0.959| 0.964| 0.830| 0.961| 0.857| 18429| 21406| 0.861 |
| `macro avg` | 0.730| 0.642| 0.582| 0.661| 0.617| 18429| 21406| 0.861 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| '| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|338 |9864 |70 |0 |0 |31 |2 |0 |0 |
|895 |347 |5597 |0 |0 |15 |16 |0 |0 |
|1225 |56 |43 |0 |0 |0 |0 |0 |0 |
|133 |0 |0 |0 |654 |0 |0 |0 |0 |
|16 |9 |19 |0 |0 |832 |0 |0 |0 |
|13 |39 |0 |0 |0 |0 |815 |0 |0 |
|347 |5 |1 |0 |0 |0 |0 |0 |0 |
|10 |0 |0 |0 |10 |0 |0 |0 |4 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| js/jquery.sudoSlider.js | 527 |
| sliderBuilder/js/sudoSliderAngular.js | 47 |
| sliderBuilder/js/optionExplorer.js | 43 |
| sliderBuilder/js/exports.js | 19 |
| sliderBuilder/js/events.js | 18 |
| js/lib/jquery.properload.js | 9 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.992412746585736, "precision": 0.9849397590361446, "recall": 1.0, "support": 654}, "\u0027": {"f1-score": 0.4444444444444445, "precision": 1.0, "recall": 0.2857142857142857, "support": 14}, "macro avg": {"f1-score": 0.6611600722411624, "precision": 0.7304427625297808, "recall": 0.6417363747509842, "support": 18429}, "micro avg": {"f1-score": 0.9640240924629659, "precision": 0.9640240924629659, "recall": 0.9640240924629659, "support": 18429}, "weighted avg": {"f1-score": 0.9610019015121766, "precision": 0.9588990618716557, "recall": 0.9640240924629659, "support": 18429}, "\u2205": {"f1-score": 0.972445408389609, "precision": 0.9558139534883721, "recall": 0.9896658974616234, "support": 9967}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 99}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9574223245109321, "precision": 0.9476082004555809, "recall": 0.9674418604651163, "support": 860}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9662122110254889, "precision": 0.978391356542617, "recall": 0.9543325526932084, "support": 854}, "\u2423": {"f1-score": 0.9563434429730885, "precision": 0.9767888307155322, "recall": 0.9367364016736401, "support": 5975}},
  "cl_report_full": {"\"": {"f1-score": 0.90144727773949, "precision": 0.9849397590361446, "recall": 0.8310038119440915, "support": 787}, "\u0027": {"f1-score": 0.2857142857142857, "precision": 1.0, "recall": 0.16666666666666666, "support": 24}, "macro avg": {"f1-score": 0.617449449212983, "precision": 0.7304427625297808, "recall": 0.5824215096872976, "support": 21406}, "micro avg": {"f1-score": 0.8919794150872348, "precision": 0.9640240924629659, "recall": 0.8299542184434271, "support": 21406}, "weighted avg": {"f1-score": 0.8567162889675803, "precision": 0.8893637175463124, "recall": 0.8299542184434271, "support": 21406}, "\u2205": {"f1-score": 0.956509090909091, "precision": 0.9558139534883721, "recall": 0.9572052401746725, "support": 10305}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1324}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 353}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9486887115165338, "precision": 0.9476082004555809, "recall": 0.9497716894977168, "support": 876}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9588235294117647, "precision": 0.978391356542617, "recall": 0.9400230680507498, "support": 867}, "\u2423": {"f1-score": 0.8884126984126984, "precision": 0.9767888307155322, "recall": 0.8147016011644833, "support": 6870}},
  "ppcr": 0.8609268429412315
}
```
</details>
