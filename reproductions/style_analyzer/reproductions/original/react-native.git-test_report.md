# Test report for javascript / file:///tmp/top-repos-quality-repos-92qzanha/react-native.git HEAD 1bc885b8b856c7a050f0df68d9a09ca7581d0220

### Classification report

PPCR: 0.933

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.975| 0.996| 0.983| 0.985| 0.979| 51507| 52174| 0.987 |
| `␣` | 0.969| 0.984| 0.950| 0.977| 0.960| 33115| 34306| 0.965 |
| `⏎` | 0.901| 0.935| 0.676| 0.917| 0.772| 6376| 8818| 0.723 |
| `⏎␣⁺␣⁺` | 0.972| 0.741| 0.571| 0.841| 0.719| 4298| 5578| 0.771 |
| `⏎␣⁻␣⁻` | 0.903| 0.763| 0.600| 0.827| 0.721| 3314| 4218| 0.786 |
| `'` | 0.882| 0.981| 0.938| 0.929| 0.909| 1714| 1792| 0.956 |
| `"` | 0.966| 0.875| 0.727| 0.918| 0.829| 1396| 1680| 0.831 |
| `⏎⏎` | 0.774| 0.855| 0.606| 0.813| 0.680| 317| 447| 0.709 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.923| 0.117| 0.064| 0.207| 0.119| 309| 565| 0.547 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 62| 114| 0.544 |
| `⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 30| 101| 0.297 |
| `⏎⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 22| 26| 0.846 |
| `⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 30| 0.300 |
| `weighted avg` | 0.962| 0.964| 0.899| 0.961| 0.922| 102469| 109849| 0.933 |
| `macro avg` | 0.636| 0.557| 0.470| 0.570| 0.515| 102469| 109849| 0.933 |
| `micro avg` | 0.964| 0.964| 0.899| 0.964| 0.930| 102469| 109849| 0.933 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| '| ⏎⏎| "| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻| ⏎⏎␣⁻␣⁻| ⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|667 |51288 |172 |1 |27 |19 |0 |0 |0 |0 |0 |0 |0 |0 |
|1191 |241 |32584 |251 |34 |3 |1 |1 |0 |0 |0 |0 |0 |0 |
|2442 |29 |251 |5960 |22 |20 |20 |73 |1 |0 |0 |0 |0 |0 |
|1280 |704 |316 |50 |3184 |0 |30 |0 |14 |0 |0 |0 |0 |0 |
|904 |283 |230 |264 |0 |2530 |0 |4 |0 |3 |0 |0 |0 |0 |
|78 |1 |4 |0 |0 |0 |1681 |0 |28 |0 |0 |0 |0 |0 |
|130 |3 |7 |29 |0 |0 |7 |271 |0 |0 |0 |0 |0 |0 |
|284 |8 |0 |0 |0 |0 |167 |0 |1221 |0 |0 |0 |0 |0 |
|256 |10 |8 |50 |0 |205 |0 |0 |0 |36 |0 |0 |0 |0 |
|52 |33 |21 |0 |8 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|71 |4 |0 |9 |0 |17 |0 |0 |0 |0 |0 |0 |0 |0 |
|4 |0 |18 |3 |0 |0 |0 |1 |0 |0 |0 |0 |0 |0 |
|21 |1 |0 |0 |0 |8 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9180451127819549, "precision": 0.9659810126582279, "recall": 0.8746418338108882, "support": 1396}, "\u0027": {"f1-score": 0.9287292817679558, "precision": 0.8819517313746065, "recall": 0.9807467911318553, "support": 1714}, "macro avg": {"f1-score": 0.5702922591521303, "precision": 0.6358119060539248, "recall": 0.557345314126324, "support": 102469}, "micro avg": {"f1-score": 0.9637548917233504, "precision": 0.9637548917233505, "recall": 0.9637548917233505, "support": 102469}, "weighted avg": {"f1-score": 0.9611610085469409, "precision": 0.9624891487864086, "recall": 0.9637548917233505, "support": 102469}, "\u2205": {"f1-score": 0.9852466574458276, "precision": 0.9749643570002852, "recall": 0.995748150736793, "support": 51507}, "\u23ce": {"f1-score": 0.9174170707303932, "precision": 0.9007102916729636, "recall": 0.9347553324968633, "support": 6376}, "\u23ce\u23ce": {"f1-score": 0.8125937031484257, "precision": 0.7742857142857142, "recall": 0.8548895899053628, "support": 317}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8408820810775123, "precision": 0.9722137404580152, "recall": 0.7408096789204281, "support": 4298}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 62}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8273381294964028, "precision": 0.9029264810849393, "recall": 0.7634278817139408, "support": 3314}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.20689655172413796, "precision": 0.9230769230769231, "recall": 0.11650485436893204, "support": 309}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 30}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u2423": {"f1-score": 0.9766507808050834, "precision": 0.9694445270893457, "recall": 0.9839649705571494, "support": 33115}},
  "cl_report_full": {"\"": {"f1-score": 0.829483695652174, "precision": 0.9659810126582279, "recall": 0.7267857142857143, "support": 1680}, "\u0027": {"f1-score": 0.9091400757166036, "precision": 0.8819517313746065, "recall": 0.9380580357142857, "support": 1792}, "macro avg": {"f1-score": 0.5145193001342359, "precision": 0.6358119060539248, "recall": 0.4703201599826349, "support": 109849}, "micro avg": {"f1-score": 0.9302555600561423, "precision": 0.9637548917233505, "recall": 0.8990068184507825, "support": 109849}, "weighted avg": {"f1-score": 0.9217289024823995, "precision": 0.9592305763515058, "recall": 0.8990068184507825, "support": 109849}, "\u2205": {"f1-score": 0.9789747945676137, "precision": 0.9749643570002852, "recall": 0.9830183616360639, "support": 52174}, "\u23ce": {"f1-score": 0.7722708130871395, "precision": 0.9007102916729636, "recall": 0.6758902245407121, "support": 8818}, "\u23ce\u23ce": {"f1-score": 0.6800501882057717, "precision": 0.7742857142857142, "recall": 0.6062639821029083, "support": 447}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 26}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7193041906698295, "precision": 0.9722137404580152, "recall": 0.5708139117963428, "support": 5578}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 114}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7207977207977209, "precision": 0.9029264810849393, "recall": 0.5998103366524419, "support": 4218}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.11920529801324502, "precision": 0.9230769230769231, "recall": 0.06371681415929203, "support": 565}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 101}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 30}, "\u2423": {"f1-score": 0.9595241250349692, "precision": 0.9694445270893457, "recall": 0.9498046988864921, "support": 34306}},
  "ppcr": 0.9328168667898661
}
```
</details>