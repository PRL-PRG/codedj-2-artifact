# Model report for file:///tmp/top-repos-quality-repos-yh3x8nlw/esox-webstore.git HEAD a939a58e6ef0b90efba760b9dfb27f1dff0dfd8b

### Dump

```json
{'created_at': '2021-09-02 00:06:08',
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
 'size': '15.3 kB',
 'tags': [],
 'uuid': '06caabda-e6b3-4ad3-8018-071413aa98e0',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-yh3x8nlw/esox-webstore.git a939a58e6ef0b90efba760b9dfb27f1dff0dfd8b

# javascript
4 rules, avg.len. 2.5
## train
PPCR: 0.479924
### report
macro
{'f1-score': 0.2616414682916678,
 'precision': 0.2778892455858748,
 'recall': 0.25054945054945055,
 'support': 2283}
micro
{'f1-score': 0.9487516425755584,
 'precision': 0.9487516425755584,
 'recall': 0.9487516425755584,
 'support': 2283}
weighted
{'f1-score': 0.9328860584170571,
 'precision': 0.9213353954731355,
 'recall': 0.9487516425755584,
 'support': 2283}
### report_full
macro
{'f1-score': 0.151487824098887,
 'precision': 0.2778892455858748,
 'recall': 0.12567689791968664,
 'support': 4757}
micro
{'f1-score': 0.615340909090909,
 'precision': 0.9487516425755584,
 'recall': 0.4553289888585243,
 'support': 4757}
weighted
{'f1-score': 0.5266574261955586,
 'precision': 0.7802596646455962,
 'recall': 0.4553289888585243,
 'support': 4757}
## test
PPCR: 0.063281
### report
macro
{'f1-score': 0.27496795819777187,
 'precision': 0.27692307692307694,
 'recall': 0.2738095238095238,
 'support': 76}
micro
{'f1-score': 0.9473684210526315,
 'precision': 0.9473684210526315,
 'recall': 0.9473684210526315,
 'support': 76}
weighted
{'f1-score': 0.9281809596454905,
 'precision': 0.9111336032388664,
 'recall': 0.9473684210526315,
 'support': 76}
### report_full
macro
{'f1-score': 0.10755186258812367,
 'precision': 0.27692307692307694,
 'recall': 0.0789835164835165,
 'support': 1201}
micro
{'f1-score': 0.11276429130775256,
 'precision': 0.9473684210526315,
 'recall': 0.05995004163197336,
 'support': 1201}
weighted
{'f1-score': 0.083670741096295,
 'precision': 0.26851982322423623,
 'recall': 0.05995004163197336,
 'support': 1201}
```

## javascript
### Summary
3 rules, avg.len. 2.7

| | |
|-|-|
|Min support|113|
|Max support|1011|
|Min confidence|0.9589515328407288|
|Max confidence|0.9955752491950989|

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
                     'min_samples_leaf': 95,
                     'min_samples_split': 203,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type = Identifier<br>	∧ +1.reserved = =<br>⇒ y = ␣<br>Confidence: 0.996. Support: 113.` |
| 2 | `  -1.internal_type = Identifier<br>	∧ +1.reserved not in {=}<br>⇒ y = ∅<br>Confidence: 0.959. Support: 1011.` |
| 3 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = (<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 275.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 2.6666666666666665, "max_conf": 0.9955752491950989, "max_support": 1011, "min_conf": 0.9589515328407288, "min_support": 113, "num_rules": 3}}
```
</details>
