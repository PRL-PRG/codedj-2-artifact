# Model report for file:///tmp/top-repos-quality-repos-3wu4z8ze/nodejs_course_project.git HEAD 2b5b25cb120679e3aee6b79b1ad3dd0877a8f498

### Dump

```json
{'created_at': '2021-09-02 16:22:20',
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
 'uuid': '6d41710f-b3b6-4381-9647-bd99e5eb0565',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-3wu4z8ze/nodejs_course_project.git 2b5b25cb120679e3aee6b79b1ad3dd0877a8f498

# javascript
8 rules, avg.len. 7.6
## train
PPCR: 0.641817
### report
macro
{'f1-score': 0.6082939856857253,
 'precision': 0.6512463701356266,
 'recall': 0.5903026428216397,
 'support': 4718}
micro
{'f1-score': 0.9116150911403137,
 'precision': 0.9116150911403137,
 'recall': 0.9116150911403137,
 'support': 4718}
weighted
{'f1-score': 0.9013198951095734,
 'precision': 0.9078519579175273,
 'recall': 0.9116150911403137,
 'support': 4718}
### report_full
macro
{'f1-score': 0.40611813648281414,
 'precision': 0.6512463701356266,
 'recall': 0.31781680882792884,
 'support': 7351}
micro
{'f1-score': 0.7127351064711244,
 'precision': 0.9116150911403137,
 'recall': 0.585090463882465,
 'support': 7351}
weighted
{'f1-score': 0.6566947014539488,
 'precision': 0.8501296399255649,
 'recall': 0.585090463882465,
 'support': 7351}
## test
PPCR: 0.631615
### report
macro
{'f1-score': 0.6140379132350084,
 'precision': 0.6480287534921682,
 'recall': 0.6019603283082503,
 'support': 931}
micro
{'f1-score': 0.9215896885069818,
 'precision': 0.9215896885069818,
 'recall': 0.9215896885069818,
 'support': 931}
weighted
{'f1-score': 0.9119153799248171,
 'precision': 0.9180754013293899,
 'recall': 0.9215896885069818,
 'support': 931}
### report_full
macro
{'f1-score': 0.43703346374227575,
 'precision': 0.6480287534921682,
 'recall': 0.3445063137422556,
 'support': 1474}
micro
{'f1-score': 0.7135135135135134,
 'precision': 0.9215896885069818,
 'recall': 0.582089552238806,
 'support': 1474}
weighted
{'f1-score': 0.6674303457733313,
 'precision': 0.8427337484090305,
 'recall': 0.582089552238806,
 'support': 1474}
```

## javascript
### Summary
3 rules, avg.len. 5.7

| | |
|-|-|
|Min support|140|
|Max support|1203|
|Min confidence|0.9721529483795166|
|Max confidence|0.9964285492897034|

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
| 1 | `  -1.reserved = :<br>	∧ +1.roles in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 214.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {VALUE}<br>⇒ y = ⏎<br>Confidence: 0.996. Support: 140.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 1203.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.666666666666667, "max_conf": 0.9964285492897034, "max_support": 1203, "min_conf": 0.9721529483795166, "min_support": 140, "num_rules": 3}}
```
</details>
