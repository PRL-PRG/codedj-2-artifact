# Model report for file:///tmp/top-repos-quality-repos-a7lhypiv/riftsketchedit.git HEAD e3e45f52f5b044dc8d2ebd3ce5c6bc6f88b6b0cf

### Dump

```json
{'created_at': '2021-09-02 02:46:22',
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
 'size': '15.6 kB',
 'tags': [],
 'uuid': '875a603c-70ad-4632-8830-7c98ade3bd15',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-a7lhypiv/riftsketchedit.git e3e45f52f5b044dc8d2ebd3ce5c6bc6f88b6b0cf

# javascript
7 rules, avg.len. 4.6
## train
PPCR: 0.699832
### report
macro
{'f1-score': 0.5216725714174086,
 'precision': 0.5297131873778003,
 'recall': 0.5143486283089772,
 'support': 4178}
micro
{'f1-score': 0.9394447103877454,
 'precision': 0.9394447103877454,
 'recall': 0.9394447103877454,
 'support': 4178}
weighted
{'f1-score': 0.9332831923972222,
 'precision': 0.9278808249710568,
 'recall': 0.9394447103877454,
 'support': 4178}
### report_full
macro
{'f1-score': 0.4648628586484,
 'precision': 0.5297131873778003,
 'recall': 0.42192606680276706,
 'support': 5970}
micro
{'f1-score': 0.7735514387071344,
 'precision': 0.9394447103877454,
 'recall': 0.6574539363484088,
 'support': 5970}
weighted
{'f1-score': 0.7202479078740843,
 'precision': 0.8177268424085529,
 'recall': 0.6574539363484088,
 'support': 5970}
## test
PPCR: 0.670465
### report
macro
{'f1-score': 0.4927390941503914,
 'precision': 0.49438961045927854,
 'recall': 0.49650653298069714,
 'support': 706}
micro
{'f1-score': 0.896600566572238,
 'precision': 0.896600566572238,
 'recall': 0.896600566572238,
 'support': 706}
weighted
{'f1-score': 0.8882742384764851,
 'precision': 0.8835230291908613,
 'recall': 0.896600566572238,
 'support': 706}
### report_full
macro
{'f1-score': 0.415312111908995,
 'precision': 0.49438961045927854,
 'recall': 0.3663294326862543,
 'support': 1053}
micro
{'f1-score': 0.7197271176805002,
 'precision': 0.896600566572238,
 'recall': 0.6011396011396012,
 'support': 1053}
weighted
{'f1-score': 0.6746862872421505,
 'precision': 0.7980365944595451,
 'recall': 0.6011396011396012,
 'support': 1053}
```

## javascript
### Summary
4 rules, avg.len. 4.5

| | |
|-|-|
|Min support|114|
|Max support|1284|
|Min confidence|0.9204545617103577|
|Max confidence|0.9941588640213013|

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
| 1 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.994. Support: 1284.` |
| 2 | `  -1.reserved = {<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.969. Support: 114.` |
| 3 | `  +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.920. Support: 132.` |
| 4 | `  -1.roles in {LITERAL, NUMBER}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 128.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.5, "max_conf": 0.9941588640213013, "max_support": 1284, "min_conf": 0.9204545617103577, "min_support": 114, "num_rules": 4}}
```
</details>
