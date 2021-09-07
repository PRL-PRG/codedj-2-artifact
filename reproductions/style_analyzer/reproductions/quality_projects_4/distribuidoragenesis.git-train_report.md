# Train report for javascript / file:///tmp/top-repos-quality-repos-2pdd6bsp/distribuidoragenesis.git HEAD ef55a3ba001fd57db80e68a9add969136e76ff95

### Classification report

PPCR: 0.866

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.974| 0.984| 0.975| 0.979| 0.975| 74043| 74698| 0.991 |
| `␣` | 0.951| 0.970| 0.907| 0.960| 0.928| 43609| 46646| 0.935 |
| `'` | 0.984| 1.000| 0.952| 0.992| 0.968| 30704| 32269| 0.952 |
| `⏎` | 0.987| 0.910| 0.606| 0.947| 0.751| 8036| 12076| 0.665 |
| `⏎⏎` | 0.960| 0.924| 0.264| 0.942| 0.414| 1652| 5791| 0.285 |
| `⏎␣⁻␣⁻` | 1.000| 0.564| 0.083| 0.721| 0.154| 633| 4291| 0.148 |
| `"` | 1.000| 0.113| 0.071| 0.203| 0.132| 550| 878| 0.626 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 416| 4266| 0.098 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 1.000| 0.959| 0.119| 0.979| 0.212| 146| 1181| 0.124 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 115| 772| 0.149 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 101| 1191| 0.085 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 42| 663| 0.063 |
| `macro avg` | 0.655| 0.535| 0.331| 0.560| 0.378| 160047| 184722| 0.866 |
| `weighted avg` | 0.966| 0.970| 0.840| 0.967| 0.865| 160047| 184722| 0.866 |
| `micro avg` | 0.970| 0.970| 0.840| 0.970| 0.901| 160047| 184722| 0.866 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| "| ⏎⇥⁺| ⏎⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|655 |72839 |1174 |0 |30 |0 |0 |0 |0 |0 |0 |0 |0 |
|3037 |1275 |42308 |0 |25 |1 |0 |0 |0 |0 |0 |0 |0 |
|1565 |0 |0 |30704 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|4040 |74 |586 |0 |7314 |62 |0 |0 |0 |0 |0 |0 |0 |
|4139 |8 |80 |0 |38 |1526 |0 |0 |0 |0 |0 |0 |0 |
|3850 |323 |93 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3658 |166 |110 |0 |0 |0 |0 |357 |0 |0 |0 |0 |0 |
|1090 |18 |83 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1035 |1 |5 |0 |0 |0 |0 |0 |0 |140 |0 |0 |0 |
|328 |0 |0 |488 |0 |0 |0 |0 |0 |0 |62 |0 |0 |
|657 |45 |70 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|621 |38 |0 |0 |4 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| public/plugins/datatables/Responsive-2.2.2/js/dataTables.responsive.js | 932 |
| public/plugins/fileinput/js/plugins/piexif.js | 676 |
| public/plugins/select2/js/select2.full.js | 426 |
| public/plugins/select2/js/select2.js | 318 |
| public/plugins/fileinput/js/plugins/sortable.js | 244 |
| public/plugins/numeric.js | 204 |
| public/plugins/datatables/DataTables-1.10.18/js/dataTables.jqueryui.js | 190 |
| public/plugins/fileinput/js/plugins/purify.js | 152 |
| public/plugins/datatables/DataTables-1.10.18/js/dataTables.semanticui.js | 143 |
| public/plugins/datatables/DataTables-1.10.18/js/dataTables.bootstrap4.js | 126 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.20261437908496732, "precision": 1.0, "recall": 0.11272727272727273, "support": 550}, "\u0027": {"f1-score": 0.992115807160398, "precision": 0.9843549628109771, "recall": 1.0, "support": 30704}, "macro avg": {"f1-score": 0.5602256048391322, "precision": 0.6546767294274922, "recall": 0.5352834526205573, "support": 160047}, "micro avg": {"f1-score": 0.9700275544058933, "precision": 0.9700275544058933, "recall": 0.9700275544058933, "support": 160047}, "weighted avg": {"f1-score": 0.9665234755311113, "precision": 0.9661964383950725, "recall": 0.9700275544058933, "support": 160047}, "\u2205": {"f1-score": 0.9788214741651547, "precision": 0.9739526923128351, "recall": 0.9837391785854166, "support": 74043}, "\u23ce": {"f1-score": 0.9469799961157507, "precision": 0.9869113479962218, "recall": 0.9101543056246889, "support": 8036}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 115}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 42}, "\u23ce\u23ce": {"f1-score": 0.9416846652267818, "precision": 0.960352422907489, "recall": 0.923728813559322, "support": 1652}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 416}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 101}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7212121212121213, "precision": 1.0, "recall": 0.5639810426540285, "support": 633}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9790209790209791, "precision": 1.0, "recall": 0.958904109589041, "support": 146}, "\u2423": {"f1-score": 0.9602578360834336, "precision": 0.9505493271023838, "recall": 0.9701667087069183, "support": 43609}},
  "cl_report_full": {"\"": {"f1-score": 0.13191489361702127, "precision": 1.0, "recall": 0.07061503416856492, "support": 878}, "\u0027": {"f1-score": 0.9676494224799482, "precision": 0.9843549628109771, "recall": 0.9515014410114971, "support": 32269}, "macro avg": {"f1-score": 0.3776784527089713, "precision": 0.6546767294274922, "recall": 0.3312623950382433, "support": 184722}, "micro avg": {"f1-score": 0.9006030124518156, "precision": 0.9700275544058933, "recall": 0.8404521388897912, "support": 184722}, "weighted avg": {"f1-score": 0.8651142803125309, "precision": 0.9348379442449141, "recall": 0.8404521388897912, "support": 184722}, "\u2205": {"f1-score": 0.9745325617954979, "precision": 0.9739526923128351, "recall": 0.9751131221719457, "support": 74698}, "\u23ce": {"f1-score": 0.7506542823420742, "precision": 0.9869113479962218, "recall": 0.6056641271944353, "support": 12076}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 772}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 663}, "\u23ce\u23ce": {"f1-score": 0.413550135501355, "precision": 0.960352422907489, "recall": 0.26351234674494906, "support": 5791}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4266}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1191}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.1536144578313253, "precision": 1.0, "recall": 0.08319738988580751, "support": 4291}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.21196063588190764, "precision": 1.0, "recall": 0.11854360711261643, "support": 1181}, "\u2423": {"f1-score": 0.9282650430585268, "precision": 0.9505493271023838, "recall": 0.9070016721691034, "support": 46646}},
  "ppcr": 0.8664208919349076
}
```
</details>
