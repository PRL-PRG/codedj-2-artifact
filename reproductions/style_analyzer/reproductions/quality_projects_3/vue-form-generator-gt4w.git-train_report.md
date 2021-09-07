# Train report for javascript / file:///tmp/top-repos-quality-repos-owvt72oi/vue-form-generator-gt4w.git HEAD 61bcec723425abee2eb0ffbaab87b36f0eb8b556

### Classification report

PPCR: 0.823

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.990| 0.999| 0.986| 0.994| 0.988| 27983| 28341| 0.987 |
| `"` | 1.000| 1.000| 0.984| 1.000| 0.992| 5864| 5962| 0.984 |
| `␣` | 0.978| 0.960| 0.567| 0.969| 0.718| 5580| 9446| 0.591 |
| `⏎⇥⁺` | 0.925| 0.946| 0.902| 0.935| 0.913| 1399| 1467| 0.954 |
| `⏎⏎` | 0.952| 0.996| 0.497| 0.974| 0.653| 563| 1129| 0.499 |
| `⏎` | 0.967| 0.570| 0.058| 0.718| 0.110| 305| 2984| 0.102 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 76| 1448| 0.052 |
| `macro avg` | 0.830| 0.782| 0.571| 0.799| 0.625| 41770| 50777| 0.823 |
| `weighted avg` | 0.985| 0.987| 0.812| 0.986| 0.849| 41770| 50777| 0.823 |
| `micro avg` | 0.987| 0.987| 0.812| 0.987| 0.891| 41770| 50777| 0.823 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎⇥⁺| ⏎⇥⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|358 |27948 |35 |0 |0 |0 |0 |0 |
|3866 |112 |5356 |0 |5 |107 |0 |0 |
|98 |0 |0 |5864 |0 |0 |0 |0 |
|2679 |71 |32 |0 |174 |0 |0 |28 |
|68 |22 |54 |0 |0 |1323 |0 |0 |
|1372 |76 |0 |0 |0 |0 |0 |0 |
|566 |0 |1 |0 |1 |0 |0 |561 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| test/unit/specs/VueFormGenerator.spec.js | 114 |
| test/unit/specs/fields/fieldVueMultiSelect.spec.js | 34 |
| src/utils/validators.js | 26 |
| test/unit/specs/utils/schema.spec.js | 23 |
| dev/projects/full/schema.js | 22 |
| test/unit/specs/formGroup.spec.js | 21 |
| test/unit/specs/fields/fieldSelect.spec.js | 20 |
| test/unit/specs/fields/fieldChecklist.spec.js | 17 |
| dev/projects/full/data.js | 16 |
| test/unit/specs/fields/abstractField.spec.js | 15 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 5864}, "macro avg": {"f1-score": 0.7985551140482289, "precision": 0.8302968696285336, "recall": 0.7816029656189089, "support": 41770}, "micro avg": {"f1-score": 0.986976298779028, "precision": 0.986976298779028, "recall": 0.986976298779028, "support": 41770}, "weighted avg": {"f1-score": 0.9856543509604272, "precision": 0.9851464216746068, "recall": 0.986976298779028, "support": 41770}, "\u2205": {"f1-score": 0.9943784245356863, "precision": 0.990045697686776, "recall": 0.9987492406103706, "support": 27983}, "\u23ce": {"f1-score": 0.7175257731958762, "precision": 0.9666666666666667, "recall": 0.5704918032786885, "support": 305}, "\u23ce\u21e5\u207a": {"f1-score": 0.9353128313891834, "precision": 0.9251748251748252, "recall": 0.9456754824874911, "support": 1399}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 76}, "\u23ce\u23ce": {"f1-score": 0.9739583333333333, "precision": 0.9524617996604414, "recall": 0.9964476021314387, "support": 563}, "\u2423": {"f1-score": 0.9687104358835232, "precision": 0.9777290982110259, "recall": 0.9598566308243728, "support": 5580}},
  "cl_report_full": {"\"": {"f1-score": 0.9917131743615761, "precision": 1.0, "recall": 0.9835625628983563, "support": 5962}, "macro avg": {"f1-score": 0.6248571077299667, "precision": 0.8302968696285336, "recall": 0.5705370875866537, "support": 50777}, "micro avg": {"f1-score": 0.8909202891503777, "precision": 0.986976298779028, "recall": 0.8119030269610257, "support": 50777}, "weighted avg": {"f1-score": 0.8488373641955544, "precision": 0.9566065020289445, "recall": 0.8119030269610257, "support": 50777}, "\u2205": {"f1-score": 0.988085557716104, "precision": 0.990045697686776, "recall": 0.986133163967397, "support": 28341}, "\u23ce": {"f1-score": 0.10998735777496839, "precision": 0.9666666666666667, "recall": 0.058310991957104555, "support": 2984}, "\u23ce\u21e5\u207a": {"f1-score": 0.9133586468760787, "precision": 0.9251748251748252, "recall": 0.901840490797546, "support": 1467}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1448}, "\u23ce\u23ce": {"f1-score": 0.6530849825378346, "precision": 0.9524617996604414, "recall": 0.49689991142604073, "support": 1129}, "\u2423": {"f1-score": 0.7177700348432056, "precision": 0.9777290982110259, "recall": 0.5670124920601313, "support": 9446}},
  "ppcr": 0.8226165389841857
}
```
</details>
