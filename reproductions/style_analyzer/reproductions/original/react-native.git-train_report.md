# Train report for javascript / file:///tmp/top-repos-quality-repos-92qzanha/react-native.git HEAD 1bc885b8b856c7a050f0df68d9a09ca7581d0220

### Classification report

PPCR: 0.838

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.981| 0.996| 0.936| 0.988| 0.958| 287901| 306357| 0.940 |
| `␣` | 0.977| 0.986| 0.895| 0.981| 0.934| 154570| 170257| 0.908 |
| `⏎` | 0.956| 0.945| 0.387| 0.951| 0.551| 18757| 45839| 0.409 |
| `⏎␣⁺␣⁺` | 0.978| 0.806| 0.495| 0.884| 0.657| 17426| 28391| 0.614 |
| `'` | 0.983| 1.000| 0.854| 0.991| 0.914| 17295| 20243| 0.854 |
| `⏎␣⁻␣⁻` | 0.964| 0.805| 0.400| 0.877| 0.565| 11497| 23159| 0.496 |
| `"` | 1.000| 0.903| 0.403| 0.949| 0.575| 3139| 7023| 0.447 |
| `⏎⏎` | 0.984| 0.786| 0.117| 0.874| 0.209| 990| 6639| 0.149 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.855| 0.332| 0.060| 0.478| 0.113| 410| 2256| 0.182 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 268| 648| 0.414 |
| `⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 29| 457| 0.063 |
| `⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 141| 0.028 |
| `⏎⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 193| 0.016 |
| `weighted avg` | 0.978| 0.978| 0.820| 0.977| 0.873| 512289| 611603| 0.838 |
| `macro avg` | 0.667| 0.581| 0.350| 0.613| 0.421| 512289| 611603| 0.838 |
| `micro avg` | 0.978| 0.978| 0.820| 0.978| 0.892| 512289| 611603| 0.838 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| '| ⏎⏎| "| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻| ⏎⏎␣⁻␣⁻| ⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|18456 |286692 |1062 |0 |78 |69 |0 |0 |0 |0 |0 |0 |0 |0 |
|15687 |1651 |152446 |268 |176 |28 |0 |0 |0 |1 |0 |0 |0 |0 |
|27082 |302 |638 |17733 |0 |71 |0 |13 |0 |0 |0 |0 |0 |0 |
|10965 |2164 |1168 |42 |14052 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|11662 |1302 |662 |257 |0 |9254 |0 |0 |0 |22 |0 |0 |0 |0 |
|2948 |0 |0 |0 |0 |0 |17295 |0 |0 |0 |0 |0 |0 |0 |
|5649 |12 |9 |191 |0 |0 |0 |778 |0 |0 |0 |0 |0 |0 |
|3884 |0 |0 |0 |0 |0 |306 |0 |2833 |0 |0 |0 |0 |0 |
|1846 |57 |14 |38 |0 |165 |0 |0 |0 |136 |0 |0 |0 |0 |
|380 |122 |80 |0 |66 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|428 |1 |0 |17 |0 |11 |0 |0 |0 |0 |0 |0 |0 |0 |
|190 |0 |2 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|137 |3 |0 |0 |0 |1 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| Libraries/Renderer/implementations/ReactNativeRenderer-profiling.js | 793 |
| Libraries/Renderer/implementations/ReactNativeRenderer-prod.js | 771 |
| Libraries/Renderer/implementations/ReactNativeRenderer-profiling.fb.js | 765 |
| Libraries/Renderer/implementations/ReactFabric-profiling.js | 754 |
| Libraries/Renderer/implementations/ReactNativeRenderer-prod.fb.js | 743 |
| Libraries/Renderer/implementations/ReactFabric-prod.js | 732 |
| Libraries/Renderer/implementations/ReactFabric-profiling.fb.js | 729 |
| Libraries/Renderer/implementations/ReactFabric-prod.fb.js | 706 |
| Libraries/Lists/__tests__/VirtualizedList-test.js | 160 |
| docs/generatedComponentApiDocs.js | 140 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.948760884125921, "precision": 1.0, "recall": 0.9025167250716789, "support": 3139}, "\u0027": {"f1-score": 0.9912310866574966, "precision": 0.9826146241690813, "recall": 1.0, "support": 17295}, "macro avg": {"f1-score": 0.6133286947110018, "precision": 0.6674565610519588, "recall": 0.5814489127540143, "support": 512289}, "micro avg": {"f1-score": 0.9783911034591802, "precision": 0.9783911034591802, "recall": 0.9783911034591802, "support": 512289}, "weighted avg": {"f1-score": 0.9774235717425646, "precision": 0.9776807645892986, "recall": 0.9783911034591802, "support": 512289}, "\u2205": {"f1-score": 0.9882404038558653, "precision": 0.9807940993342593, "recall": 0.9958006398032657, "support": 287901}, "\u23ce": {"f1-score": 0.950729144327686, "precision": 0.9561115005122122, "recall": 0.9454070480354001, "support": 18757}, "\u23ce\u23ce": {"f1-score": 0.8736664795058954, "precision": 0.9835651074589128, "recall": 0.7858585858585858, "support": 990}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8838291716460155, "precision": 0.9777344837183413, "recall": 0.8063812693676116, "support": 17426}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 268}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8773227152066743, "precision": 0.9640587561204292, "recall": 0.8049056275550144, "support": 11497}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.47803163444639724, "precision": 0.8553459119496856, "recall": 0.33170731707317075, "support": 410}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 29}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u2423": {"f1-score": 0.9814615114710721, "precision": 0.9767108104125422, "recall": 0.9862586530374587, "support": 154570}},
  "cl_report_full": {"\"": {"f1-score": 0.5748782467532468, "precision": 1.0, "recall": 0.4033888651573402, "support": 7023}, "\u0027": {"f1-score": 0.91401543177254, "precision": 0.9826146241690813, "recall": 0.854369411648471, "support": 20243}, "macro avg": {"f1-score": 0.4212331454591902, "precision": 0.6674565610519588, "recall": 0.34983162123258765, "support": 611603}, "micro avg": {"f1-score": 0.8919344563356622, "precision": 0.9783911034591802, "recall": 0.8195169088444628, "support": 611603}, "weighted avg": {"f1-score": 0.87256971959297, "precision": 0.9745722287712888, "recall": 0.8195169088444628, "support": 611603}, "\u2205": {"f1-score": 0.9577742402653914, "precision": 0.9807940993342593, "recall": 0.935810182238369, "support": 306357}, "\u23ce": {"f1-score": 0.5508340322430343, "precision": 0.9561115005122122, "recall": 0.3868539889613648, "support": 45839}, "\u23ce\u23ce": {"f1-score": 0.20942126514131898, "precision": 0.9835651074589128, "recall": 0.11718632324145202, "support": 6639}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 193}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.6572036573673502, "precision": 0.9777344837183413, "recall": 0.49494558134620126, "support": 28391}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 648}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.5649917577385677, "precision": 0.9640587561204292, "recall": 0.39958547432963426, "support": 23159}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.1126293995859213, "precision": 0.8553459119496856, "recall": 0.06028368794326241, "support": 2256}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 457}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 141}, "\u2423": {"f1-score": 0.9342828601021026, "precision": 0.9767108104125422, "recall": 0.8953875611575441, "support": 170257}},
  "ppcr": 0.8376168854632825
}
```
</details>
