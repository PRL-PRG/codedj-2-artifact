# Model report for file:///tmp/top-repos-quality-repos-7yig_9kf/biz-beautyshop.git HEAD 579d58e23c091102de4e3cc695238e389d366afa

### Dump

```json
{'created_at': '2021-09-01 18:02:36',
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
 'size': '19.0 kB',
 'tags': [],
 'uuid': 'a75cca84-c78a-418b-aa44-e1f98748a196',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-7yig_9kf/biz-beautyshop.git 579d58e23c091102de4e3cc695238e389d366afa

# javascript
15 rules, avg.len. 6.2
## train
PPCR: 0.811626
### report
macro
{'f1-score': 0.6305327165806609,
 'precision': 0.6426824760716844,
 'recall': 0.6221464150528349,
 'support': 8070}
micro
{'f1-score': 0.9230483271375465,
 'precision': 0.9230483271375465,
 'recall': 0.9230483271375465,
 'support': 8070}
weighted
{'f1-score': 0.9172257665550158,
 'precision': 0.9145332489411301,
 'recall': 0.9230483271375465,
 'support': 8070}
### report_full
macro
{'f1-score': 0.5467822569057674,
 'precision': 0.6426824760716844,
 'recall': 0.4997727354648377,
 'support': 9943}
micro
{'f1-score': 0.8270693388108588,
 'precision': 0.9230483271375465,
 'recall': 0.74917027054209,
 'support': 9943}
weighted
{'f1-score': 0.799420851222059,
 'precision': 0.8937755586744269,
 'recall': 0.74917027054209,
 'support': 9943}
## test
PPCR: 0.809574
### report
macro
{'f1-score': 0.4543758351963699,
 'precision': 0.47204934957678873,
 'recall': 0.461871738585809,
 'support': 1522}
micro
{'f1-score': 0.8482260183968463,
 'precision': 0.8482260183968463,
 'recall': 0.8482260183968463,
 'support': 1522}
weighted
{'f1-score': 0.8638871296820669,
 'precision': 0.8908702347708207,
 'recall': 0.8482260183968463,
 'support': 1522}
### report_full
macro
{'f1-score': 0.36522859119441387,
 'precision': 0.47204934957678873,
 'recall': 0.33865651891981274,
 'support': 1880}
micro
{'f1-score': 0.7589653145208701,
 'precision': 0.8482260183968463,
 'recall': 0.6867021276595745,
 'support': 1880}
weighted
{'f1-score': 0.7474045226441889,
 'precision': 0.8761696013649748,
 'recall': 0.6867021276595745,
 'support': 1880}
```

## javascript
### Summary
7 rules, avg.len. 6.1

| | |
|-|-|
|Min support|103|
|Max support|2182|
|Min confidence|0.9372623562812805|
|Max confidence|0.9956521987915039|

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
                     'min_samples_leaf': 102,
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
| 1 | `  -1.reserved = :<br>	∧ ^2.roles in {MAP}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 403.` |
| 2 | `  -1.reserved = ,<br>	∧ ^2.roles in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.937. Support: 263.` |
| 3 | `  -1.internal_type = StringLiteral<br>	∧ ^2.roles not in {MAP}<br>⇒ y = '<br>Confidence: 0.973. Support: 310.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 152.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = function<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 103.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, function, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ ^1.roles in {EXPRESSION} and not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ∅<br>Confidence: 0.952. Support: 2182.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, function, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {EXPRESSION, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 115.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.142857142857143, "max_conf": 0.9956521987915039, "max_support": 2182, "min_conf": 0.9372623562812805, "min_support": 103, "num_rules": 7}}
```
</details>
