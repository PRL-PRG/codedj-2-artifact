# Model report for file:///tmp/top-repos-quality-repos-oytmtc04/necsus.git HEAD d662dfec5b90476bb56c224d69430339344529ba

### Dump

```json
{'created_at': '2021-09-01 02:36:30',
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
 'size': '14.7 kB',
 'tags': [],
 'uuid': 'ad3037e5-b6d5-4528-947a-10b87dd9699a',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-oytmtc04/necsus.git d662dfec5b90476bb56c224d69430339344529ba

# javascript
17 rules, avg.len. 3.0
## train
PPCR: 0.661096
### report
macro
{'f1-score': 0.284578050925628,
 'precision': 0.28604729204351065,
 'recall': 0.28508201307177056,
 'support': 1978}
micro
{'f1-score': 0.8736097067745198,
 'precision': 0.8736097067745198,
 'recall': 0.8736097067745198,
 'support': 1978}
weighted
{'f1-score': 0.8471097950419412,
 'precision': 0.8257844184405403,
 'recall': 0.8736097067745198,
 'support': 1978}
### report_full
macro
{'f1-score': 0.23786081842685616,
 'precision': 0.28604729204351065,
 'recall': 0.21056321054111596,
 'support': 2992}
micro
{'f1-score': 0.6953722334004024,
 'precision': 0.8736097067745198,
 'recall': 0.5775401069518716,
 'support': 2992}
weighted
{'f1-score': 0.626477705147239,
 'precision': 0.7066136629643878,
 'recall': 0.5775401069518716,
 'support': 2992}
## test
PPCR: 0.764835
### report
macro
{'f1-score': 0.2914671985815603,
 'precision': 0.2849509163886068,
 'recall': 0.29842010151744663,
 'support': 348}
micro
{'f1-score': 0.867816091954023,
 'precision': 0.867816091954023,
 'recall': 0.867816091954023,
 'support': 348}
weighted
{'f1-score': 0.8517126742479824,
 'precision': 0.8365051751652101,
 'recall': 0.867816091954023,
 'support': 348}
### report_full
macro
{'f1-score': 0.27070527097253155,
 'precision': 0.2849509163886068,
 'recall': 0.25956212081245056,
 'support': 455}
micro
{'f1-score': 0.7521793275217933,
 'precision': 0.867816091954023,
 'recall': 0.6637362637362637,
 'support': 455}
weighted
{'f1-score': 0.6852827528818621,
 'precision': 0.7131012031635853,
 'recall': 0.6637362637362637,
 'support': 455}
```

## javascript
### Summary
10 rules, avg.len. 2.7

| | |
|-|-|
|Min support|145|
|Max support|592|
|Min confidence|0.9275861978530884|
|Max confidence|0.9971264600753784|

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
                     'min_samples_split': 182,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.982. Support: 592.` |
| 2 | `  +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 155.` |
| 3 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 590.` |
| 4 | `  +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 187.` |
| 5 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 561.` |
| 6 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 246.` |
| 7 | `  +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 174.` |
| 8 | `  +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 160.` |
| 9 | `  +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 183.` |
| 10 | `  +1.reserved = ;<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.928. Support: 145.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 2.7, "max_conf": 0.9971264600753784, "max_support": 592, "min_conf": 0.9275861978530884, "min_support": 145, "num_rules": 10}}
```
</details>
