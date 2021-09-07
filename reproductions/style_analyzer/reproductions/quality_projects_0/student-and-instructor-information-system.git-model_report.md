# Model report for file:///tmp/top-repos-quality-repos-buvox7hc/student-and-instructor-information-system.git HEAD b8a3ea744f01f038e2ca13dd6d45ea638d082051

### Dump

```json
{'created_at': '2021-09-01 20:42:19',
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
 'size': '18.9 kB',
 'tags': [],
 'uuid': '70d2b18d-56e3-4df6-89d7-76356ee1232b',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-buvox7hc/student-and-instructor-information-system.git b8a3ea744f01f038e2ca13dd6d45ea638d082051

# javascript
26 rules, avg.len. 7.6
## train
PPCR: 0.853907
### report
macro
{'f1-score': 0.4582734097826117,
 'precision': 0.4530132953416408,
 'recall': 0.46500897518971496,
 'support': 25712}
micro
{'f1-score': 0.9539125700062228,
 'precision': 0.9539125700062228,
 'recall': 0.9539125700062228,
 'support': 25712}
weighted
{'f1-score': 0.9464706664367303,
 'precision': 0.939666126269379,
 'recall': 0.9539125700062228,
 'support': 25712}
### report_full
macro
{'f1-score': 0.408562936360795,
 'precision': 0.4530132953416408,
 'recall': 0.379324166182682,
 'support': 30111}
micro
{'f1-score': 0.8787417372767496,
 'precision': 0.9539125700062228,
 'recall': 0.8145528212281226,
 'support': 30111}
weighted
{'f1-score': 0.8463526114645665,
 'precision': 0.8872347412062161,
 'recall': 0.8145528212281226,
 'support': 30111}
## test
PPCR: 0.859421
### report
macro
{'f1-score': 0.4605944570969528,
 'precision': 0.45151799628602013,
 'recall': 0.4703117714047993,
 'support': 6144}
micro
{'f1-score': 0.9482421875,
 'precision': 0.9482421875,
 'recall': 0.9482421875,
 'support': 6144}
weighted
{'f1-score': 0.9406760048175524,
 'precision': 0.9335073627242435,
 'recall': 0.9482421875,
 'support': 6144}
### report_full
macro
{'f1-score': 0.4094631814664026,
 'precision': 0.45151799628602013,
 'recall': 0.3797058556148365,
 'support': 7149}
micro
{'f1-score': 0.8765515684946965,
 'precision': 0.9482421875,
 'recall': 0.814939152328997,
 'support': 7149}
weighted
{'f1-score': 0.8442077339418297,
 'precision': 0.8799724014367848,
 'recall': 0.814939152328997,
 'support': 7149}
```

## javascript
### Summary
14 rules, avg.len. 7.9

| | |
|-|-|
|Min support|97|
|Max support|4432|
|Min confidence|0.9270650148391724|
|Max confidence|0.9937950968742371|

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
| 1 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.981. Support: 243.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.939. Support: 220.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -2.label not in {<space>}<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.927. Support: 569.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.994. Support: 4432.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.994. Support: 3808.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 705.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = ObjectExpression<br>⇒ y = ⏎<br>Confidence: 0.948. Support: 163.` |
| 8 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression, ObjectExpression}<br>	∧ ^1.roles in {FILE}<br>⇒ y = ⏎<br>Confidence: 0.933. Support: 97.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.length ≥ 3<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {import}<br>	∧ +1.length ≥ 5<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression, ObjectExpression}<br>	∧ ^1.roles not in {BODY, FILE}<br>⇒ y = ␣<br>Confidence: 0.961. Support: 524.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {import}<br>	∧ +1.length ≤ 4<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression, ObjectExpression}<br>	∧ ^1.roles not in {BODY, FILE}<br>⇒ y = ␣<br>Confidence: 0.935. Support: 1169.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.944. Support: 436.` |
| 12 | `  -1.diff_offset ≤ 4<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ +3.reserved = {<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles not in {VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.960. Support: 211.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ +3.reserved not in {{}<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles not in {OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 144.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {if}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ +3.reserved not in {{}<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles not in {OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.956. Support: 4200.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.857142857142857, "max_conf": 0.9937950968742371, "max_support": 4432, "min_conf": 0.9270650148391724, "min_support": 97, "num_rules": 14}}
```
</details>
