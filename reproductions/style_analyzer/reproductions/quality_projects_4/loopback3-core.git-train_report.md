# Train report for javascript / file:///tmp/top-repos-quality-repos-ell3ub5d/loopback3-core.git HEAD 75d6745945bdba4122dd0daa5a8fb0c2227cc2a4

### Classification report

PPCR: 0.879

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.964| 0.979| 0.938| 0.972| 0.951| 32377| 33803| 0.958 |
| `␣` | 0.936| 0.971| 0.890| 0.953| 0.912| 15072| 16444| 0.917 |
| `'` | 1.000| 1.000| 0.982| 1.000| 0.991| 4025| 4098| 0.982 |
| `⏎␣⁺␣⁺` | 0.923| 0.769| 0.734| 0.839| 0.818| 2316| 2424| 0.955 |
| `⏎␣⁻␣⁻` | 0.976| 0.763| 0.689| 0.857| 0.808| 2062| 2285| 0.902 |
| `⏎` | 0.866| 0.756| 0.225| 0.807| 0.357| 1209| 4071| 0.297 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 92| 1927| 0.048 |
| `macro avg` | 0.809| 0.748| 0.637| 0.775| 0.691| 57153| 65052| 0.879 |
| `weighted avg` | 0.954| 0.956| 0.840| 0.954| 0.868| 57153| 65052| 0.879 |
| `micro avg` | 0.956| 0.956| 0.840| 0.956| 0.894| 57153| 65052| 0.879 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1426 |31712 |630 |0 |2 |12 |21 |0 |
|1372 |172 |14639 |0 |108 |136 |17 |0 |
|73 |0 |0 |4025 |0 |0 |0 |0 |
|2862 |177 |118 |0 |914 |0 |0 |0 |
|108 |413 |123 |0 |0 |1780 |0 |0 |
|223 |406 |51 |0 |31 |0 |1574 |0 |
|1835 |7 |85 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| server/api/controllers/moveItemPositionController.js | 184 |
| server/api/boot/jsonApiFormatter.js | 132 |
| server/api/database/models/mysql/workbook-version.js | 108 |
| server/api/helpers/imageConverter.js | 107 |
| server/api/controllers/upload/uploadFile.js | 81 |
| server/api/docs_web/server_api_docs/source/javascripts/lib/_lunr.js | 81 |
| server/api/docs_web/client_api_docs/source/javascripts/lib/_lunr.js | 81 |
| packages/utils/lib/rabbitmq/index.js | 77 |
| packages/logger/lib/winston.js | 75 |
| server/api/database/models/mysql/item.js | 73 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 4025}, "macro avg": {"f1-score": 0.7754303343338973, "precision": 0.8094177318519696, "recall": 0.7483759587990414, "support": 57153}, "micro avg": {"f1-score": 0.9561002921981349, "precision": 0.9561002921981349, "recall": 0.9561002921981349, "support": 57153}, "weighted avg": {"f1-score": 0.9542864777797098, "precision": 0.9543889353541616, "recall": 0.9561002921981349, "support": 57153}, "\u2205": {"f1-score": 0.9718068153959304, "precision": 0.964271596679539, "recall": 0.9794607282947771, "support": 32377}, "\u23ce": {"f1-score": 0.8074204946996466, "precision": 0.8663507109004739, "recall": 0.7559966914805625, "support": 1209}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 92}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8388312912346843, "precision": 0.9232365145228216, "recall": 0.768566493955095, "support": 2316}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8568317909635276, "precision": 0.9764267990074442, "recall": 0.7633365664403492, "support": 2062}, "\u2423": {"f1-score": 0.9531219480434924, "precision": 0.9356385018535088, "recall": 0.9712712314225053, "support": 15072}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9910131724732242, "precision": 1.0, "recall": 0.9821864324060517, "support": 4098}, "macro avg": {"f1-score": 0.6909772414774612, "precision": 0.8094177318519696, "recall": 0.6368914419194398, "support": 65052}, "micro avg": {"f1-score": 0.8943005605335297, "precision": 0.9561002921981349, "recall": 0.8400049191416098, "support": 65052}, "weighted avg": {"f1-score": 0.8684167156244608, "precision": 0.923490232399445, "recall": 0.8400049191416098, "support": 65052}, "\u2205": {"f1-score": 0.9510271405008248, "precision": 0.964271596679539, "recall": 0.9381415850664142, "support": 33803}, "\u23ce": {"f1-score": 0.3566133437378073, "precision": 0.8663507109004739, "recall": 0.22451486121346106, "support": 4071}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1927}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8180147058823529, "precision": 0.9232365145228216, "recall": 0.7343234323432343, "support": 2424}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8078008724659994, "precision": 0.9764267990074442, "recall": 0.6888402625820569, "support": 2285}, "\u2423": {"f1-score": 0.9123714552820194, "precision": 0.9356385018535088, "recall": 0.8902335198248601, "support": 16444}},
  "ppcr": 0.8785740638258623
}
```
</details>
