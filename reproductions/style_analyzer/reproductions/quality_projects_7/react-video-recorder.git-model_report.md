# Model report for file:///tmp/top-repos-quality-repos-lnub4lqn/react-video-recorder.git HEAD dc441024546cfe9862e5e4a8bc0ea3a72840833e

### Dump

```json
{'created_at': '2021-09-01 01:47:04',
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
 'size': '15.4 kB',
 'tags': [],
 'uuid': 'c38a44fd-6a46-44f7-b384-d01fb3826b35',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-lnub4lqn/react-video-recorder.git dc441024546cfe9862e5e4a8bc0ea3a72840833e

# javascript
4 rules, avg.len. 3.0
## train
PPCR: 0.277948
### report
macro
{'f1-score': 0.41208288544533084,
 'precision': 0.40627452314739887,
 'recall': 0.4184385595948324,
 'support': 1360}
micro
{'f1-score': 0.961764705882353,
 'precision': 0.961764705882353,
 'recall': 0.961764705882353,
 'support': 1360}
weighted
{'f1-score': 0.9522063673951677,
 'precision': 0.943245825614853,
 'recall': 0.961764705882353,
 'support': 1360}
### report_full
macro
{'f1-score': 0.18777958812886966,
 'precision': 0.40627452314739887,
 'recall': 0.12546904951489868,
 'support': 4893}
micro
{'f1-score': 0.4183591875899568,
 'precision': 0.961764705882353,
 'recall': 0.2673206621704476,
 'support': 4893}
weighted
{'f1-score': 0.3895980230140507,
 'precision': 0.7838501033210369,
 'recall': 0.2673206621704476,
 'support': 4893}
## test
PPCR: 0.248079
### report
macro
{'f1-score': 0.38871620162261566,
 'precision': 0.37588685287315426,
 'recall': 0.40631446126837834,
 'support': 226}
micro
{'f1-score': 0.9070796460176991,
 'precision': 0.9070796460176991,
 'recall': 0.9070796460176991,
 'support': 226}
weighted
{'f1-score': 0.8993438660527744,
 'precision': 0.898587928013313,
 'recall': 0.9070796460176991,
 'support': 226}
### report_full
macro
{'f1-score': 0.16764666616121496,
 'precision': 0.37588685287315426,
 'recall': 0.10854271018034438,
 'support': 911}
micro
{'f1-score': 0.3605980650835532,
 'precision': 0.9070796460176991,
 'recall': 0.22502744237102085,
 'support': 911}
weighted
{'f1-score': 0.3436554830144787,
 'precision': 0.743698944545222,
 'recall': 0.22502744237102085,
 'support': 911}
```

## javascript
### Summary
2 rules, avg.len. 2.0

| | |
|-|-|
|Min support|93|
|Max support|735|
|Min confidence|0.9623655676841736|
|Max confidence|0.980272114276886|

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
| 1 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.980. Support: 735.` |
| 2 | `  -1.reserved not in {{}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = ObjectExpression<br>⇒ y = ⏎<br>Confidence: 0.962. Support: 93.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 2.0, "max_conf": 0.980272114276886, "max_support": 735, "min_conf": 0.9623655676841736, "min_support": 93, "num_rules": 2}}
```
</details>
