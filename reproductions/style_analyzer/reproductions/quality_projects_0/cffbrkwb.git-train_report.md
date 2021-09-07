# Train report for javascript / file:///tmp/top-repos-quality-repos-13uctz3q/cffbrkwb.git HEAD 8db8e4492380f6c8875835b83539262438606432

### Classification report

PPCR: 0.766

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.947| 0.993| 0.879| 0.969| 0.912| 5506| 6214| 0.886 |
| `␣` | 0.975| 0.800| 0.610| 0.879| 0.751| 1487| 1950| 0.763 |
| `"` | 1.000| 1.000| 0.576| 1.000| 0.731| 765| 1328| 0.576 |
| `⏎` | 0.796| 0.896| 0.347| 0.843| 0.483| 240| 620| 0.387 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 27| 112| 0.241 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 256| 0.000 |
| `macro avg` | 0.620| 0.615| 0.402| 0.615| 0.479| 8025| 10480| 0.766 |
| `micro avg` | 0.951| 0.951| 0.729| 0.951| 0.825| 8025| 10480| 0.766 |
| `weighted avg` | 0.950| 0.951| 0.729| 0.949| 0.802| 8025| 10480| 0.766 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| '| ⏎␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|708 |5465 |16 |0 |25 |0 |0 |
|463 |281 |1190 |0 |16 |0 |0 |
|563 |0 |0 |765 |0 |0 |0 |
|380 |18 |7 |0 |215 |0 |0 |
|256 |0 |0 |0 |0 |0 |0 |
|85 |5 |8 |0 |14 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/Components/StoreLocator.js | 82 |
| src/Components/Events.js | 58 |
| src/App.js | 45 |
| src/Components/Control.js | 37 |
| src/Components/Menu.js | 35 |
| src/registerServiceWorker.js | 33 |
| src/Components/Home.js | 25 |
| src/Components/ToggleNavigation.js | 25 |
| src/Components/About.js | 11 |
| src/Components/Partners.js | 10 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 765}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.6152359975957203, "precision": 0.6197019716260486, "recall": 0.6147759848718084, "support": 8025}, "micro avg": {"f1-score": 0.9514018691588785, "precision": 0.9514018691588785, "recall": 0.9514018691588785, "support": 8025}, "weighted avg": {"f1-score": 0.9485068361957874, "precision": 0.949684302975743, "recall": 0.9514018691588785, "support": 8025}, "\u2205": {"f1-score": 0.9694013303769401, "precision": 0.9473045588490207, "recall": 0.9925535779150018, "support": 5506}, "\u23ce": {"f1-score": 0.8431372549019608, "precision": 0.7962962962962963, "recall": 0.8958333333333334, "support": 240}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 27}, "\u2423": {"f1-score": 0.878877400295421, "precision": 0.9746109746109746, "recall": 0.8002689979825152, "support": 1487}},
  "cl_report_full": {"\"": {"f1-score": 0.7310081223124701, "precision": 1.0, "recall": 0.5760542168674698, "support": 1328}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 256}, "macro avg": {"f1-score": 0.47947192954144574, "precision": 0.6197019716260486, "recall": 0.40209175720570406, "support": 10480}, "micro avg": {"f1-score": 0.8251823831396919, "precision": 0.9514018691588785, "recall": 0.728530534351145, "support": 10480}, "weighted avg": {"f1-score": 0.8017035718891159, "precision": 0.9168650413056221, "recall": 0.728530534351145, "support": 10480}, "\u2205": {"f1-score": 0.9121255111407828, "precision": 0.9473045588490207, "recall": 0.8794657225619569, "support": 6214}, "\u23ce": {"f1-score": 0.4831460674157304, "precision": 0.7962962962962963, "recall": 0.3467741935483871, "support": 620}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 112}, "\u2423": {"f1-score": 0.7505518763796909, "precision": 0.9746109746109746, "recall": 0.6102564102564103, "support": 1950}},
  "ppcr": 0.7657442748091603
}
```
</details>
