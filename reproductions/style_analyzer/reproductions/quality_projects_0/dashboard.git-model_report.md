# Model report for file:///tmp/top-repos-quality-repos-mwgl8e7c/dashboard.git HEAD 265e8beaa7cf94381ac208074d1405a3b046b475

### Dump

```json
{'created_at': '2021-09-01 19:36:27',
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
 'size': '16.0 kB',
 'tags': [],
 'uuid': '2a721185-7e25-4639-9554-e9181623696d',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-mwgl8e7c/dashboard.git 265e8beaa7cf94381ac208074d1405a3b046b475

# javascript
27 rules, avg.len. 5.7
## train
PPCR: 0.816933
### report
macro
{'f1-score': 0.4899933419057162,
 'precision': 0.5087381088272523,
 'recall': 0.4833841553820391,
 'support': 4342}
micro
{'f1-score': 0.8281897742975587,
 'precision': 0.8281897742975587,
 'recall': 0.8281897742975587,
 'support': 4342}
weighted
{'f1-score': 0.79992786497848,
 'precision': 0.7996078528760093,
 'recall': 0.8281897742975587,
 'support': 4342}
### report_full
macro
{'f1-score': 0.41822421870510756,
 'precision': 0.5087381088272523,
 'recall': 0.37550825771892715,
 'support': 5315}
micro
{'f1-score': 0.7447447447447448,
 'precision': 0.8281897742975587,
 'recall': 0.6765757290686736,
 'support': 5315}
weighted
{'f1-score': 0.6899031474278007,
 'precision': 0.7624197079178681,
 'recall': 0.6765757290686736,
 'support': 5315}
## test
PPCR: 0.804438
### report
macro
{'f1-score': 0.39625257832594124,
 'precision': 0.4560748476063412,
 'recall': 0.37347074238036876,
 'support': 580}
micro
{'f1-score': 0.7086206896551724,
 'precision': 0.7086206896551724,
 'recall': 0.7086206896551724,
 'support': 580}
weighted
{'f1-score': 0.6666318485398963,
 'precision': 0.6869743342639473,
 'recall': 0.7086206896551724,
 'support': 580}
### report_full
macro
{'f1-score': 0.3364779938854013,
 'precision': 0.4560748476063412,
 'recall': 0.2962363024416609,
 'support': 721}
micro
{'f1-score': 0.6318216756341275,
 'precision': 0.7086206896551724,
 'recall': 0.5700416088765603,
 'support': 721}
weighted
{'f1-score': 0.5768792217012282,
 'precision': 0.6845920546845606,
 'recall': 0.5700416088765603,
 'support': 721}
```

## javascript
### Summary
10 rules, avg.len. 3.4

| | |
|-|-|
|Min support|136|
|Max support|542|
|Min confidence|0.9301470518112183|
|Max confidence|0.9983277320861816|

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
| 1 | `  -1.reserved = :<br>⇒ y = ␣<br>Confidence: 0.998. Support: 299.` |
| 2 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {:}<br>⇒ y = "<br>Confidence: 0.997. Support: 193.` |
| 3 | `  -1.internal_type = StringLiteral<br>	∧ +1.roles not in {EXPRESSION}<br>⇒ y = "<br>Confidence: 0.997. Support: 198.` |
| 4 | `  -1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.998. Support: 220.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.diff_line = 0<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 3<br>⇒ y = ∅<br>Confidence: 0.997. Support: 157.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +2.reserved = :<br>⇒ y = ⏎<br>Confidence: 0.935. Support: 161.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.997. Support: 186.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved = :<br>⇒ y = ⏎<br>Confidence: 0.930. Support: 136.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.970. Support: 542.` |
| 10 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {:}<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = "<br>Confidence: 0.997. Support: 192.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 3.4, "max_conf": 0.9983277320861816, "max_support": 542, "min_conf": 0.9301470518112183, "min_support": 136, "num_rules": 10}}
```
</details>
