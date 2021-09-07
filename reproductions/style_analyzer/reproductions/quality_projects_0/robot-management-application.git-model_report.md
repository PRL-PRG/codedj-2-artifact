# Model report for file:///tmp/top-repos-quality-repos-3rdli07t/robot-management-application.git HEAD 62d8414fac3f513a13d7bc31981ad83035cf0848

### Dump

```json
{'created_at': '2021-09-01 18:29:04',
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
 'size': '20.1 kB',
 'tags': [],
 'uuid': '93947ea6-431d-48e3-b7f8-1ea432d83d71',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-3rdli07t/robot-management-application.git 62d8414fac3f513a13d7bc31981ad83035cf0848

# javascript
27 rules, avg.len. 7.4
## train
PPCR: 0.879174
### report
macro
{'f1-score': 0.44319476129009966,
 'precision': 0.44857710520741745,
 'recall': 0.43978427502777323,
 'support': 38681}
micro
{'f1-score': 0.9410821850520927,
 'precision': 0.9410821850520927,
 'recall': 0.9410821850520927,
 'support': 38681}
weighted
{'f1-score': 0.9348372176931461,
 'precision': 0.9293738062119448,
 'recall': 0.9410821850520927,
 'support': 38681}
### report_full
macro
{'f1-score': 0.3923168548790241,
 'precision': 0.44857710520741745,
 'recall': 0.35565313935911963,
 'support': 43997}
micro
{'f1-score': 0.8805728246933887,
 'precision': 0.9410821850520927,
 'recall': 0.8273745937222993,
 'support': 43997}
weighted
{'f1-score': 0.8526049731000855,
 'precision': 0.8835063062140104,
 'recall': 0.8273745937222993,
 'support': 43997}
## test
PPCR: 0.905252
### report
macro
{'f1-score': 0.4297545444488722,
 'precision': 0.4558006269301962,
 'recall': 0.41008399894181596,
 'support': 3516}
micro
{'f1-score': 0.941410693970421,
 'precision': 0.941410693970421,
 'recall': 0.941410693970421,
 'support': 3516}
weighted
{'f1-score': 0.9367430740793642,
 'precision': 0.9343698156438724,
 'recall': 0.941410693970421,
 'support': 3516}
### report_full
macro
{'f1-score': 0.37233856009237476,
 'precision': 0.4558006269301962,
 'recall': 0.3269369475831225,
 'support': 3884}
micro
{'f1-score': 0.8945945945945948,
 'precision': 0.941410693970421,
 'recall': 0.8522142121524202,
 'support': 3884}
weighted
{'f1-score': 0.8791970035006712,
 'precision': 0.9203672466920843,
 'recall': 0.8522142121524202,
 'support': 3884}
```

## javascript
### Summary
13 rules, avg.len. 6.7

| | |
|-|-|
|Min support|104|
|Max support|7808|
|Min confidence|0.9219101071357727|
|Max confidence|0.9981949329376221|

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
                     'min_samples_leaf': 90,
                     'min_samples_split': 240,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 7554.` |
| 2 | `  -1.reserved = (<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 277.` |
| 3 | `  •••start_line ≥ 39<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.941. Support: 160.` |
| 4 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.963. Support: 122.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved not in {)}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.922. Support: 4450.` |
| 6 | `  -1.reserved = ;<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {FOR} and not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.969. Support: 112.` |
| 7 | `  •••start_line ≥ 70<br>	∧ -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.937. Support: 197.` |
| 8 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 1219.` |
| 9 | `  -1.reserved not in {;}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.970. Support: 752.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 104.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, if, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.991. Support: 171.` |
| 12 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;, if, {}<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ -2.length ≥ 5<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {ConditionalExpression, ObjectProperty}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.940. Support: 511.` |
| 13 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;, if, {}<br>	∧ -2.length ≤ 4<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.939. Support: 7808.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.6923076923076925, "max_conf": 0.9981949329376221, "max_support": 7808, "min_conf": 0.9219101071357727, "min_support": 104, "num_rules": 13}}
```
</details>
