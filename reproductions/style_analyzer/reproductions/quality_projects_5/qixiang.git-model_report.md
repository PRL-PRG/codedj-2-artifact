# Model report for file:///tmp/top-repos-quality-repos-mw8neqtd/qixiang.git HEAD d888c3b4f626ebe7fd86b81b2055358ee458cf56

### Dump

```json
{'created_at': '2021-09-02 09:40:43',
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
 'size': '15.5 kB',
 'tags': [],
 'uuid': '7e0ad63c-3902-4d19-9e81-3b4a310ca92e',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-mw8neqtd/qixiang.git d888c3b4f626ebe7fd86b81b2055358ee458cf56

# javascript
6 rules, avg.len. 4.0
## train
PPCR: 0.540961
### report
macro
{'f1-score': 0.342572816237587,
 'precision': 0.35697721622649237,
 'recall': 0.3364839805645646,
 'support': 1981}
micro
{'f1-score': 0.8440181726400807,
 'precision': 0.8440181726400807,
 'recall': 0.8440181726400807,
 'support': 1981}
weighted
{'f1-score': 0.8254676253469779,
 'precision': 0.8190536370858773,
 'recall': 0.8440181726400807,
 'support': 1981}
### report_full
macro
{'f1-score': 0.2673486851714291,
 'precision': 0.35697721622649237,
 'recall': 0.21693751644364245,
 'support': 3662}
micro
{'f1-score': 0.5925925925925926,
 'precision': 0.8440181726400807,
 'recall': 0.4565811032222829,
 'support': 3662}
weighted
{'f1-score': 0.5497815551733155,
 'precision': 0.6968742037553967,
 'recall': 0.4565811032222829,
 'support': 3662}
## test
PPCR: 0.570258
### report
macro
{'f1-score': 0.3544632655568926,
 'precision': 0.3514930458755382,
 'recall': 0.3608927373766408,
 'support': 487}
micro
{'f1-score': 0.8562628336755647,
 'precision': 0.8562628336755647,
 'recall': 0.8562628336755647,
 'support': 487}
weighted
{'f1-score': 0.8497589260493215,
 'precision': 0.8526999035289868,
 'recall': 0.8562628336755647,
 'support': 487}
### report_full
macro
{'f1-score': 0.2866884864585109,
 'precision': 0.3514930458755382,
 'recall': 0.24310631229235882,
 'support': 854}
micro
{'f1-score': 0.6219239373601789,
 'precision': 0.8562628336755647,
 'recall': 0.4882903981264637,
 'support': 854}
weighted
{'f1-score': 0.58028669790521,
 'precision': 0.7205823774100841,
 'recall': 0.4882903981264637,
 'support': 854}
```

## javascript
### Summary
2 rules, avg.len. 3.5

| | |
|-|-|
|Min support|100|
|Max support|290|
|Min confidence|0.932758629322052|
|Max confidence|0.9549999833106995|

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
                     'min_samples_split': 233,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.length ≥ 3<br>	∧ -2.diff_offset ≤ 12<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.933. Support: 290.` |
| 2 | `  +1.length ≤ 1<br>	∧ ^1.roles in {VARIABLE} and not in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 100.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 3.5, "max_conf": 0.9549999833106995, "max_support": 290, "min_conf": 0.932758629322052, "min_support": 100, "num_rules": 2}}
```
</details>
