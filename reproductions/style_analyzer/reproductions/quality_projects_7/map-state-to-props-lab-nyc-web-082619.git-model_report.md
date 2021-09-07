# Model report for file:///tmp/top-repos-quality-repos-i8r3erp2/map-state-to-props-lab-nyc-web-082619.git HEAD ead61ff840ae7ec0dc18405f0d2263f9edc4b2b2

### Dump

```json
{'created_at': '2021-09-01 02:32:21',
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
 'size': '12.8 kB',
 'tags': [],
 'uuid': '36e34152-429b-447a-984a-8e86e25a47cc',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-i8r3erp2/map-state-to-props-lab-nyc-web-082619.git ead61ff840ae7ec0dc18405f0d2263f9edc4b2b2

# javascript
4 rules, avg.len. 2.0
## train
PPCR: 0.449036
### report
macro
{'f1-score': 0.21969045571797077,
 'precision': 0.1959355828220859,
 'recall': 0.25,
 'support': 652}
micro
{'f1-score': 0.7837423312883436,
 'precision': 0.7837423312883436,
 'recall': 0.7837423312883436,
 'support': 652}
weighted
{'f1-score': 0.6887228397048041,
 'precision': 0.6142520418532876,
 'recall': 0.7837423312883436,
 'support': 652}
### report_full
macro
{'f1-score': 0.17730742539902844,
 'precision': 0.1959355828220859,
 'recall': 0.16191381495564006,
 'support': 1452}
micro
{'f1-score': 0.48574144486692006,
 'precision': 0.7837423312883436,
 'recall': 0.35192837465564736,
 'support': 1452}
weighted
{'f1-score': 0.3853872138838387,
 'precision': 0.4258765147289966,
 'recall': 0.35192837465564736,
 'support': 1452}
## test
PPCR: 0.219653
### report
macro
{'f1-score': 0.2323943661971831,
 'precision': 0.21710526315789475,
 'recall': 0.25,
 'support': 38}
micro
{'f1-score': 0.868421052631579,
 'precision': 0.868421052631579,
 'recall': 0.868421052631579,
 'support': 38}
weighted
{'f1-score': 0.8072646404744255,
 'precision': 0.7541551246537397,
 'recall': 0.868421052631579,
 'support': 38}
### report_full
macro
{'f1-score': 0.10714285714285715,
 'precision': 0.21710526315789475,
 'recall': 0.07112068965517242,
 'support': 173}
micro
{'f1-score': 0.3127962085308057,
 'precision': 0.868421052631579,
 'recall': 0.1907514450867052,
 'support': 173}
weighted
{'f1-score': 0.2873658133773741,
 'precision': 0.5822938850015211,
 'recall': 0.1907514450867052,
 'support': 173}
```

## javascript
### Summary
2 rules, avg.len. 2.0

| | |
|-|-|
|Min support|174|
|Max support|187|
|Min confidence|0.9741379022598267|
|Max confidence|0.9759358167648315|

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
                     'max_depth': 5,
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
| 1 | `  +1.roles not in {LITERAL}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 187.` |
| 2 | `  +1.roles not in {STRING}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 174.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 2.0, "max_conf": 0.9759358167648315, "max_support": 187, "min_conf": 0.9741379022598267, "min_support": 174, "num_rules": 2}}
```
</details>
