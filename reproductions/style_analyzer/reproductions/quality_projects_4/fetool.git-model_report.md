# Model report for file:///tmp/top-repos-quality-repos-xbezziop/fetool.git HEAD 5d553810ecfdf3218461a63d942aa1b5dc1073b3

### Dump

```json
{'created_at': '2021-09-02 16:58:51',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-74-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '26.8 kB',
 'tags': [],
 'uuid': '0f4d7d2b-92fe-4a5f-b7d1-ff215a43f490',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-xbezziop/fetool.git 5d553810ecfdf3218461a63d942aa1b5dc1073b3

# javascript
38 rules, avg.len. 9.6
## train
PPCR: 0.812691
### report
macro
{'f1-score': 0.4331674572570292,
 'precision': 0.43550634071211797,
 'recall': 0.4336664988231553,
 'support': 77495}
micro
{'f1-score': 0.9497645009355442,
 'precision': 0.9497645009355442,
 'recall': 0.9497645009355442,
 'support': 77495}
weighted
{'f1-score': 0.9440818184165632,
 'precision': 0.939784937745247,
 'recall': 0.9497645009355442,
 'support': 77495}
### report_full
macro
{'f1-score': 0.3471991980315978,
 'precision': 0.43550634071211797,
 'recall': 0.31421269009912944,
 'support': 95356}
micro
{'f1-score': 0.8516236527413784,
 'precision': 0.9497645009355442,
 'recall': 0.771865430596921,
 'support': 95356}
weighted
{'f1-score': 0.8072391342854423,
 'precision': 0.8870304953488685,
 'recall': 0.771865430596921,
 'support': 95356}
## test
PPCR: 0.806544
### report
macro
{'f1-score': 0.2297556593580919,
 'precision': 0.2298053448960641,
 'recall': 0.23030905110344177,
 'support': 19449}
micro
{'f1-score': 0.9534680446295438,
 'precision': 0.9534680446295439,
 'recall': 0.9534680446295439,
 'support': 19449}
weighted
{'f1-score': 0.9480978399783847,
 'precision': 0.9433927048862796,
 'recall': 0.9534680446295439,
 'support': 19449}
### report_full
macro
{'f1-score': 0.17602190273767582,
 'precision': 0.2298053448960641,
 'recall': 0.15741973629950473,
 'support': 24114}
micro
{'f1-score': 0.8513646902187637,
 'precision': 0.9534680446295439,
 'recall': 0.7690138508750104,
 'support': 24114}
weighted
{'f1-score': 0.8045631298652436,
 'precision': 0.8805567205424363,
 'recall': 0.7690138508750104,
 'support': 24114}
```

## javascript
### Summary
20 rules, avg.len. 8.2

| | |
|-|-|
|Min support|130|
|Max support|12057|
|Min confidence|0.9201030731201172|
|Max confidence|0.9972826242446899|

### Configuration

```json
{'feature_extractor': {'cutoff_label_support': 80,
                       'debug_parsing': False,
                       'label_composites': '<cut>',
                       'left_features': ['length',
                                         'diff_offset',
                                         'diff_col',
                                         'diff_line',
                                         'internal_type',
                                         'label',
                                         'reserved',
                                         'roles'],
                       'left_siblings_window': 5,
                       'no_labels_on_right': True,
                       'node_features': ['start_line', 'start_col'],
                       'parent_features': ['internal_type', 'roles'],
                       'parents_depth': 2,
                       'return_sibling_indices': False,
                       'right_features': ['length', 'internal_type', 'reserved', 'roles'],
                       'right_siblings_window': 5,
                       'select_features_number': 500,
                       'selected_features': '<cut>'},
 'line_length_limit': 500,
 'lines_ratio_train_trigger': 0.2,
 'lower_bound_instances': 500,
 'optimizer': {'base_model_name_categories': ['sklearn.ensemble.RandomForestClassifier',
                                              'sklearn.tree.DecisionTreeClassifier'],
               'cv': 3,
               'max_depth_categories': [None, 5, 10],
               'max_features_categories': [None, 'auto'],
               'min_samples_leaf_max': 120,
               'min_samples_leaf_min': 90,
               'min_samples_split_max': 240,
               'min_samples_split_min': 180,
               'n_iter': 50,
               'n_jobs': -1},
 'overall_size_limit': 5242880,
 'random_state': 42,
 'test_dataset_ratio': 0.2,
 'trainable_rules': {'attribute_similarity_threshold': 0.98,
                     'base_model_name': 'sklearn.tree.DecisionTreeClassifier',
                     'confidence_threshold': 0.8,
                     'min_samples_leaf': 91,
                     'min_samples_split': 239,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 10133.` |
| 2 | `  -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = ArrayExpression<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.997. Support: 184.` |
| 3 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 4039.` |
| 4 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {FILE} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 667.` |
| 5 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.roles in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {FILE} and not in {IDENTIFIER}<br>⇒ y = ⇥<br>Confidence: 0.991. Support: 174.` |
| 6 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.diff_col ≥ 6<br>	∧ -2.diff_offset ≤ 4<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {LITERAL} and not in {FILE, IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.985. Support: 172.` |
| 7 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.diff_col ≤ 5<br>	∧ -5.label in {<tab>} and not in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {LITERAL} and not in {FILE, IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.997. Support: 146.` |
| 8 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_line = 0<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.948. Support: 9949.` |
| 9 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_line = 0<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.reserved = (<br>	∧ -3.diff_offset ≤ 4<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.956. Support: 148.` |
| 10 | `  •••start_line ≤ 175<br>	∧ -1.internal_type = StringLiteral<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -3.reserved = ,<br>	∧ -4.length ≤ 3<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.957. Support: 387.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.963. Support: 2409.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type = BinaryExpression<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.920. Support: 582.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -5.label not in {<tab>}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 12057.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 2612.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>⇒ y = ∅<br>Confidence: 0.958. Support: 1945.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ,<br>	∧ +1.length ≤ 1<br>⇒ y = ∅<br>Confidence: 0.959. Support: 788.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ;, =, }}<br>	∧ +1.length ≤ 1<br>⇒ y = ∅<br>Confidence: 0.994. Support: 755.` |
| 18 | `  -1.diff_offset ≥ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ,, ;, =, }}<br>	∧ +1.length ≤ 1<br>⇒ y = ∅<br>Confidence: 0.942. Support: 547.` |
| 19 | `  •••start_line ≥ 173<br>	∧ -1.diff_offset ≤ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ,, ;, =, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.921. Support: 133.` |
| 20 | `  •••start_col ≤ 10<br>	∧ -1.diff_offset ≤ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.length ≥ 2<br>	∧ +1.reserved not in {), ,, ;, =, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles in {INITIALIZATION}<br>⇒ y = ⇥⇥⇥⇥<br>Confidence: 0.958. Support: 130.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.2, "max_conf": 0.9972826242446899, "max_support": 12057, "min_conf": 0.9201030731201172, "min_support": 130, "num_rules": 20}}
```
</details>
