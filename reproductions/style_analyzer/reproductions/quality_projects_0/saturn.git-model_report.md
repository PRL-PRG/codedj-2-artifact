# Model report for file:///tmp/top-repos-quality-repos-j608t2um/saturn.git HEAD 610b59f2b2758406cd58b8b7b5bd7106b86e720d

### Dump

```json
{'created_at': '2021-09-01 19:41:09',
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
 'size': '16.4 kB',
 'tags': [],
 'uuid': '4169aec7-c037-4b54-9ad6-569a6bb90602',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-j608t2um/saturn.git 610b59f2b2758406cd58b8b7b5bd7106b86e720d

# javascript
17 rules, avg.len. 5.5
## train
PPCR: 0.929888
### report
macro
{'f1-score': 0.9250754899068375,
 'precision': 0.9433424405585489,
 'recall': 0.9115089658465029,
 'support': 11048}
micro
{'f1-score': 0.9463251267197683,
 'precision': 0.9463251267197683,
 'recall': 0.9463251267197683,
 'support': 11048}
weighted
{'f1-score': 0.946037610308856,
 'precision': 0.9476350229475734,
 'recall': 0.9463251267197683,
 'support': 11048}
### report_full
macro
{'f1-score': 0.8567388762048294,
 'precision': 0.9433424405585489,
 'recall': 0.804011161864274,
 'support': 11881}
micro
{'f1-score': 0.9119455711108204,
 'precision': 0.9463251267197683,
 'recall': 0.8799764329601886,
 'support': 11881}
weighted
{'f1-score': 0.9080285391337986,
 'precision': 0.9485810750416294,
 'recall': 0.8799764329601886,
 'support': 11881}
## test
PPCR: 0.924462
### report
macro
{'f1-score': 0.7989618853423334,
 'precision': 0.894569486428893,
 'recall': 0.7549480187876245,
 'support': 2619}
micro
{'f1-score': 0.8377243222604047,
 'precision': 0.8377243222604047,
 'recall': 0.8377243222604047,
 'support': 2619}
weighted
{'f1-score': 0.8356014797518743,
 'precision': 0.8672467483739101,
 'recall': 0.8377243222604047,
 'support': 2619}
### report_full
macro
{'f1-score': 0.7403021419421909,
 'precision': 0.894569486428893,
 'recall': 0.677253306047895,
 'support': 2833}
micro
{'f1-score': 0.8048422597212032,
 'precision': 0.8377243222604047,
 'recall': 0.7744440522414402,
 'support': 2833}
weighted
{'f1-score': 0.7960625385138507,
 'precision': 0.8699461186049837,
 'recall': 0.7744440522414402,
 'support': 2833}
```

## javascript
### Summary
10 rules, avg.len. 4.6

| | |
|-|-|
|Min support|117|
|Max support|1908|
|Min confidence|0.9324817657470703|
|Max confidence|0.9994523525238037|

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
| 1 | `  -1.roles in {STRING}<br>⇒ y = '<br>Confidence: 0.999. Support: 913.` |
| 2 | `  -1.reserved = from<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles in {STRING}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 333.` |
| 3 | `  -1.reserved not in {from}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.label not in {<newline>}<br>	∧ +1.roles in {STRING}<br>⇒ y = '<br>Confidence: 0.985. Support: 919.` |
| 4 | `  -1.roles not in {STRING}<br>	∧ +1.reserved = export<br>	∧ +1.roles not in {STRING}<br>	∧ +3.reserved = default<br>	∧ +3.roles in {EXPRESSION}<br>⇒ y = ⏎<br>Confidence: 0.978. Support: 158.` |
| 5 | `  -1.roles not in {STRING}<br>	∧ +1.reserved = export<br>	∧ +1.roles not in {STRING}<br>	∧ +3.reserved not in {default}<br>	∧ +3.roles in {EXPRESSION}<br>⇒ y = ⏎⏎<br>Confidence: 0.983. Support: 146.` |
| 6 | `  -1.roles not in {STRING}<br>	∧ +1.reserved not in {export}<br>	∧ +1.roles not in {STRING}<br>	∧ +3.roles in {EXPRESSION}<br>	∧ ^1.roles not in {MAP}<br>⇒ y = ␣<br>Confidence: 0.936. Support: 1472.` |
| 7 | `  •••start_line ≥ 11<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length = 0<br>	∧ +3.roles not in {EXPRESSION}<br>⇒ y = ⏎<br>Confidence: 0.996. Support: 125.` |
| 8 | `  •••start_line ≤ 10<br>	∧ -1.roles not in {STRING}<br>	∧ -5.roles in {INCOMPLETE}<br>	∧ +1.roles not in {STRING}<br>	∧ +3.roles not in {EXPRESSION}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.962. Support: 117.` |
| 9 | `  •••start_line ≤ 10<br>	∧ -1.roles not in {STRING}<br>	∧ -5.roles not in {INCOMPLETE}<br>	∧ +1.roles not in {STRING}<br>	∧ +3.roles not in {EXPRESSION}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 1908.` |
| 10 | `  •••start_line ≤ 10<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles in {IMPORT}<br>	∧ +3.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.932. Support: 274.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.6, "max_conf": 0.9994523525238037, "max_support": 1908, "min_conf": 0.9324817657470703, "min_support": 117, "num_rules": 10}}
```
</details>
