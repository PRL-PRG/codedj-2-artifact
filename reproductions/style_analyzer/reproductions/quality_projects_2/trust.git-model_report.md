# Model report for file:///tmp/top-repos-quality-repos-qpbpda06/trust.git HEAD 38a9e1976cc5284eef67673e9d9cdf0ca1711c80

### Dump

```json
{'created_at': '2021-09-02 05:24:35',
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
 'size': '16.9 kB',
 'tags': [],
 'uuid': 'c46f393d-e2b2-4ba3-9728-c33681ec98e6',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-qpbpda06/trust.git 38a9e1976cc5284eef67673e9d9cdf0ca1711c80

# javascript
11 rules, avg.len. 4.3
## train
PPCR: 0.877771
### report
macro
{'f1-score': 0.6502345969396588,
 'precision': 0.6722488522813301,
 'recall': 0.6340801459353222,
 'support': 9106}
micro
{'f1-score': 0.9155501866900945,
 'precision': 0.9155501866900945,
 'recall': 0.9155501866900945,
 'support': 9106}
weighted
{'f1-score': 0.9027146665649325,
 'precision': 0.894590888372045,
 'recall': 0.9155501866900945,
 'support': 9106}
### report_full
macro
{'f1-score': 0.6244331329902637,
 'precision': 0.6722488522813301,
 'recall': 0.5956195500105854,
 'support': 10374}
micro
{'f1-score': 0.8559548254620123,
 'precision': 0.9155501866900945,
 'recall': 0.8036437246963563,
 'support': 10374}
weighted
{'f1-score': 0.8207059521598811,
 'precision': 0.858228667954003,
 'recall': 0.8036437246963563,
 'support': 10374}
## test
PPCR: 0.851024
### report
macro
{'f1-score': 0.6600623688042468,
 'precision': 0.7237670912625873,
 'recall': 0.6271159626641354,
 'support': 457}
micro
{'f1-score': 0.9518599562363238,
 'precision': 0.9518599562363238,
 'recall': 0.9518599562363238,
 'support': 457}
weighted
{'f1-score': 0.9420044766917449,
 'precision': 0.9385465008885658,
 'recall': 0.9518599562363238,
 'support': 457}
### report_full
macro
{'f1-score': 0.6273615633375418,
 'precision': 0.7237670912625873,
 'recall': 0.572240323522148,
 'support': 537}
micro
{'f1-score': 0.8752515090543259,
 'precision': 0.9518599562363238,
 'recall': 0.8100558659217877,
 'support': 537}
weighted
{'f1-score': 0.8376465152795509,
 'precision': 0.8808682277505382,
 'recall': 0.8100558659217877,
 'support': 537}
```

## javascript
### Summary
4 rules, avg.len. 5.5

| | |
|-|-|
|Min support|120|
|Max support|4987|
|Min confidence|0.9208333492279053|
|Max confidence|0.9974489808082581|

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
| 1 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.921. Support: 120.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = var<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 196.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, var}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.940. Support: 260.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, var, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.922. Support: 4987.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.5, "max_conf": 0.9974489808082581, "max_support": 4987, "min_conf": 0.9208333492279053, "min_support": 120, "num_rules": 4}}
```
</details>
