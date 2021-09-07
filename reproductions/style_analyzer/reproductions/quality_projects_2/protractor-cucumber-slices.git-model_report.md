# Model report for file:///tmp/top-repos-quality-repos-8jjadn08/protractor-cucumber-slices.git HEAD bac7c77acc117b766e8d4aea80d5c57ed18e07a4

### Dump

```json
{'created_at': '2021-09-02 04:50:05',
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
 'size': '15.2 kB',
 'tags': [],
 'uuid': 'ee37e683-5103-4ee2-8dca-659c1e9ae591',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-8jjadn08/protractor-cucumber-slices.git bac7c77acc117b766e8d4aea80d5c57ed18e07a4

# javascript
4 rules, avg.len. 5.5
## train
PPCR: 0.494734
### report
macro
{'f1-score': 0.2791359019156029,
 'precision': 0.27418938677216825,
 'recall': 0.2845714285714286,
 'support': 1785}
micro
{'f1-score': 0.9249299719887956,
 'precision': 0.9249299719887956,
 'recall': 0.9249299719887956,
 'support': 1785}
weighted
{'f1-score': 0.8892516401480296,
 'precision': 0.8564763894721042,
 'recall': 0.9249299719887956,
 'support': 1785}
### report_full
macro
{'f1-score': 0.2092255667103204,
 'precision': 0.27418938677216825,
 'recall': 0.17462686567164182,
 'support': 3608}
micro
{'f1-score': 0.6122751715186353,
 'precision': 0.9249299719887956,
 'recall': 0.4575942350332594,
 'support': 3608}
weighted
{'f1-score': 0.5104551249892312,
 'precision': 0.5864314500788268,
 'recall': 0.4575942350332594,
 'support': 3608}
## test
PPCR: 0.452532
### report
macro
{'f1-score': 0.2765792031098154,
 'precision': 0.26854219948849106,
 'recall': 0.2857142857142857,
 'support': 429}
micro
{'f1-score': 0.8904428904428905,
 'precision': 0.8904428904428905,
 'recall': 0.8904428904428905,
 'support': 429}
weighted
{'f1-score': 0.8391671820243248,
 'precision': 0.7940550498095255,
 'recall': 0.8904428904428905,
 'support': 429}
### report_full
macro
{'f1-score': 0.200453514739229,
 'precision': 0.26854219948849106,
 'recall': 0.1664705230803312,
 'support': 948}
micro
{'f1-score': 0.5548293391430646,
 'precision': 0.8904428904428905,
 'recall': 0.4029535864978903,
 'support': 948}
weighted
{'f1-score': 0.45177148215122903,
 'precision': 0.5280844313509664,
 'recall': 0.4029535864978903,
 'support': 948}
```

## javascript
### Summary
3 rules, avg.len. 6.0

| | |
|-|-|
|Min support|99|
|Max support|379|
|Min confidence|0.9327176809310913|
|Max confidence|0.9950494766235352|

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
                     'max_depth': 10,
                     'min_samples_leaf': 99,
                     'min_samples_split': 181,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.995. Support: 101.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -4.diff_offset ≥ 29<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≤ 2<br>	∧ +2.length ≥ 3<br>	∧ ^1.roles not in {CALL, DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.955. Support: 99.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≤ 2<br>	∧ +2.length ≤ 2<br>	∧ ^1.roles not in {CALL, DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.933. Support: 379.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.0, "max_conf": 0.9950494766235352, "max_support": 379, "min_conf": 0.9327176809310913, "min_support": 99, "num_rules": 3}}
```
</details>
