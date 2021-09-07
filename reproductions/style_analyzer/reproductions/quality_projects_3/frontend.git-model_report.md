# Model report for file:///tmp/top-repos-quality-repos-l3up5jui/frontend.git HEAD d5935119454a058ff243d57e85ac3f9d4c39719a

### Dump

```json
{'created_at': '2021-09-02 07:37:54',
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
 'size': '17.3 kB',
 'tags': [],
 'uuid': '087e7f31-89ec-42dd-81a5-5f0378239275',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-l3up5jui/frontend.git d5935119454a058ff243d57e85ac3f9d4c39719a

# javascript
10 rules, avg.len. 5.6
## train
PPCR: 0.653384
### report
macro
{'f1-score': 0.3054657298754406,
 'precision': 0.29447317932556255,
 'recall': 0.31817220394600065,
 'support': 6854}
micro
{'f1-score': 0.9115844762182667,
 'precision': 0.9115844762182667,
 'recall': 0.9115844762182667,
 'support': 6854}
weighted
{'f1-score': 0.8951493948986374,
 'precision': 0.8801831105219409,
 'recall': 0.9115844762182667,
 'support': 6854}
### report_full
macro
{'f1-score': 0.2583887884825511,
 'precision': 0.29447317932556255,
 'recall': 0.2308619655120239,
 'support': 10490}
micro
{'f1-score': 0.720479704797048,
 'precision': 0.9115844762182667,
 'recall': 0.5956148713060058,
 'support': 10490}
weighted
{'f1-score': 0.6811094697752437,
 'precision': 0.7984693539581711,
 'recall': 0.5956148713060058,
 'support': 10490}
## test
PPCR: 0.607198
### report
macro
{'f1-score': 0.27888085770126825,
 'precision': 0.2571716538449817,
 'recall': 0.31531789288967094,
 'support': 1569}
micro
{'f1-score': 0.8521351179094965,
 'precision': 0.8521351179094965,
 'recall': 0.8521351179094965,
 'support': 1569}
weighted
{'f1-score': 0.8230274139173386,
 'precision': 0.8037810116625651,
 'recall': 0.8521351179094965,
 'support': 1569}
### report_full
macro
{'f1-score': 0.23216718670522965,
 'precision': 0.2571716538449817,
 'recall': 0.21543124862639937,
 'support': 2584}
micro
{'f1-score': 0.6438718998314471,
 'precision': 0.8521351179094965,
 'recall': 0.5174148606811145,
 'support': 2584}
weighted
{'f1-score': 0.5903158273428807,
 'precision': 0.6965626109362819,
 'recall': 0.5174148606811145,
 'support': 2584}
```

## javascript
### Summary
3 rules, avg.len. 6.3

| | |
|-|-|
|Min support|146|
|Max support|1891|
|Min confidence|0.9289264678955078|
|Max confidence|0.9965753555297852|

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
| 1 | `  +1.reserved = ><br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 146.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, MODULE}<br>	∧ ^2.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.955. Support: 1891.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, MODULE}<br>	∧ ^2.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.929. Support: 1006.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.333333333333333, "max_conf": 0.9965753555297852, "max_support": 1891, "min_conf": 0.9289264678955078, "min_support": 146, "num_rules": 3}}
```
</details>
