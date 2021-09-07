# Model report for file:///tmp/top-repos-quality-repos-t4upby31/devcontra-client.git HEAD 7abd70e6679357276ccce4d563f45fedc1ecbfd2

### Dump

```json
{'created_at': '2021-09-01 23:21:13',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.15.0-135-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '17.3 kB',
 'tags': [],
 'uuid': '8bd15238-4cd6-42c8-b686-f3f957724ae3',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-t4upby31/devcontra-client.git 7abd70e6679357276ccce4d563f45fedc1ecbfd2

# javascript
35 rules, avg.len. 5.8
## train
PPCR: 0.952177
### report
macro
{'f1-score': 0.6700755621571188,
 'precision': 0.7024389815222976,
 'recall': 0.6504604122676212,
 'support': 14674}
micro
{'f1-score': 0.9157693880332561,
 'precision': 0.9157693880332561,
 'recall': 0.9157693880332561,
 'support': 14674}
weighted
{'f1-score': 0.9057109106016209,
 'precision': 0.9026567849758159,
 'recall': 0.9157693880332561,
 'support': 14674}
### report_full
macro
{'f1-score': 0.6482742276715916,
 'precision': 0.7024389815222976,
 'recall': 0.6125219837031991,
 'support': 15411}
micro
{'f1-score': 0.8933355492770484,
 'precision': 0.9157693880332561,
 'recall': 0.8719745636233859,
 'support': 15411}
weighted
{'f1-score': 0.8793883364557737,
 'precision': 0.8966816404983129,
 'recall': 0.8719745636233859,
 'support': 15411}
## test
PPCR: 0.899386
### report
macro
{'f1-score': 0.5554089392777833,
 'precision': 0.6164602145744371,
 'recall': 0.5378380604571535,
 'support': 2342}
micro
{'f1-score': 0.8377455166524339,
 'precision': 0.8377455166524338,
 'recall': 0.8377455166524338,
 'support': 2342}
weighted
{'f1-score': 0.8234443699103343,
 'precision': 0.833949333169961,
 'recall': 0.8377455166524338,
 'support': 2342}
### report_full
macro
{'f1-score': 0.5096865885265909,
 'precision': 0.6164602145744371,
 'recall': 0.46123704844916064,
 'support': 2604}
micro
{'f1-score': 0.7933683784876667,
 'precision': 0.8377455166524338,
 'recall': 0.7534562211981567,
 'support': 2604}
weighted
{'f1-score': 0.7677662173332769,
 'precision': 0.81674572438964,
 'recall': 0.7534562211981567,
 'support': 2604}
```

## javascript
### Summary
21 rules, avg.len. 5.2

| | |
|-|-|
|Min support|144|
|Max support|5202|
|Min confidence|0.9247400760650635|
|Max confidence|0.9995941519737244|

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
                     'base_model_name': 'sklearn.ensemble.RandomForestClassifier',
                     'confidence_threshold': 0.8,
                     'max_depth': 10,
                     'min_samples_leaf': 90,
                     'min_samples_split': 180,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.reserved = ,<br>	∧ +1.reserved = }<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.998. Support: 270.` |
| 2 | `  •••start_line ≥ 132<br>	∧ -1.reserved = ,<br>	∧ +1.reserved not in {}}<br>⇒ y = ⏎<br>Confidence: 0.981. Support: 1210.` |
| 3 | `  •••start_line ≤ 132<br>	∧ -1.reserved = ,<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved = :<br>⇒ y = ⏎<br>Confidence: 0.992. Support: 190.` |
| 4 | `  -1.reserved = :<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1232.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.982. Support: 256.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -3.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.938. Support: 5125.` |
| 7 | `  •••start_line ≥ 136<br>	∧ -1.reserved = ,<br>	∧ +1.reserved not in {}}<br>⇒ y = ⏎<br>Confidence: 0.981. Support: 1107.` |
| 8 | `  •••start_line ≤ 135<br>	∧ -1.reserved = ,<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type = ObjectExpression<br>⇒ y = ⏎<br>Confidence: 0.970. Support: 182.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -3.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.936. Support: 5161.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {MAP}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.967. Support: 260.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.internal_type not in {File}<br>⇒ y = ∅<br>Confidence: 0.932. Support: 5202.` |
| 12 | `  -1.reserved = ,<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.986. Support: 1188.` |
| 13 | `  -1.reserved = ,<br>	∧ +1.reserved not in {}}<br>	∧ +4.reserved = ,<br>	∧ ^1.roles not in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.983. Support: 144.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.977. Support: 239.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.925. Support: 4232.` |
| 16 | `  -1.reserved = ,<br>	∧ +1.reserved not in {}}<br>	∧ +4.reserved = ,<br>⇒ y = ⏎<br>Confidence: 0.998. Support: 1081.` |
| 17 | `  -1.reserved = ,<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.reserved not in {}}<br>	∧ +4.reserved not in {,}<br>⇒ y = ⏎<br>Confidence: 0.959. Support: 231.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved = :<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.970. Support: 247.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {KEY}<br>	∧ +1.length ≥ 2<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.995. Support: 287.` |
| 20 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>⇒ y = ∅<br>Confidence: 0.962. Support: 942.` |
| 21 | `  -1.reserved = ,<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved = :<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 1000.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.190476190476191, "max_conf": 0.9995941519737244, "max_support": 5202, "min_conf": 0.9247400760650635, "min_support": 144, "num_rules": 21}}
```
</details>
